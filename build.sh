#!/bin/sh

# This will (re)create the generated client code for both the
# importer and the client service, based on the spacetime module table definitions.
spacetime generate --lang rust --out-dir ./services/superhero-client/src/generated --module-path ./spacetime-modules/superhero-server
spacetime generate --lang rust --out-dir ./services/superhero-importer/src/generated --module-path ./spacetime-modules/superhero-server

# docker build . -t superhero/spacetimedb-importer --target superhero-importer
# docker build . -t superhero/spacetimedb-client --target superhero-client

# Only makes sense if the server is running
spacetime build --module-path spacetime-modules/superhero-server
spacetime publish --module-path spacetime-modules/superhero-server --server local
