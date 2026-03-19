#!/bin/bash

SERVICE=clearstats-auth
SERVICE_NAME=$ENV-$SERVICE

echo "Deploying ${SERVICE_NAME} to cloud run"

IMAGE=$DOCKER_REGISTRY/$PROJECT_ID/$GCP_ARTIFACT_REPOSITORY/$SERVICE:latest
gcloud run deploy $SERVICE_NAME --image $IMAGE \
    --allow-unauthenticated \
    --region $GCP_CLOUD_RUN_REGION \
    --update-env-vars=ENV=$ENV \
    --update-env-vars=INDAGGO_LAB_URL=$INDAGGO_LAB_URL \
    --update-env-vars=JWT_TOKEN_LIFETIME=$JWT_TOKEN_LIFETIME \
    --update-env-vars=JWT_REFRESH_TOKEN_LIFETIME=$JWT_REFRESH_TOKEN_LIFETIME \
    --update-secrets=DATABASE_URL=${ENV_VAR_PREFIX}_DATABASE_URL:latest \
    --update-secrets=GCP_PROJECT_ID=GCP_PROJECT_ID:latest \
    --update-secrets=JWT_TOKEN_SECRET=${ENV_VAR_PREFIX}_JWT_TOKEN_SECRET:latest \
    --update-secrets=JWT_REFRESH_TOKEN_SECRET=${ENV_VAR_PREFIX}_JWT_REFRESH_TOKEN_SECRET:latest \
    --update-secrets=MAILERSEND_API_KEY=MAILERSEND_API_KEY:latest \
    --update-secrets=SENTRY_DSN_URL=${ENV_VAR_PREFIX}_SENTRY_DSN_URL_BACKEND:latest \
    --update-labels env=$ENV,service=$SERVICE_NAME \
    --execution-environment=gen2