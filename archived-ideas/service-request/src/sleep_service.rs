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

struct SleepService<WEBSERVICE> {
    inner: WEBSERVICE,
}

impl<WEBSERVICE, REQUEST> Service<REQUEST> for SleepService<WEBSERVICE>
where
    WEBSERVICE: Service<REQUEST>,
    <WEBSERVICE as Service<REQUEST>>::Future: 'static,
{
    type Response = WEBSERVICE::Response;
    type Error = WEBSERVICE::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: REQUEST) -> Self::Future {
        let fut = self.inner.call(req);

        Box::pin(async move {
            // imagine we're fetching data from an API instead
            sleep(Duration::from_millis(250)).await;

            fut.await
        })
    }
}

struct MyService;

impl Service<Request<Body>> for MyService {
    type Response = Response<Body>;
    type Error = Infallible;
    // this future is no longer boxed! it's a future that immediately completes.
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let body = format!("you've requested {:?}", req.uri());
        let res = Response::builder().body(Body::from(body)).unwrap();
        std::future::ready(Ok(res))
    }
}