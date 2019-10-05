//#![deny(warnings)]

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::{client::GreeterClient, HelloRequest};

pub mod nightingale {
    tonic::include_proto!("grpc.health.v1");
}

use nightingale::{client::HealthClient, HealthCheckRequest};

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {

            let client_connection = GreeterClient::connect("http://[::1]:50051");

            let request = tonic::Request::new(HelloRequest {
                name: "hello".into(),
            });

            match client_connection {
                Ok(mut client) => {
                    match client.say_hello(request).await {
                        Ok(res) => {
                            println!("RESPONSE={:?}", res);
                            return Ok(Response::new(Body::from("oK!")))
                        },
                        Err(err) => {
                            return Ok(Response::new(Body::from("not ok!")))
                        }
                    }



                }
                Err(err) => Ok(Response::new(Body::from("no ok!")))
            }

        }

        (&Method::GET, "/healthcheck") => {

            let client_connection = HealthClient::connect("http://[::1]:50051");

            let request = tonic::Request::new(HealthCheckRequest {
                service: "any".into()
            });

            match client_connection {
                Ok(mut client) => {
                    match client.check(request).await {
                        Ok(res) => {
                            println!("RESPONSE={:?}", res);
                            return Ok(Response::new(Body::from("healthy!")))
                        },
                        Err(err) => {
                            println!("ERR={:?}", err);
                            return Ok(Response::new(Body::from("not healthy!")))
                        }
                    }



                }
                Err(err) => Ok(Response::new(Body::from("not healthy!")))
            }

        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| {
        async {
            Ok::<_, hyper::Error>(service_fn(echo))
        }
    });

    let server = Server::bind(&addr)
        .serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
