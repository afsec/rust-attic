use crate::{api::Api, result::AppResult};

pub struct Service;

impl Service {
    pub async fn run() -> AppResult<()> {
        let listener = &"0.0.0.0:3000".parse()?;

        let api = Api::build().await?;

        axum::Server::bind(listener)
            .serve(api.into_make_service())
            .await?;

        Ok(())
    }
}
