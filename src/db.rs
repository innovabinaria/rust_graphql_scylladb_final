use async_graphql::{SimpleObject, InputObject};

#[derive(Clone, Debug, SimpleObject)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(InputObject)]
pub struct NewUser {
    pub id: i32,
    pub name: String,
}

