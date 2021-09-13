set -x


DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=wellness}
DB_PORT=${POSTGRES_PORT:=5432}

if [ -z ${SKIP_DOCKER} ]
then
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p ${DB_PORT}:5432 \
        -d postgres \
        postgres -N 1000
fi

export PGPASSWORD=${DB_PASSWORD}
until psql -h "localhost" -U ${DB_USER} -p ${DB_PORT} -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}


sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"