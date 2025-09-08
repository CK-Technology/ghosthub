use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    if !Postgres::database_exists(database_url).await? {
        Postgres::create_database(database_url).await?;
        tracing::info!("Database created successfully");
    }

    let pool = PgPool::connect(database_url).await?;
    tracing::info!("Database connection established");
    Ok(pool)
}

pub async fn migrate(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    tracing::info!("Database migrations completed");
    Ok(())
}