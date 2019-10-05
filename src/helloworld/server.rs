use tonic::{transport::Server, Request, Response, Status};

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

use nightingale::{server::{Health, HealthServer}, HealthCheckResponse, HealthCheckRequest, health_check_response};

use std::collections::VecDeque;
use futures_util::join;

type HealthCheckResult<T> = Result<Response<T>, Status>;
type ServerStreamingHealthCheckStream = VecDeque<Result<HealthCheckResponse, Status>>;

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
pub struct ExampleServer;

#[tonic::async_trait]
impl Health for ExampleServer {

    async fn check(&self, request: Request<HealthCheckRequest>) -> Result<Response<HealthCheckResponse>, Status> {
        println!("Got healthcheck request: {:?}", request);

        let res = HealthCheckResponse {
            status: health_check_response::ServingStatus::Serving as i32
        };

        Ok(Response::new(res))
    }

//    async fn watch(
//        &self,
//        _: Request<HealthCheckRequest>,
//    ) -> HealthCheckResult<ServerStreamingHealthCheckStream> {
//        Err(Status::unimplemented("not implemented"))
//    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    let greeter_server = Server::builder()
        .serve(addr, GreeterServer::new(greeter));

    let example = ExampleServer::default();
    let health_server = Server::builder()
        .serve(addr, HealthServer::new(example));

    join!(greeter_server, health_server);

    Ok(())
}
