use async_graphql::SimpleObject;
use crate::query::login::LoginQuery;
use crate::query::query_root::QueryRoot;

mod login;
pub mod query_root;

#[derive(SimpleObject, Default)]
pub struct Query {
    pub query_root: QueryRoot,
    pub login_query: LoginQuery,
}
