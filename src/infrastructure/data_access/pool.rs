use sqlx::postgres::PgPoolOptions;

pub async fn get_connection_pool(
    config: &crate::config::Db,
) -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
    let db = PgPoolOptions::new()
        .max_connections(config.pool_size)
        .connect(&config.db_uri)
        .await?;
    Ok(db)
}

// #[tracing::instrument(name = "Running migrations", skip(pool), level = "debug")]
// pub async fn run_migrations(pool: sqlx::Pool<sqlx::Postgres>) -> anyhow::Result<()> {
//     sqlx::migrate!()
//         .run(&pool)
//         .await
//         .map_err(|err| {
//             tracing::error!("Failed to run migrations: {}", err);
//             err
//         })
//         .context("Failed to run migrations")?;

//     Ok(())
// }
