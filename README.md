# service-api

Service API for Website

Powered by actix-web.

## Develop

```bash
$ cargo install systemfd
$ cargo watch -i .trigger -x build -s 'touch .trigger' &
$ systemfd --no-pid -s http::3000 -- cargo watch -w .trigger -x run
```

## Install

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

## Rust Multi-stage build

The dockerized image is only `xxMB` in size:

```bash
# Build the docker image
$ docker build -t engineersmy/service-api .

$ docker images | grep service-api
engineersmy/service-api                                                     latest              8f4359bacb1c        16 hours ago        13MB
```

## Credit

* Project reference [alextanhongpin/rust-api](https://github.com/alextanhongpin/rust-api)
