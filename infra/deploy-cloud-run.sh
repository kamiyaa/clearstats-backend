#!/bin/bash

if [[ $# -ne 1 ]]; then
	echo "usage: $0 env"
	exit 1
fi

set -e

ENV_FILE="$1"

# Load environment variables
source "$ENV_FILE"

CWD=$(dirname "$0")

echo "Deploying services to cloud run"

######################################

"$CWD/deploy/clearstats-auth.sh"
"$CWD/deploy/clearstats-api.sh"

######################################

echo "Deployed services to cloud run"
