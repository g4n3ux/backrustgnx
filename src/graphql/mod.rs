pub mod query;

use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use self::query::QueryRoot;

// Alias para no escribir todo el tipo largo siempre
pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> MySchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}