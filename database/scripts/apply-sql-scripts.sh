#!/bin/sh

DOCKER_CONTAINER_NAME=${DOCKER_CONTAINER_NAME:-indaggo-db}
MYSQL_ROOT_PASSWORD=indaggo-pw

for sql_file in "$@"; do
	echo "Running $sql_file"
	docker exec -i $DOCKER_CONTAINER_NAME sh -c 'exec mysql -uroot -p"$MYSQL_ROOT_PASSWORD" clearstats' < "$sql_file"
done
