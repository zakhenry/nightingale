# Installation

1. Install rust 
    https://www.rust-lang.org/tools/install
2. Build & run
    ```sh
    
    rustup install beta
    
    rustup component add rustfmt --toolchain beta
    
    cargo +beta run --bin nightingale
    
    cargo +beta run --bin helloworld-server
    
    ```
3. Builder docker image
    ```sh
    docker build -t nightingale:latest .
    ```
4. Run docker image
    ```sh
    docker run -it -p 0.0.0.0:8088:8181 nightingale
    ```
5. Call yourself a [Rustacean](https://en.wiktionary.org/wiki/Rustacean)
