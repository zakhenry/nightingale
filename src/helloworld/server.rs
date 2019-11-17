use tonic::{transport::Server, Request, Response, Status};

use futures::Stream;
use std::pin::Pin;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::{
    server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

pub mod nightingale {
    tonic::include_proto!("grpc.health.v1");
}

use nightingale::{
    health_check_response,
    server::{Health, HealthServer},
    HealthCheckRequest, HealthCheckResponse,
};

type HealthCheckResult<T> = Result<Response<T>, Status>;

#[derive(Default)]
pub struct MyGreeter {
    data: String,
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let string = &self.data;

        println!("My data: {:?}", string);

        let reply = hello_world::HelloReply {
            message: "Zomg, it works!".into(),
        };
        Ok(Response::new(reply))
    }
}

#[derive(Default)]
pub struct ExampleServer {
}

#[tonic::async_trait]
impl Health for ExampleServer {
    async fn check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> HealthCheckResult<HealthCheckResponse> {
        println!("Got healthcheck request: {:?}", request);

        let res = HealthCheckResponse {
            status: health_check_response::ServingStatus::Serving as i32,
        };

        Ok(Response::new(res))
    }

    type WatchStream = Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send + Sync>>;

    async fn watch(&self, _: Request<HealthCheckRequest>) -> HealthCheckResult<Self::WatchStream> {
        Err(Status::unimplemented("not implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let greeter = MyGreeter::default();
    let greeter_server = GreeterServer::new(greeter);

    let example = ExampleServer::default();
    let health_server = HealthServer::new(example);

    Server::builder()
        .add_service(greeter_server)
        .add_service(health_server)
        .serve(addr)
        .await?;

    Ok(())
}
