use async_graphql::{Context, Object, Schema, EmptySubscription, Result};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;
use scylla::client::session::Session;
use std::sync::Arc;
use crate::db::{User, NewUser};


#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Session>,
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(state: AppState) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(state)
        .finish()
}

pub async fn graphql_handler(
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self, _ctx: &Context<'_>) -> &str {
        "Hi from GraphQL and Rust"
    }

    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<User> {

        let state = ctx.data::<AppState>()?;
        let query = "SELECT id, name FROM users WHERE id = ?";
        let query_rows = (&*state.db)
            .query_unpaged(query, (id,))
            .await?
            .into_rows_result()?;

        for row in query_rows.rows()? {
            let (int_val, text_val): (i32, &str) = row?;
            return Ok(User {
                id: int_val,
                name: text_val.to_string(),
            });
        }

        Err("User not found".into())
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, input: NewUser) -> Result<User> {
        let state = ctx.data::<AppState>()?;
        let query = "INSERT INTO users (id, name) VALUES (?, ?)";
        let prepared = state.db.prepare(query).await?;
        state.db.execute_iter(prepared, (input.id, &input.name)).await?;
        Ok(User { id: input.id, name: input.name })
    }
}










