# service-api

Service API for Website

Powered by Rocket.

## Install

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

## Setup

```bash
$ rustup default nightly

$ rustup override set nightly

$ rustup update && cargo update
```

## Rust Multi-stage build

The dockerized image is only `13MB` in size:

```bash
# Build the docker image
$ docker build -t engineersmy/service-api .

$ docker images | grep service-api
engineersmy/service-api                                                     latest              8f4359bacb1c        16 hours ago        13MB
```

## Credit

* Project reference [alextanhongpin/rust-api](https://github.com/alextanhongpin/rust-api)
