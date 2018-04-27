IMAGE   = docker.tuxuri.com/engineersmy/service-api
BUILDER = docker run --rm -it -v "$$PWD":/home/rust/src ekidd/rust-musl-builder

build-local:
	$(BUILDER) cargo build --release

docker:
	docker build -t $(IMAGE):latest .
	docker tag $(IMAGE):latest $(IMAGE):$(shell date +%F)
