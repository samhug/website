#!/bin/bash

DOMAIN="sa.muelh.ug"

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." && pwd )"

docker run -it --rm --name letsencrypt \
	-v "$DIR/www:/var/www" \
	-v "$DIR/keys:/etc/letsencrypt" \
	quay.io/letsencrypt/letsencrypt:latest --renew certonly --webroot -w /var/www -d $DOMAIN

