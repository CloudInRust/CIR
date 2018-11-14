use chrono::{Utc, NaiveDateTime};
use juniper::{RootNode, FieldResult};

use super::graphql_actor::GLContext;
use db::models as db_models;

#[derive(GraphQLObject)]
#[graphql(description = "An existing user")]
struct User {
    id: String,
    email: String,
    display_name: String,
    created_on: NaiveDateTime,
    modified_on: NaiveDateTime,
}

impl<'a> From<&'a db_models::User> for User {
    fn from(user: &db_models::User) -> Self {
        User {
            id: format!("{}", user.id),
            email: user.email.clone(),
            display_name: user.display_name.clone(),
            created_on: user.created_on,
            modified_on: user.modified_on,
        }
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A new user to be inserted")]
struct NewUser {
    email: String,
    display_name: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "An existing user group")]
struct Group {
    pub id: i32,
    pub display_name: String,
    pub created_on: NaiveDateTime,
    pub modified_on: NaiveDateTime,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A new user group to be inserted")]
struct NewGroup {
    display_name: String,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: GLContext |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field users(&executor) -> FieldResult<Vec<User>> {
        let conn = &executor.context().db_conn.get().unwrap();
        Ok(
            db_models::User::get_all(conn)?
                .iter().map(|db_user| User::from(db_user))
                .collect()
        )
    }

    field user(&executor, id: String) -> FieldResult<User> {
        Ok(
        User{
            id: "111".to_owned(),
            email: "111111".to_owned(),
            display_name: "1111".to_owned(),
            created_on: Utc::now().naive_utc(),
            modified_on: Utc::now().naive_utc()
        })
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: GLContext |&self| {
    field createUser(&executor, new_user: NewUser) -> FieldResult<User> {
        Ok(User{
            id: "222".to_owned(),
            email: "111111".to_owned(),
            display_name: "1111".to_owned(),
            created_on: Utc::now().naive_utc(),
            modified_on: Utc::now().naive_utc()
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_graphql_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}