#!/bin/sh
spacetime generate --lang rust --out-dir ./services/superhero-client/src/generated --project-path ./spacetime-modules/superhero-server
spacetime generate --lang rust --out-dir ./services/superhero-importer/src/generated --project-path ./spacetime-modules/superhero-server

docker build . -t superhero/spacetimedb-importer --target superhero-importer
docker build . -t superhero/spacetimedb-client --target superhero-client

spacetime build -p spacetime-modules/superhero-server