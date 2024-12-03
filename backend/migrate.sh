#!/bin/bash

set -e

# Define default values for PostgreSQL configuration
MIGRATION_DIR="./db/migrations/"

POSTGRES_USER="${POSTGRES_USER:-postgres}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-postgres}"
POSTGRES_HOST="${POSTGRES_HOST:-localhost}"
POSTGRES_PORT="${POSTGRES_PORT:-5432}"
POSTGRES_DB="${POSTGRES_DB:-postgres}"

# Construct the PostgreSQL connection URL
POSTGRES_URL="postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DB?sslmode=disable"

docker run -v "$MIGRATION_DIR:/migrations" --network host migrate/migrate \
    -path=/migrations -database "$POSTGRES_URL" "$@"

