#![allow(dead_code, unused)]
use log::{error, info};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    println!("Using database: {}", db_url);
    let db = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            info!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            error!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let ids: Vec<i32> = vec![2, 3, 6];
    let res = sqlx::query!("UPDATE students SET age=$1 WHERE id = ANY($2)", 10, &ids)
        .execute(&db)
        .await
        .unwrap();
}
