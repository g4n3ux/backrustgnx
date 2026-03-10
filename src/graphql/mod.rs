use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use sqlx::PgPool;

mod query;
pub use query::QueryRoot;

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

// Asegúrate de que el parámetro 'pool' esté aquí:
pub fn create_schema(pool: PgPool) -> MySchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool) // <--- Importante para que los resolvers usen la DB
        .finish()
}