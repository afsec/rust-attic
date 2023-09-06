use std::task::{Context, Poll};

use std::future::Future;

pub trait Service<REQUEST> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: REQUEST) -> Self::Future;
}
