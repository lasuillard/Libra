use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    app::cli::execute().await
}
