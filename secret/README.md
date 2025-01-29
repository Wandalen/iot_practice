# Keys

This document provides a concise example of an environment configuration script, used to set up environment variables for a project.
These variables configure application behavior without altering the code.

## Example of `.key/-env.sh`

```bash
# Username for connecting to the application-specific PostgreSQL database
POSTGRES_USER='user1'
# Password for the application PostgreSQL user
POSTGRES_PASSWORD='password1'
# Hostname and port of the PostgreSQL database
POSTGRES_HOST='localhost'
POSTGRES_PORT='5432'
# Name of the application-specific PostgreSQL database
POSTGRES_DB='app1'
# Connection string for an app PostgreSQL database, including username, password, host, and database name
# DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DB}?sslmode=require"
DATABASE_URL="postgres://user1:password1@localhost:5432/app1?sslmode=require"

# Connection string for Python simulator
PY_CONNECTION_STRING="HostName=localhost;DeviceId=local_device;SharedAccessKey=base64encoded_key"
```

## To Inspect DB

To login to the debug database:

```bash
export PGPASSWORD="password1"
psql -h $POSTGRES_HOST -p $POSTGRES_PORT -U $POSTGRES_USER -d $POSTGRES_DB

# To see list of tables
\d
# To see table details
\d table1
```

## How to Use in Shell

To apply these variables to your current shell session, use:

```bash
. ./secret/-env.sh
```

This command sources the script, making the variables available in your current session. Ensure `-env.sh` is in the `secret` directory relative to your current location.

## How to Use with Docker

To use these environment variables with Docker, you can pass them to your Docker container using the `--env-file` option. Create a `.env` file with the same content as your `secret/-env.sh` and use it when running your Docker container:

```bash
docker run --env-file ./secret/-env your-docker-image
```

This command will start a Docker container with the environment variables defined in your `.env` file.

## How to Use with Docker Compose

To use these environment variables with Docker Compose, you can specify the `.env` file directly when running the `docker-compose` command by using the `--env-file` option:

```bash
docker-compose --env-file ./secret/-env up
```

This command will apply the environment variables from the `.env` file to all services defined in your `docker-compose.yml` file when you run `docker-compose up`.

Ensure that your `.env` file is correctly formatted and accessible to Docker and Docker Compose commands.