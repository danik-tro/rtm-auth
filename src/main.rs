#[tokio::main]
async fn main() -> anyhow::Result<()> {
    rtm_auth::app::run().await
}
