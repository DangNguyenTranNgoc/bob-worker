use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase, sqlite::SqlitePoolOptions};

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    if !Sqlite::database_exists("sqlite://./data/bob.db")
        .await
        .unwrap_or(false)
    {
        println!("Creating database sqlite://./data/bob.db");
        match Sqlite::create_database("sqlite://./data/bob.db").await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://./data/bob.db")
        .await?;

    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

pub async fn validate_key(pool: &SqlitePool, key: &str) -> bool {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(1) FROM api_keys WHERE key = ? AND revoked = 0")
        .bind(key)
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
    row.0 > 0
}
