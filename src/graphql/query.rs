use async_graphql::Object;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // Tu primer Query: { hello }
    async fn hello(&self) -> &str {
        "¡Hola! Estás conectado a GraphQL con Rust y Axum"
    }
}