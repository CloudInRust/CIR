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
    pub modified_on: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
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

impl User {
    pub fn get_all(conn: &DBConn) -> Result<Vec<User>, WebError> {
        users::dsl::users
            .select(ALL_COLUMNS)
            .load::<User>(conn)
            .map_err(|e| {
                error!("Error selecting all users: {:?}", e);
                WebError::DatabaseError(e).into()
            })
    }

    pub fn get_by_id(conn: &DBConn, uid: i32) -> Result<User, WebError> {
        users::dsl::users
            .select(ALL_COLUMNS)
            .filter(users::dsl::id.eq(uid))
            .get_result::<User>(conn)
            .map_err(|e| {
                error!("Error selecting user by id: {}: {:?}", uid, e);
                WebError::DatabaseError(e).into()
            })
    }

    pub fn get_by_name(conn: &DBConn, name: &str) -> Result<User, WebError> {
        users::dsl::users
            .select(ALL_COLUMNS)
            .filter(users::dsl::display_name.eq(&name))
            .get_result::<User>(conn)
            .map_err(|e| {
                error!("Error selecting user by name: {}: {:?}", &name, e);
                WebError::DatabaseError(e).into()
            })
    }
}

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

        let conn: &DBConn = &self.0.get().unwrap();
        Ok(User::get_all(conn)?)
    }
}


// ---- Get Single User

#[derive(Debug)]
pub struct GetSingleUser {
    pub id: Option<i32>,
    pub name: Option<String>,
}

impl Message for GetSingleUser {
    type Result = Result<User, Error>;
}

impl Handler<GetSingleUser> for DbExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, msg: GetSingleUser, _: &mut Self::Context) -> <Self as Handler<GetSingleUser>>::Result {
        trace!("{:?} received by DBExecutor", msg);

        let conn: &DBConn = &self.0.get().unwrap();

        if let Some(uid) = msg.id {
            return Ok(User::get_by_id(conn, uid)?);
        } else if let Some(name) = msg.name {
            return Ok(User::get_by_name(conn, &name)?);
        } else {
            error!("GetSinglUser message sent to DB Worker without id or name defined");
            Err(WebError::InternalError.into())
        }
    }
}