# clearstats-auth

Auth service

## Environment Variables

```bash
export ENV=local

export DATABASE_URL=mysql://root:clearstats-pw@localhost:3306/clearstats

export JWT_TOKEN_SECRET=access-token-secret
export JWT_TOKEN_LIFETIME=86400
export JWT_REFRESH_TOKEN_SECRET=refresh-token-secret
export JWT_REFRESH_TOKEN_LIFETIME=864000

export GCP_ACCESS_TOKEN=$(gcloud auth application-default print-access-token)
export GCP_PROJECT_ID=

export MAILERSEND_API_KEY=
```


## Development

### Building project
```
~$ cargo build
```

### Running project
```
~$ cargo run
```

## Production

### Building project
```
~$ cargo build --release
```

### Running project
```
~$ cargo run --release
```
