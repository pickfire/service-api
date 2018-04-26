image         = "docker.tuxuri.com/engineersmy/service-api"
date_version 	?= $(shell date +%Y.%m.%d)

build-local:
	alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
	rust-musl-builder cargo build --release

docker:
	docker build -t ${image}:latest .
	docker tag ${image}:latest ${image}:${date_version}
