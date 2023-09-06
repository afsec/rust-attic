use hyper::{Body, Request, Response};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::sleep;
use tower::Service;

struct MyService;

impl Service<Request<Body>> for MyService {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // we don't apply any backpressure here, so always ready
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        Box::pin(async move {
            // imagine we're fetching data from an API instead
            sleep(Duration::from_millis(250)).await;

            let body = format!("you've requested {:?}", req.uri());
            let res = Response::builder().body(Body::from(body)).unwrap();
            Ok(res)
        })
    }
}
