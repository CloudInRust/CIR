#![allow(proc_macro_derive_resolution_fallback)]

use chrono::NaiveDateTime;
use diesel::prelude::*;

use db::schema::groups;
use error::WebError;

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct Group {
    pub id: i32,
    pub display_name: String,
    pub created_on: NaiveDateTime,
    pub modified_on: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "groups"]
pub struct NewGroup<'a> {
    pub display_name: &'a str,
}

type AllColumns = (
    groups::id,
    groups::display_name,
    groups::created_on,
    groups::modified_on
);

pub const ALL_COLUMNS: AllColumns = (
    groups::id,
    groups::display_name,
    groups::created_on,
    groups::modified_on
);

// DB ACTOR ACTIONS---------------------------------------
use actix::prelude::*;
use actix_web::*;

use db::db_actor::DbExecutor;
use db::DBConn;


// ---- List All Groups

pub struct ListAllGroups {}

impl Message for ListAllGroups {
    type Result = Result<Vec<Group>, Error>;
}

impl Handler<ListAllGroups> for DbExecutor {
    type Result = Result<Vec<Group>, Error>;

    fn handle(&mut self, _: ListAllGroups, _: &mut Self::Context) -> <Self as Handler<ListAllGroups>>::Result {
        trace!("ListAllGroups received by DBexecutor");

        use db::schema::groups::dsl::*;

        let conn: &DBConn = &self.0.get().unwrap();
        groups
            .select(ALL_COLUMNS)
            .load::<Group>(conn)
            .map_err(|e| {
                error!("Error selecting all groups: {:?}", e);
                WebError::DatabaseError(e).into()
            })
    }
}


// ---- Get Single Group

#[derive(Debug)]
pub struct GetSingleGroup {
    pub id: i32,
}

impl Message for GetSingleGroup {
    type Result = Result<Group, Error>;
}

impl Handler<GetSingleGroup> for DbExecutor {
    type Result = Result<Group, Error>;

    fn handle(&mut self, msg: GetSingleGroup, _: &mut Self::Context) -> <Self as Handler<GetSingleGroup>>::Result {
        trace!("{:?} received by DBExecutor", msg);

        use db::schema::groups;

        let conn: &DBConn = &self.0.get().unwrap();

        return groups::dsl::groups
            .select(ALL_COLUMNS)
            .filter(groups::dsl::id.eq(msg.id))
            .get_result::<Group>(conn)
            .map_err(|e| {
                error!("Error selecting group by id: {}: {:?}", msg.id, e);
                WebError::DatabaseError(e).into()
            });
    }
}