use anyhow::Result;
use dotenvy;

mod cli;
mod dispatcher;
use dispatcher::Dispatcher;
mod structure_service;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let _tracing_guard = logkit::tracing_init!();

    let dispatcher = Dispatcher::new();
    dispatcher.dispatch().await?;

    Ok(())

}
