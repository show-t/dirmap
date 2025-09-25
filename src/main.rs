use anyhow::Result;
use dotenvy;

mod macros;

mod tracing_setup;
use tracing_setup::tracing_init;

mod cli;
mod dispatcher;
use dispatcher::Dispatcher;
mod structure_service;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let _tracing_guard = tracing_init()?;

    let dispatcher = Dispatcher::new();
    dispatcher.dispatch().await?;

    Ok(())

}
