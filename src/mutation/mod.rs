use async_graphql::SimpleObject;
use crate::mutation::create_todo::CreateToDoMut;
use crate::mutation::sign_up::SignUpMut;

mod sign_up;
mod create_todo;

#[derive(SimpleObject, Default)]
pub struct Mutation {
    pub sign_up_mut: SignUpMut,
    pub create_to_do_mut: CreateToDoMut,
}
