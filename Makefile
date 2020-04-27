all: build run


build:
	./scripts/build.sh

check:
	./scripts/check.sh

run:
	./scripts/run.sh

deploy:
	docker build -t afsec/concierge-db .
	docker run --name concierge-db -d -p 3341:3341 afsec/concierge-db
	docker logs -f concierge-db

clean:
	rm -rf ./dist
	cargo clean

undeploy:
	docker rm -f concierge-db
	docker rmi afsec/concierge-db:latest
