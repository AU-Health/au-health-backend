# AU Health Backend

## Running the server locally (for frontend development)

You need [Docker](https://www.docker.com/) installed on your computer first.

If you don't know much about Docker, that's fine-- it's pretty easy and you can learn more [here](https://www.docker.com/101-tutorial/)

Clone this repo to your computer if you haven't.

Then run this command inside the top-level directory: `docker compose up -d --build`

This should download all of the images you need, build the server, and then run the databases and server. It may take a few minutes especially on the first run. It is much slower than the debug build which we explain further on.

The `.env` file controls the base admin email and password.

## Learning Rust

If you don't know how to use [Rust](https://doc.rust-lang.org/book/title-page.html), first do the [Rustlings](https://github.com/rust-lang/rustlings) tutorial and then come back here. You don't need to do all the exercises, but enough until you feel confident writing and using Rust. If you get stuck on the tutorials, I've found [this blog](https://lazyren.github.io/studylog/rustlings.html#error-handling) to be the most helpful at quickly and correctly explaining the solutions.

This project uses an asynchronous runtime of Rust called [tokio](https://docs.rs/tokio/latest/tokio/). If you want to know more about asynchronous rust, you can do so [here](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html). The async Rust book is not necesarrily finished at the moment, but there should still be enough there to help you generally understand what is going on.

## Learning GraphQL

This is the query language that we use to transfer data between the front and backend (importantly, not the database- we use SQLX for that). If you know REST, you will not struggle much with this. If you have never worked in web programming before, recomend watching the longer crash course.

A word of warning: Most resources for GraphQL use examples in Javascript, which means you will have to figure out how to transfer that knowledge over to Rust more indedependently than if this project were in JS.

[Fireship video](https://www.youtube.com/watch?v=eIQh02xuVw4) 

[More in-depth tutorial](https://www.youtube.com/watch?v=qux4-yWeZvo&t=10937s)

This project uses [Async-GraphQL](https://async-graphql.github.io/async-graphql/en/introduction.html), specifically, which is a library implemented in rust for asynchronous programming.

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

Theres some helpers set up in the `tests/api/helpers` and `tests/api/gql` folders that help set up the queries and server for testing purposes. You can see these used in the existing tests.

### CI

There is a few CI pipelines that run on pushes to branches. You can find those in `.github/workflows/general.yml`.

There is also security audits that use a database of security issues from the Rust community, those are in `.github/workflows/audit-on-push.yml` and `.github/workflows/scheduled-audit.yml`

Dependabot also runs on this repo, and makes automated Pull Requests to upgrade dependencies. If the tests pass on the CI pipeline, you should be free to merge these. If not, fix whatever is broken in using the new package version and then push it yourself.

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

If you are working on the code without the server running, there is a pretty large chance that there will be some errors communicating with the database (because the db's are not running). Those should go away when you run the docker setup/cargo run situation explained earlier.

## Prepare the sqlx queries for offline building

You only need to do this when you make changes to the database schema, and then want to build the Docker container image.

`cargo sqlx prepare -- --lib`
