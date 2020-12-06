all: build run

release:
	./scripts/build.sh

build:
	./scripts/build.sh -d

check:
	./scripts/check.sh

fetch:
	./scripts/fetch.sh

clean:
	rm -rf ./dist
	cargo clean

run:
	./scripts/run.sh

deploy:
	./scripts/deploy.sh

undeploy:
	./scripts/deploy.sh -u
