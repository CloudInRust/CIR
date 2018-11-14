use actix::prelude::*;
use actix_web::Error;
use juniper::http::GraphQLRequest;
use juniper::Context as JuniperContext;


use web::graphql::Schema;
use db::MysqlPool;

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

pub struct GLContext {
    pub db_conn: MysqlPool
}

impl JuniperContext for GLContext {}

pub struct GraphQLExecutor {
    pub schema: std::sync::Arc<Schema>,
    pub context: GLContext
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &self.context);
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}
