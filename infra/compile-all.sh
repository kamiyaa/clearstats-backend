#!/bin/bash

set -e

BUILD_DIR=build

echo "Compiling all services..."
docker buildx build --build-arg \
    DATABASE_URL=$DATABASE_URL -f \
    docker/Dockerfile.builder --output type=local,dest=output .
echo "Compiled all services."

ALL_SERVICES=(
    clearstats-api
    clearstats-auth
)

echo "Copying services..."

for SERVICE in ${ALL_SERVICES[@]}; do
    mkdir -p $BUILD_DIR/$SERVICE
    cp output/app/$SERVICE $BUILD_DIR/$SERVICE/app
done

echo "Copied services."
