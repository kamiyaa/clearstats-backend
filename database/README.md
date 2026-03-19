# clearstats-database

SQL Database schema for Indaggo

## Prerequisites

 - sqlx installed `cargo install sqlx`
   - https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md

## Development

Start a local MySQL database instance for local development.

```sh
cd database/
docker compose up
```

Test connection

```sh
export MYSQL_ROOT_PASSWORD=""
mysql -h localhost -P 3306 --protocol=tcp -u root clearstats -p${MYSQL_ROOT_PASSWORD}
```

Create table schemas

```sh
export DATABASE_URL=mysql://root:${MYSQL_ROOT_PASSWORD}@localhost:3306/indaggo
sqlx migrate run
```

Populate database with test data

```sh
./database/scripts/apply-sql-scripts.sh fixtures/*.sql
```

## Run unittests

```sh
export DATABASE_URL=mysql://root:${MYSQL_ROOT_PASSWORD}@localhost:3306/indaggo
cargo test
```
