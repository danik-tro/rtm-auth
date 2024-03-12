use sqlx::postgres::PgPoolOptions;

#[tracing::instrument(err, name = "Get connection pool", skip_all, level = "trace")]
pub async fn get_connection_pool(
    config: &crate::config::Db,
) -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
    let db = PgPoolOptions::new()
        .max_connections(config.pool_size)
        .connect(&config.db_uri)
        .await?;
    Ok(db)
}

#[tracing::instrument(err, name = "Running migrations", skip(pool), level = "debug")]
pub async fn run_migrations(pool: sqlx::Pool<sqlx::Postgres>) -> anyhow::Result<()> {
    sqlx::migrate!().run(&pool).await?;

    Ok(())
}
