# ![Nightingale](/doc/logo.svg)
Tiny binary for gRPC services for [`HEALTHCHECK` docker command](https://docs.docker.com/engine/reference/builder/#healthcheck) which calls the [gRPC recommended health check service](https://github.com/grpc/grpc/blob/master/doc/health-checking.md)

![MicroBadger Size (tag)](https://img.shields.io/microbadger/image-size/xiphiaz/nightingale/latest)
![Docker Build Status](https://img.shields.io/docker/build/xiphiaz/nightingale)
![GitHub](https://img.shields.io/github/license/zakhenry/nightingale)

# Installation

In your `Dockerfile` add the following lines (this is using the [multi stage build strategy](https://docs.docker.com/develop/develop-images/multistage-build/))

```
COPY --from=nightingale:latest /nightingale /nightingale

HEALTHCHECK CMD /nightingale --host 0.0.0.0 --port 50051
```

## Options
@todo derive from binary directly?

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
