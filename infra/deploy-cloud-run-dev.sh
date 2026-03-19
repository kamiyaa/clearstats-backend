#!/bin/bash

CWD=$(dirname "$0")

"$CWD/deploy-cloud-run.sh" "$CWD/env/env-dev.sh"
