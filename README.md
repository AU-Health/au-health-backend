# AU Health Backend

## Running the server locally (for frontend development)

You need [Docker](https://www.docker.com/) installed on your computer first.

Clone this repo to your computer if you haven't.

Then run this command inside the top-level directory: `docker compose up -d --build`

This should download all of the images you need, build the server, and then run the databases and server. It may take a few minutes especially on the first run.

The `.env` file controls the base admin email and password.

## Learning Rust

If you don't know how to use Rust, first do the [Rustlings](https://github.com/rust-lang/rustlings) tutorial and then come back here. You don't need to do all the exercises, but enough until you feel confident writing and using Rust.

## Learning GraphQL

[Fireship video](https://www.youtube.com/watch?v=eIQh02xuVw4)

## Backend Development book

[Zero To Production](https://www.zero2prod.com/) is a great book and how I learned to do a lot of the cool stuff in this project. I recommend getting it if you want to dive deep into backend development and learn a ton about Rust and backend APIs. It's a long book though, so don't feel like you need to finish all of it.

## Architecture

### Databases

[Postgres](https://www.postgresql.org/) is the main data store for users, surveys, basically anything. It's a SQL database.

[Redis](https://redis.io/) is the session store, to store user's login sessions. Redis is used for lots of different things but in this app we're just using it for sessions at the moment.

### Libraries

[Axum](https://github.com/tokio-rs/axum) is the web server framework.

[Async-graphql](https://github.com/async-graphql/async-graphql) is the library for implementing GraphQL queries and mutations.

[SQLx](https://github.com/launchbadge/sqlx) is the library for accessing the Postgres database, running queries on it, etc.

[SQLx CLI](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli) is the command line tool for running and creating database migrations. This intergrates nicely with SQLx as it is part of the same project.

### Code layout

Start point is `src/main.rs`.

The server is setup in `src/routes.rs` and `src/startup.rs`.

The configuration for the databases etc is in `src/configuration.rs`, with the actual configuration files in the `configuration` directory.

Check this directory out to see how the app is configured for different environments: local (`cargo run`), docker, and production.

The GraphQL schema is setup from `src/gql/schema.rs`, with each of the resolvers in the `src/gql/resolvers` folder.

Everything that is "domain-specific" (think business logic) should go in the `src/domain` folder, with the GQL resolvers calling out to it.

### Tests

Most of the tests are intergation tests run from the `tests` folder. These require the databases (Postgres and Redis) to be up and running to be able to run the tests. The tests use [Cynic](https://github.com/obmarg/cynic) to query the GraphQL server and check the responses.

## Running server for backend development

Start the databases (Postgres and Redis) with this command:
`docker compose up -d postgres redis`

If this is your first time running the databases, you need to migrate Postgres:

- Make sure you have the SQLx CLI installed:
`https://crates.io/crates/sqlx-cli`

- Migrate the database:
`sqlx migrate run`

Finally start the server with:
`cargo run`

## Prepare the sqlx queries for offline building

You only need to do this when you make changes to the database schema, and then want to build the Docker container image.

`cargo sqlx prepare -- --lib`
