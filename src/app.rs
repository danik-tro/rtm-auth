use std::net::SocketAddr;

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

async fn run_server(_config: &crate::config::Config, addr: &SocketAddr) -> anyhow::Result<()> {
    let router = crate::presentation::application_router().await;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Running application on {}", addr);
    axum::serve(listener, router).await?;

    Ok(())
}
