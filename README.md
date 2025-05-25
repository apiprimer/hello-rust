# Hello from Rust

Uses Rust Actix framework for exposing an API

## Installation

https://www.rust-lang.org/tools/install

## Code

Contains endpoints exposing CRUD methods

## Build

Command to expose the service in your `http://localhost:8080`

```sh
cargo run
```

For different port use 

```sh
PORT=9000 cargo run
```

For docker packaging,

```sh
docker build -t hello-rust .
docker run -e PORT=3000 -p 3000:3000 hello-rust
```
