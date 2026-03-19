# Deployment variables

export PROJECT_ID=$PROJECT_ID
export GCP_ARTIFACT_REPOSITORY=clearstats-prod
export DOCKER_REGISTRY=northamerica-northeast2-docker.pkg.dev

export GCP_CLOUD_RUN_REGION=us-east1
export GCP_CLOUD_RUN_SERVICE_ACCOUNT=$GCP_CLOUD_RUN_SERVICE_ACCOUNT

export ENV=prod
export ENV_VAR_PREFIX=PROD

#####################################

## Runtime env vars
export GCP_BUCKET_PREFIX=prod
export JWT_TOKEN_LIFETIME=86400
export JWT_REFRESH_TOKEN_LIFETIME=864000
export FEATURE_FLAG_BILLING=false