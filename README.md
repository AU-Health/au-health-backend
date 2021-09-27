# AU Health Backend

## Running the server locally (for frontend development)

You need [Docker](https://www.docker.com/) installed on your computer first.

Clone this repo to your computer if you haven't.

Then run this command inside the top-level directory: `docker compose up -d`

This should download all of the images you need, build the server, and then run the databases and server. It may take a few minutes especially on the first run.

The `.env` file controls the base admin email and password.

## Prepare the sqlx queries for offline building

`cargo sqlx prepare -- --lib`
