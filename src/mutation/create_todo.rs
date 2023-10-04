use async_graphql::{Context, InputObject, Object, Result};
use chrono::NaiveDate;

use diesel_async::{
    AsyncPgConnection,
    RunQueryDsl,
    pooled_connection::deadpool::Pool,
};

use crate::{schema::todos, ToDoError};
use crate::models::NewToDo;

#[derive(Default)]
pub struct CreateToDoMut;

#[derive(InputObject, Debug)]
pub struct ICreateTodo {
    pub username: String,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<String>,
}

impl<'a> From<&'a ICreateTodo> for NewToDo<'a> {
    fn from(input: &'a ICreateTodo) -> Self {
        log::debug!("FROM ICreateTodo TO NewToDo");
        log::debug!("input: {:#?}", input);

        let data = NaiveDate::parse_from_str(input.due_date.as_deref().unwrap(), "%Y-%m-%d").unwrap();

        let new_todo = Self {
            id: 0,
            username: &input.username,
            title: &input.title,
            completed: false,
            description: input.description.as_deref(),
            due_date: Some(data),
        };

        log::debug!("new_todo: {:#?}", new_todo);

        new_todo
    }
}

#[Object]
impl CreateToDoMut {
    async fn create_todo<'ctx>(&self, ctx: &Context<'ctx>, data: ICreateTodo) -> Result<bool> {
        log::debug!("CREATE TODO");
        log::debug!("INPUT: {:#?}", data);

        // TODO: check if user with `username` logged in another - return

        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;

        let new_todo: NewToDo = (&data).into();
        log::debug!("NEW TODO: {:#?}", new_todo);

        diesel::insert_into(todos::table)
            .values(new_todo)
            .execute(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to create todo: {}", e);
                ToDoError::ToDoWithIdAlreadyExists
            })?;

        log::debug!("OK");

        Ok(true)
    }
}