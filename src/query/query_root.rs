use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn borrow_from_context_data<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&'ctx String> {
        ctx.data::<String>()
    }
}
