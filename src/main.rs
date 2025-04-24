use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL não encontrada")

    let _pool = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Erro ao conectar com SQLite");

    println!("Conectado ao banco com sucesso!");
}
