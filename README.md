# Auth Rust Microservice


## Getting started

### Setup database

#### Setup database locally in docker
```bash
$ docker-compose up rtm_auth_db
```

#### Set database url env variable

Default url for local development
```bash
$ export DATABASE_URL=postgres://rtm_auth:rtm_auth@localhost:5432/auth
```

