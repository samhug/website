#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

pushd "$DIR"

# Update docker images to latest version
docker-compose pull

# Stop server and rebuild from scratch
docker-compose stop
docker-compose rm -f
docker-compose up -d

# Update DNS forwardings through HE
./scripts/update_dns.sh

# Renew SSL certificates through letsencrypt
./scripts/update_certs.sh

# Restart to make nginx use the latest certs
docker-compose restart

popd
