#!/bin/bash

DOMAIN="sa.muelh.ug"

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." && pwd )"
CONTAINER="quay.io/letsencrypt/letsencrypt:latest"

docker pull $CONTAINER
docker run -it --rm --name letsencrypt \
	-v "$DIR/www:/var/www" \
	-v "$DIR/keys:/etc/letsencrypt" \
	$CONTAINER --renew certonly --webroot -w /var/www -d $DOMAIN

