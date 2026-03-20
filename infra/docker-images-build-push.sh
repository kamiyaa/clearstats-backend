#!/bin/bash

set -e

PROJECT_ID=$PROJECT_ID
GCP_REGION=${GCP_REGION:-northamerica-northeast2}
GCP_ARTIFACT_REPOSITORY=$GCP_ARTIFACT_REPOSITORY
DOCKER_REGISTRY=$GCP_REGION-docker.pkg.dev

echo "======= Config ======="
echo "PROJECT_ID=$PROJECT_ID"
echo "GCP_REGION=$GCP_REGION"
echo "GCP_ARTIFACT_REPOSITORY=$GCP_ARTIFACT_REPOSITORY"
echo "DOCKER_REGISTRY=$GCP_REGION-docker.pkg.dev"
echo "======================"

BUILD_DIR=build
ALL_SERVICES=(
    clearstats-auth
    clearstats-api
)

echo "Building docker images..."
for SERVICE in ${ALL_SERVICES[@]}; do
    echo "building $SERVICE..."

    IMAGE=$DOCKER_REGISTRY/$PROJECT_ID/$GCP_ARTIFACT_REPOSITORY/$SERVICE:latest
    docker buildx build \
        -f docker/Dockerfile.runner \
        -t $IMAGE \
        --push \
        $BUILD_DIR/$SERVICE

done
echo "Finished building docker images."

echo "List of docker images"
echo "docker image ls"
docker image ls
