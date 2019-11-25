# ![Nightingale](/doc/logo.svg)
Tiny binary for gRPC services for [`HEALTHCHECK` docker command](https://docs.docker.com/engine/reference/builder/#healthcheck) which calls the [gRPC recommended health check service](https://github.com/grpc/grpc/blob/master/doc/health-checking.md)

[![Build Status](https://github.com/zakhenry/nightingale/workflows/Continuous%20integration/badge.svg)](https://github.com/zakhenry/nightingale/actions)
[![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/xiphiaz/nightingale)](https://hub.docker.com/repository/docker/xiphiaz/nightingale/builds)
[![GitHub](https://img.shields.io/github/license/zakhenry/nightingale)](https://raw.githubusercontent.com/zakhenry/nightingale/master/LICENSE)

# Features
* Self contained binary
    * will work in `FROM scratch` images
    * same binary will work most <sup>all?</sup> linux x86_64 based images
* Small - `2.3mb` statically linked binary
* Quick installation using docker multi-build pattern

# Prerequisites
Your gRPC server must have a service which implements the [gRPC health checking protocol](https://github.com/grpc/grpc/blob/master/doc/health-checking.md)
<details>
  <summary>View .proto file:</summary>
  <!-- embedme proto/health_check.proto#L3-L24 -->

  ```proto
  syntax = "proto3";

  package grpc.health.v1;

  message HealthCheckRequest {
      string service = 1;
  }

  message HealthCheckResponse {
      enum ServingStatus {
          UNKNOWN = 0;
          SERVING = 1;
          NOT_SERVING = 2;
      }
      ServingStatus status = 1;
  }

  service Health {
      rpc Check(HealthCheckRequest) returns (HealthCheckResponse);

      rpc Watch(HealthCheckRequest) returns (stream HealthCheckResponse);
  }
  ```
</details>

Your `Health` service only needs to implement the unary `Check` rpc, as nightingale
does not use the streaming endpoint.
Feel free to throw [`12: Unimplemented`](https://github.com/grpc/grpc/blob/master/doc/statuscodes.md) in the `Watch` endpoint.

# Installation

In your `Dockerfile` add the following lines (this is using the [multi stage build strategy](https://docs.docker.com/develop/develop-images/multistage-build/))

```
COPY --from=xiphiaz/nightingale:latest /nightingale /nightingale

HEALTHCHECK CMD /nightingale
```

## Nightingale Options

Note that options like `--interval` and `--timeout` should be passed to `HEALTCHECK`, NOT `nightingale`
<!-- embedme doc/help-output.txt#L5-L15 -->
```
USAGE:
    nightingale [OPTIONS] --host <host> --port <port>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --host <host>          Set host to use [default: [::1]]
    -p, --port <port>          Set port to use [default: 50051]
    -s, --service <service>    Which gRPC service request healthcheck for [default: ]
```

Comprehensive install:
```
COPY --from=xiphiaz/nightingale:latest /nightingale /nightingale

HEALTHCHECK --interval=5s --timeout=3s --start-period=10s --retries=5 CMD /nightingale --host 0.0.0.0 --port 50051 --service example.ExampleService
```

## Usage
When your docker container starts, nightingale will be run every `--interval` (default 30s)
which will in turn call the `Check` rpc endpoint of your `Health` service.
* If this returns `SERVING`, `nightingale` will exit `0` and the container is considered `"healthy"`.
* If this returns `NOT_SERVING` or `UNKNOWN` `nightingale` will exit `1` and the container is considered `"unhealthy"`.
* If this doesn't return at all, after `--timeout` (Default 30s), docker will cancel `nightingale` and consider the container `"unhealthy"`.

You can verify the behavior and configuration is correct by running your container locally,
and running the following command to view the docker-determined health status:
```
docker inspect --format='{{json .State.Health}}' $(docker ps -q  --filter ancestor=your-image-name) | jq
```

Note the interval configuration - you need to wait some time before seeing the docker log of health.

## Future tasks
* Handle multiple service checks
* Clean up text output as docker inspect is very limiting in output format
* Unit tests over the input validation

# Contributing

1. Install rust
    https://www.rust-lang.org/tools/install
2. Build & run
    ```sh

    rustup install beta

    rustup component add rustfmt --toolchain beta

    cargo +beta run --bin nightingale

    ```
3. Builder docker image
    ```sh
    docker build -t nightingale:latest .
    ```
