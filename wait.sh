#!/bin/bash

# Define the endpoint URL
URL="http://127.0.0.1:9082/random_fight"
# Loop until the curl call succeeds
while true; do
  echo "Attempting to reach $URL..."
  
  # Make the curl call
  if curl -X POST -v -f -s "$URL" > /dev/null; then
    echo "Successfully connected to $URL"
    break
  else
    echo "Failed to connect to $URL. Retrying in 5 seconds..."
    sleep 1
  fi
done
k6 run low.js
