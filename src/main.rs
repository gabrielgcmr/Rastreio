use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    let db_url = "sqlite:./database.db"; // ou em memória: "sqlite::memory:"
    let _pool = SqlitePoolOptions::new()
        .connect(db_url)
        .await
        .expect("Erro ao conectar com SQLite");

    println!("Conectado ao banco com sucesso!");
}
