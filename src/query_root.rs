use async_graphql::{Context, Object, Result};

pub struct Query;

#[Object]
impl Query {
    async fn borrow_from_context_data<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&'ctx String> {
        ctx.data::<String>()
    }
}
