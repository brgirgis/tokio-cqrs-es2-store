# cqrs-grpc-demo

**An async demo application using the [cqrs-es2](https://github.com/brgirgis/cqrs-es2) framework.**

## Requirements

- rust stable
- docker and [docker-compose](https://docs.docker.com/compose/) for starting database instances

Alternatively, if a standard SQL database instance is running locally it can be utilized instead of the docker instances,
see [the init script](../../db/postgres/init.sql) for the expected table configuration.

## Installation

Clone this repository

    git clone https://github.com/brgirgis/tokio-cqrs-es2-store

Start the docker stack and enter the project folder:

    docker-compose up -d
    cd examples/grpc

Start the application

    cargo run --features with-sqlx-postgres --bin server

Call the API using the provided client for testing the running application:

    cargo run --bin client
