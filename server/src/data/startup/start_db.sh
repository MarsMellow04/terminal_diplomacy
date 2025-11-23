#!/bin/bash
CONTAINER_NAME="postgres"
PASSWORD="mysecretpassword"
VOLUME_NAME="pgdata"
SQL_FILE="schema.sql"

set -e

echo "Starting PostgreSQL container..."
# Using different port due to postgres being on my laptop multiple times
docker run -d \
  --name "$CONTAINER_NAME" \
  -e POSTGRES_PASSWORD="$PASSWORD" \
  -v "$VOLUME_NAME":/var/lib/postgresql/data \
  -p 5433:5432 \
  postgres

echo "Waiting for PostgreSQL to become ready..."
until docker exec "$CONTAINER_NAME" pg_isready -U postgres > /dev/null 2>&1; do
  sleep 1
done

echo "PostgreSQL is ready. Creating schema..."
docker exec -i "$CONTAINER_NAME" psql -U postgres --set ON_ERROR_STOP=1 < "$SQL_FILE"

echo "✔ Setup complete!"
