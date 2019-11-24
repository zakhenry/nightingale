# ![Nightingale](/doc/logo.svg)
Tiny binary for gRPC services for [`HEALTHCHECK` docker command](https://docs.docker.com/engine/reference/builder/#healthcheck) which calls the [gRPC recommended health check service](https://github.com/grpc/grpc/blob/master/doc/health-checking.md)

[![Build Status](https://github.com/zakhenry/nightingale/workflows/Continuous%20integration/badge.svg)](https://github.com/zakhenry/nightingale/actions)
[![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/xiphiaz/nightingale)](https://hub.docker.com/repository/docker/xiphiaz/nightingale/builds)
[![GitHub](https://img.shields.io/github/license/zakhenry/nightingale)](https://raw.githubusercontent.com/zakhenry/nightingale/master/LICENSE)

# Installation

In your `Dockerfile` add the following lines (this is using the [multi stage build strategy](https://docs.docker.com/develop/develop-images/multistage-build/))

```
COPY --from=xiphiaz/nightingale:latest /nightingale /nightingale

HEALTHCHECK CMD /nightingale --host 0.0.0.0 --port 50051
```

## Options
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
