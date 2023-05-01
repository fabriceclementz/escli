#!/usr/bin/env bash
set -x
set -eo pipefail

RUNNING_CONTAINER=$(docker ps --filter 'name=elasticsearch' --format '{{.ID}}')
if [[ -n $RUNNING_CONTAINER ]]; then
  echo "container is already running"
  exit 1
fi

STOPPED_CONTAINER=$(docker ps -a --filter 'name=elasticsearch' --format '{{.ID}}')
if [[ -n $STOPPED_CONTAINER ]]; then
  echo "container is stopped, restarting..."
  docker start ${STOPPED_CONTAINER}
else
  docker network create esctl-network

  docker run -d --name elasticsearch \
    --net esctl-network \
    -p 9200:9200 \
    -p 9300:9300 \
    -e "discovery.type=single-node" \
    -e "xpack.security.enabled=false" \
    elasticsearch:8.7.0
fi

