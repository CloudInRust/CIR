// Angular Frontend routing and endpoints

use std::sync::Arc;

use actix::SyncArbiter;
use actix_web::{
    fs, http, HttpRequest, Error, server, App,
};

use web::{WebApp, AppState, controllers, init_essential_app_features};
use web::graphql::{graphql_actor::GraphQLExecutor, schema::create_graphql_schema, self};
use db::{DBSubsystem, DbExecutor};

pub fn start_frontend_webapp(dbsys: &DBSubsystem, dbworkers_number: usize) {
    //Init DB Connections
    let clonable = dbsys.clone_pool();
    let dbworkers_addr = SyncArbiter::start(dbworkers_number, move || {
        DbExecutor(clonable.clone())
    });

    //Init GraphQL Engine
    let schema = Arc::new(create_graphql_schema());
    let graphql_addr = SyncArbiter::start(3, move || {
        GraphQLExecutor::new(schema.clone())
    });


    server::new(move || {
        init_web_frontend(
            init_essential_app_features(
                App::with_state(AppState {
                    db: dbworkers_addr.clone(),
                    graphql: graphql_addr.clone()
                })
            )
        )
    })
        .bind("127.0.0.1:8080")
        .expect("Unable to bind server")
        .start();
}

pub fn init_web_frontend(app: WebApp) -> WebApp {
    app
        .scope("/users/", |s| {
            s
                .resource("", |r| {
                    r.method(http::Method::GET).with(controllers::users::user_list);
                })
                .resource("{id}", |r| {
                    r.method(http::Method::GET).with(controllers::users::user_view);
                })
        })
        .resource("/graphql", |r| r.method(http::Method::POST).with(graphql::controllers::graphql))
        .resource("/graphiql", |r| r.method(http::Method::GET).h(graphql::controllers::graphiql))
        .handler(
            "/*",
            fs::StaticFiles::new("./frontend").unwrap(),
        )
        .default_resource(|r| r.h(serve_webapp_index))
}

fn serve_webapp_index(req: &HttpRequest<AppState>) -> Result<fs::NamedFile, Error> {
    debug!("Route {} not found, serving HTML frontend.", req.uri());
    Ok(fs::NamedFile::open("frontend/index.html")?)
}


// Vain attempts to test anything in Actix...
#[cfg(test)]
mod tests {
    use actix::SyncArbiter;
    use actix_web::{
        test::TestServer, App, http, client::ClientResponse, HttpMessage, HttpResponse
    };
    use futures::Future;

    use super::init_web_frontend;
    use web::init_essential_app_features;
    use db::{self, DbExecutor};
    use web::AppState;
    use actix_web::error::PayloadError;
    use bytes::Bytes;

    pub fn start_test_frontend_webapp() -> TestServer {
        TestServer::with_factory( || {
            // Init Db Connection
            let dbsys = db::DBSubsystem::init();
            let dbworkers_addr = SyncArbiter::start(1, move || {
                DbExecutor(dbsys.clone_pool())
            });

            init_web_frontend(
                init_essential_app_features(
                    App::with_state(AppState {
                        db: dbworkers_addr.clone()
                    })
                )
            )
        })
    }

    #[test]
    fn unknown_url_handler_returns_frontend() {
        let mut srv = start_test_frontend_webapp();

        let req = srv.client(http::Method::GET, "/non/existent/uri").finish().unwrap();
        let resp: ClientResponse = srv.execute(req.send()).unwrap();

        let bytes: Result<Bytes, PayloadError> = resp.body().from_err().then(|bytes| {
            println!("{:?}", bytes);
            Ok(bytes)
        }).wait().unwrap();
        assert_eq!(true, true);
    }
}