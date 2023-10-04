use async_graphql::{Context, InputObject, Object, Result};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::deadpool::Pool;
use crate::models::User;
use crate::ToDoError;

#[derive(Default)]
pub struct LoginQuery;

#[derive(InputObject)]
pub struct ILogIn {
    pub username: String,
    pub password: String,
}

#[Object]
impl LoginQuery {
    async fn login<'ctx>(&self, ctx: &Context<'ctx>, credentials: ILogIn) -> Result<bool> {
        log::debug!(
            "Logging in using email: {} and password: {}",
            credentials.username,
            credentials.password
        );

        // TODO: if user with `username` already logged in - return

        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;

        use crate::schema::users::dsl::{username, users};

        let user = users
            .filter(username.eq(&credentials.username))
            .first::<User>(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to log in: {}", e);
                ToDoError::WrongCredentials
            })?;

        log::debug!("User logged in: {:#?}", user);

        // TODO: save user with `username` logged in

        Ok(true)
    }
}