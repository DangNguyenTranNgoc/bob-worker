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
