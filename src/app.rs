use std::{net::SocketAddr, sync::Arc};

use axum::Extension;

use crate::{
    infrastructure::{databases::postgres::get_connection_pool, repositories::UserSqlxRepository},
    services::RegistrationServiceImpl,
};

/// Run the application with provided CLI arguments.
///
/// # Errors
///
/// Any errors that occur will be returned to the CLI
pub fn run() -> anyhow::Result<()> {
    let cli = crate::cli::parse();
    let config = crate::config::from_file(&cli.config)?;

    if !cli.quiet {
        crate::tracing::init_subscriber(&config.tracing);
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(cli.workers)
        .build()?;

    match cli.commands {
        crate::cli::Commands::RunServer { ref bind } => {
            rt.block_on(run_server(&config, bind))?;
        }
    }
    Ok(())
}

async fn run_server(config: &crate::config::Config, addr: &SocketAddr) -> anyhow::Result<()> {
    let pool = get_connection_pool(&config.db).await?;
    let user_repository = Arc::new(UserSqlxRepository::new(pool.clone()));
    let registration_service = Arc::new(RegistrationServiceImpl::new(user_repository));

    let router = crate::api::application_router()
        .await
        .layer(Extension(registration_service));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Running application on {}", addr);
    axum::serve(listener, router).await?;

    Ok(())
}
