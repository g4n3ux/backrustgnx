/*use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use crate::graphql::MySchema;

pub async fn graphql_handler(
    State(schema): State<MySchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}*/

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse}, // Importamos para devolver HTML
};
use async_graphql::http::{GraphiQLSource}; // El generador de HTML
use crate::graphql::MySchema;

// Este es el que ya tenías (para procesar datos)
pub async fn graphql_handler(
    State(schema): State<MySchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// NUEVO: Este es para mostrar la interfaz en el navegador
pub async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .finish(),
    )
}