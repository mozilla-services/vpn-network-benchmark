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

1. Build the project
`cargo build`

2. Run the server
`cargo run`

## Run tests

Run the integration tests:
`cargo test`

## Deployment

This service is deployed using docker containers.

1. [Install docker](https://docs.docker.com/engine/install/)

2. Build an image with:
`docker build -t vpn-network-benchmark:latest .`

3. To run, set environment variables and forward ports:
`docker run -e HOST=0.0.0.0 -e PORT=8080 -p 8080:8080 vpn-network-benchmark:latest`
