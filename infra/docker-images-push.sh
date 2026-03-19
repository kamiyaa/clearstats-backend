#!/bin/bash

set -e

PROJECT_ID=$PROJECT_ID
GCP_REGION=${GCP_REGION:-northamerica-northeast2}
GCP_ARTIFACT_REPOSITORY=$GCP_ARTIFACT_REPOSITORY
DOCKER_REGISTRY=$GCP_REGION-docker.pkg.dev

ALL_SERVICES=(
    clearstats-auth
)

echo "Pushing docker images to GCP..."

for SERVICE in ${ALL_SERVICES[@]}; do
    echo "pushing $SERVICE..."
    IMAGE=$DOCKER_REGISTRY/$PROJECT_ID/$GCP_ARTIFACT_REPOSITORY/$SERVICE:latest
    docker push $IMAGE

done

echo "Pushed docker images to GCP."
gcloud artifacts docker images list
