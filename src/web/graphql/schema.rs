use chrono::{Utc, DateTime, NaiveDateTime};
use juniper::{RootNode, FieldResult};

#[derive(GraphQLObject)]
#[graphql(description = "An existing user")]
struct User {
    id: String,
    username: String,
    email: String,
    display_name: String,
    created_on: NaiveDateTime,
    modified_on: NaiveDateTime
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A new user to be inserted")]
struct NewUser {
    email: String,
    display_name: String,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field users(&executor) -> FieldResult<Vec<User>> {
        Ok(vec![
        User{
            id: "111".to_owned(),
            username: "111".to_owned(),
            email: "111111".to_owned(),
            display_name: "1111".to_owned(),
            created_on: Utc::now().naive_utc(),
            modified_on: Utc::now().naive_utc()
        },
        User{
            id: "222".to_owned(),
            username: "111".to_owned(),
            email: "111111".to_owned(),
            display_name: "1111".to_owned(),
            created_on: Utc::now().naive_utc(),
            modified_on: Utc::now().naive_utc()
        }
        ])
    }

    field user(&executor, id: String) -> FieldResult<User> {
        Ok(
        User{
            id: "111".to_owned(),
            username: "111".to_owned(),
            email: "111111".to_owned(),
            display_name: "1111".to_owned(),
            created_on: Utc::now().naive_utc(),
            modified_on: Utc::now().naive_utc()
        })
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createUser(&executor, new_user: NewUser) -> FieldResult<User> {
        Ok(User{
            id: "222".to_owned(),
            username: "111".to_owned(),
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