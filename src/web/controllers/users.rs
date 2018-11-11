use actix_web::{HttpRequest, State, HttpResponse, Error, Path};
use futures::future::{Future};

use web::AppState;
use db::models::user::{ListAllUsers, GetSingleUser};

pub fn user_list(
    (_, state): (HttpRequest<AppState>, State<AppState>)
) -> Box<Future<Item=HttpResponse, Error=Error>> {
    debug!("Listing all users");
    Box::new(
        state.db.send(ListAllUsers{})
            .from_err()
            .and_then(|maybe_users| {
                if let Ok(users) = maybe_users {
                    Ok(HttpResponse::Ok().json(users))
                } else {
                    Ok(HttpResponse::InternalServerError().into())
                }
            })
    )
}

pub fn user_view(
    (_, state, id): (HttpRequest<AppState>, State<AppState>, Path<(i32,)>)
) -> Box<Future<Item=HttpResponse, Error=Error>> {
    debug!("Displaying a specific user");
    Box::new(
        state.db.send(GetSingleUser{
            id: Some(id.0),
            username: None
        })
            .from_err()
            .and_then(|maybe_user| {
                if let Ok(user) = maybe_user {
                    Ok(HttpResponse::Ok().json(user))
                } else {
                    Ok(HttpResponse::InternalServerError().into())
                }
            })
    )
}