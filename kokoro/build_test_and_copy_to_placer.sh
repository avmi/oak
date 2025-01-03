#!/usr/bin/env bash

# shellcheck source=./kokoro/helpers/copy_binaries.sh
source "$(dirname "$0")/helpers/copy_binaries.sh"
# shellcheck source=./kokoro/helpers/common.sh
source "$(dirname "$0")/helpers/common.sh"

./scripts/docker_pull
# TODO: b/337266665 - Remove bazel-cache-test logic once we are satisfied with remote cache hits.
./scripts/docker_run nix develop .#default --command just build-and-test bazel-cache-test containers_placer_artifacts
./scripts/git_check_diff

# Upload the bazel execution logs as Kokoro artifacts so we can debug remote cache. This should
# be removed once the debugging is completed. We use the binaries folder only because it is
# already configured to receive and upload artifacts to CNS.
mkdir --parents "${KOKORO_ARTIFACTS_DIR}/binaries/test"
cp --preserve=timestamps \
    ./target/bazel_* \
    "${KOKORO_ARTIFACTS_DIR}/binaries/test"

copy_binaries_to_placer

kokoro_cleanup
