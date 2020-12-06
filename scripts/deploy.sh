#!/bin/bash
PACKAGES=$(grep "'" Cargo.toml | grep -v '#' | tr -d "'" | tr -d ",")



deploy() {
    for PACKAGE_NAME in ${PACKAGES}
    do
        if [ X"${PACKAGE_NAME}" != X"brickpack" ];then
            IMAGE_NAME="afsec/${PACKAGE_NAME}"
            echo
            echo "Deploying ${IMAGE_NAME}..."
            pushd ${PACKAGE_NAME}
            docker build -t ${IMAGE_NAME} .
            popd
        fi
    done
    exit 0
}

undeploy() {
    for PACKAGE_NAME in ${PACKAGES}
    do
        if [ X"${PACKAGE_NAME}" != X"brickpack" ];then
            IMAGE_NAME="afsec/${PACKAGE_NAME}"
            echo
    	    echo "Undeploying ${IMAGE_NAME}..."
            CONTAINERS=$((docker ps -a ; docker ps) | grep "${IMAGE_NAME}" | awk '{print $1}')
            [ X"${CONTAINERS}" != X"" ] && docker rm ${CONTAINERS}
	        docker rmi ${IMAGE_NAME}:latest
        fi
    done
    exit 0
}


[ X"${1}" == X"-u" ] && undeploy || deploy

