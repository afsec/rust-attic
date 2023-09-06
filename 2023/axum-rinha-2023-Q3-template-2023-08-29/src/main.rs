mod api;
mod model;
mod result;
mod service;
mod repository;

use result::AppResult;
use service::Service;

#[tokio::main]
async fn main() -> AppResult<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    Service::run().await?;

    Ok(())
}
