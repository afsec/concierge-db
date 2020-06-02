#!/bin/bash

PACKAGE_NAME=$(head Cargo.toml | awk '/^name/{print $3}' | tr -d '"' | tr -d "'")
IMAGE_NAME=$(git remote get-url --all origin | cut -d ':' -f 2 | sed "s/\..*$//g")

deploy() {
	docker build -t ${IMAGE_NAME} .
    exit 0
}

undeploy() {
	docker rmi ${IMAGE_NAME}:latest
    exit 0
}


[ X"${1}" == X"-u" ] && undeploy || deploy
