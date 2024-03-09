pub async fn run(// TODO: pass the config
) -> anyhow::Result<()> {
    let router = crate::presentation::application_router().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
