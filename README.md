# VPN Network Benchmark

[![Coverage Status](https://codecov.io/gh/mozilla-services/vpn-network-benchmark/branch/main/graph/badge.svg?token=JW9B9YTOE0)](https://codecov.io/gh/mozilla-services/vpn-network-benchmark)
[![Security audit](https://github.com/mozilla-services/vpn-network-benchmark/actions/workflows/scheduled-audit.yml/badge.svg)](https://github.com/mozilla-services/vpn-network-benchmark/actions/workflows/scheduled-audit.yml)

Microservice for the Mozilla VPN in-app network upload benchmark.

## Endpoints

`/upload`:
- POST only: Returns `200` data is readable

`/health`:
- GET: Returns `200`
- POST: Returns `200`

## Development pre-requisites

### Rust

https://www.rust-lang.org/tools/install

## Run server

1. Create a `.env` file from `.env.local` if you would like to overwrite the default variables:
```
cp .env.local .env
```
Update the environment variables as needed.

2. Build the project
```
cargo build
```

3. Run the server
```
cargo run
```

## Testing

### Run tests

Run the integration tests:
```
cargo test
```

### Test upload while developing locally

For testing purposes you could create a file with random data using for example [dd](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/dd.html) from the terminal. The following command creates 1 MB of data:
```
dd if=/dev/random of=random_data.bin bs=1M count=1
```

You can then POST data to the endpoint `/upload` with:
```
curl -i -X POST --data-binary @random_data.bin -H "Content-type: application/json" http://localhost:{PORT}/upload
```

## Deployment

This service is deployed using docker containers.

1. [Install docker](https://docs.docker.com/engine/install/)

2. Build an image with:
```
docker build -t vpn-network-benchmark:latest .
```

3. To run, set environment variables using a `.env` file or setting them directly in the run command and forward ports:
```
docker run \
  -e HOST=0.0.0.0 \
  -e PORT=8080 \
  -e MAX_PAYLOAD_SIZE=10485760 \ # 10 MB
  -e REPO_LINK=https://github.com/mozilla-services/vpn-network-benchmark \
  -p 8080:8080 \
  vpn-network-benchmark:latest
```
