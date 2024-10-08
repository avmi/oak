#!/bin/bash
#
# Build configuration for oak_containers_nvidiasystem_image.
#
export PACKAGE_NAME=oak_containers_nvidia_system_image

export BUILD_COMMAND=(
  nix
  develop
  .#systemImageProvenance
  --command
  just
  oak_containers_nvidia_system_image
)

export SUBJECT_PATHS=(
  oak_containers/system_image/target/nvidia_image.tar.xz
)
