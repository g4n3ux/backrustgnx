mod graphql;
mod handlers;
mod database; // Cambiamos 'mod models' por 'mod database'

use axum::routing::get;
use axum::Router;
use dotenvy::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;
use crate::graphql::create_schema;
use crate::handlers::graphql::{graphql_handler, graphiql};

// Ahora traemos el modelo desde el nuevo módulo database
use crate::database::Usuario; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); 
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no definida en .env");

    // 1. Configuración del Pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 2. Migraciones automáticas
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 3. Test de Query (Validación en tiempo de compilación)
    // SQLx revisará tus archivos en /database/ para validar tipos
    let _usuarios = sqlx::query_as!(
        Usuario,
        r#"SELECT id, nombre, email, password_hash, creado_en, modificado_en, modificado_por FROM usuario"#
    )
    .fetch_all(&pool)
    .await?;
    
    println!("✅ DB Sincronizada y tablas verificadas.");

    // 4. Servidor y Schema
    let port = env::var("PORT").unwrap_or_else(|_| "5000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Pasamos el pool al schema para que los resolvers tengan acceso
    let schema = create_schema(pool.clone()); 

    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .with_state(schema);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    println!("🚀 Servidor listo en http://localhost:{}", port);
    axum::serve(listener, app).await?;

    Ok(())
}