#!/bin/bash
source set_version.sh

docker build -t revoltchat/january:${version} . &&
    docker push revoltchat/january:${version}
