#!/usr/bin/env bash

if [ "$1" == "up" ]; then
  docker compose up -d

  until [ "$(docker inspect keg-keydb --format '{{.State.Status}}' | grep -c running)" -eq 1 ] ; do
    sleep 5
  done

  ./insert-keys.sh
elif [ "$1" == "down" ]; then
  docker compose down
else
  echo "usage: $0 up|down"
fi
