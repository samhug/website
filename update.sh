#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

pushd "$DIR"

docker pull nginx:latest
docker-compose stop
docker-compose rm -f
docker-compose up -d

./scripts/update_dns.sh
./scripts/update_certs.sh

popd
