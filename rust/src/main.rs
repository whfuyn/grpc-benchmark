#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

mod example;
use example as pb;

use pb::example_service_server::ExampleService;
use pb::example_service_server::ExampleServiceServer;

use tonic::{Request, Response};

#[derive(Clone)]
struct BenchServer;

#[real_async_trait::real_async_trait]
impl ExampleService for BenchServer {
    async fn call<'a>(
        &'a self,
        _request: Request<pb::Req>,
    ) -> Result<Response<pb::Resp>, tonic::Status> {
        Ok(Response::new(pb::Resp {
            payload: "Hello".into(),
        }))
    }
}

// #[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main() {
    tonic::transport::Server::builder()
        .add_service(ExampleServiceServer::new(BenchServer))
        .serve("0.0.0.0:50051".parse().unwrap())
        .await
        .unwrap();
}
