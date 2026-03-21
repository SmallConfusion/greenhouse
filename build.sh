#!/bin/bash

set -e

cargo zigbuild -r --target=aarch64-unknown-linux-musl
podman build -t ghcr.io/smallconfusion/greenhouse --platform=arm64 .
podman push ghcr.io/smallconfusion/greenhouse:latest
