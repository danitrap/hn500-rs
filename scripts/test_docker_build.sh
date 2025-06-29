#!/bin/bash

echo "Attempting to build Docker image..."

docker compose build

if [ $? -eq 0 ]; then
  echo "Docker image built successfully."
  exit 0
else
  echo "Docker image build failed."
  exit 1
fi
