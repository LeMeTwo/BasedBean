#!/usr/bin/env bash

get() {
    echo GET:
    curl -v -XGET -H "Content-type: application/json" 'localhost:8080/v1/key'
}

delete() {
    echo DELETE:
    curl -v -XDELETE -H "Content-type: application/json" "localhost:8080/v1/key/$1"
}

if declare -f "$1" >/dev/null 2>&1; then
  "$@"
else
  echo "Method $1 not supported" >&2
  exit 1
fi
