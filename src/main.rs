//#![deny(warnings)]

pub mod nightingale {
    tonic::include_proto!("grpc.health.v1");
}

use nightingale::{client::HealthClient, health_check_response, HealthCheckRequest};

extern crate clap;
use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let arg_matches = App::new("Nightingale")
        .version("0.0.0")
        .author("Zak Henry @zak")
        .about("Simple, fast and tiny healthcheck cli for gRPC services")
        .arg(
            Arg::with_name("host")
                .takes_value(true)
                .short("h")
                .long("host")
                .default_value("[::1]")
                .help("Set host to use")
                .required(true),
        )
        .arg(
            Arg::with_name("port")
                .takes_value(true)
                .short("p")
                .long("port")
                .default_value("50051")
                .help("Set port to use")
                .required(true),
        )
        .arg(
            Arg::with_name("service")
                .takes_value(true)
                .short("s")
                .long("service")
                .default_value("")
                .help("Which gRPC service request healthcheck for"),
        )
        .get_matches();

    let url = format!(
        "http://{host}:{port}",
        host = arg_matches.value_of("host").expect("host should exist!"),
        port = arg_matches.value_of("port").expect("port should exist!")
    );

    println!("connecting to {}", url);

    let client_connection = HealthClient::connect(url).await;

    let request = tonic::Request::new(HealthCheckRequest {
        service: arg_matches
            .value_of("service")
            .expect("service should exist!")
            .to_string(),
    });

    match client_connection {
        Ok(mut client) => match client.check(request).await {
            Ok(res) => {
                let status: Option<health_check_response::ServingStatus> =
                    health_check_response::ServingStatus::from_i32(res.into_inner().status);

                match status {
                    Some(health_check_response::ServingStatus::NotServing) => {
                        println!("not serving!");
                        return Err(());
                    }
                    Some(health_check_response::ServingStatus::Serving) => {
                        println!("serving :)");
                        return Ok(());
                    }
                    Some(health_check_response::ServingStatus::Unknown) => {
                        println!("unknown");
                        return Err(());
                    }
                    None => {
                        println!("no status");
                        return Err(());
                    }
                }
            }
            Err(err) => {
                println!("SERVICE ERR={:?}", err);
                return Err(());
            }
        },
        Err(err) => {
            println!("CLIENT ERR={:?}", err);
            return Err(());
        }
    }
}
