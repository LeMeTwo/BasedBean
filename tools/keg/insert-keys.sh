#!/usr/bin/env bash

docker exec keg-keydb sh -c 'seq 1000 9999 | sed "s/\(\S*\)/sadd \1.avail \"\"/" | keydb-cli > /dev/null'
