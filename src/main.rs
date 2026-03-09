mod graphql;
mod handlers;

use axum::routing::{get};
use axum::Router;
use dotenvy::dotenv;
use std::env;
use crate::graphql::create_schema;
use crate::handlers::graphql::{graphql_handler, graphiql};

#[tokio::main]
async fn main() {
    // 1. Cargar el archivo .env
    dotenv().ok(); 

    // 2. Leer el puerto
    let port = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string());
    
    let addr = format!("0.0.0.0:{}", port);

    // 3. Inicializar el Schema PRIMERO
    // En Rust, el orden importa: no puedes pasar 'schema' a la app si no existe aún.
    let schema = create_schema();

    // 4. Configurar la App (una sola vez)
    let app = Router::new()
        .route(
            "/graphql", 
            get(graphiql).post(graphql_handler) // GET sirve el Playground, POST procesa datos
        )
        .with_state(schema); // Inyectamos el schema como estado

    // 5. Bind e inicio
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    println!("🚀 Servidor corriendo en http://localhost:{}", port);
    println!("📡 Playground: http://localhost:{}/graphql", port);

    axum::serve(listener, app).await.unwrap();
}