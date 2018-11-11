#![allow(proc_macro_derive_resolution_fallback)]

use chrono::NaiveDateTime;
use diesel::prelude::*;

use db::schema::users;
use error::WebError;

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub display_name: String,
    pub created_on: NaiveDateTime,
    pub modified_on: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub display_name: &'a str,
}

type AllColumns = (
    users::id,
    users::email,
    users::display_name,
    users::created_on,
    users::modified_on
);

pub const ALL_COLUMNS: AllColumns = (
    users::id,
    users::email,
    users::display_name,
    users::created_on,
    users::modified_on
);

// DB ACTOR ACTIONS---------------------------------------
use actix::prelude::*;
use actix_web::*;

use db::db_actor::DbExecutor;
use db::DBConn;


// ---- List All Users

pub struct ListAllUsers {}

impl Message for ListAllUsers {
    type Result = Result<Vec<User>, Error>;
}

impl Handler<ListAllUsers> for DbExecutor {
    type Result = Result<Vec<User>, Error>;

    fn handle(&mut self, _: ListAllUsers, _: &mut Self::Context) -> <Self as Handler<ListAllUsers>>::Result {
        trace!("ListAllUsers received by DBexecutor");

        use db::schema::users::dsl::*;

        let conn: &DBConn = &self.0.get().unwrap();
        users
            .select(ALL_COLUMNS)
            .load::<User>(conn)
            .map_err(|e| {
                error!("Error selecting all users: {:?}", e);
                WebError::DatabaseError(e).into()
            })
    }
}


// ---- Get Single User

#[derive(Debug)]
pub struct GetSingleUser {
    pub id: Option<i32>,
    pub username: Option<String>,
}

impl Message for GetSingleUser {
    type Result = Result<User, Error>;
}

impl Handler<GetSingleUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: GetSingleUser, _: &mut Self::Context) -> <Self as Handler<GetSingleUser>>::Result {
        trace!("{:?} received by DBExecutor", msg);

        use db::schema::users;

        let conn: &DBConn = &self.0.get().unwrap();

        if let Some(uid) = msg.id {
            return users::dsl::users
                .select(ALL_COLUMNS)
                .filter(users::dsl::id.eq(uid))
                .get_result::<User>(conn)
                .map_err(|e| {
                    error!("Error selecting user by id: {}: {:?}", uid, e);
                    WebError::DatabaseError(e).into()
                });
        } else if let Some(username) = msg.username {
            return users::dsl::users
                .select(ALL_COLUMNS)
                .filter(users::dsl::username.eq(&username))
                .get_result::<User>(conn)
                .map_err(|e| {
                    error!("Error selecting user by username: {}: {:?}", &username, e);
                    WebError::DatabaseError(e).into()
                });
        } else {
            error!("GetSinglUser message sent to DB Worker without id or username defined");
            Err(WebError::InternalError.into())
        }
    }
}