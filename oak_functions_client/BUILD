#
# Copyright 2024 The Project Oak Authors
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")

package(
    default_visibility = ["//:internal"],
    licenses = ["notice"],
)

rust_library(
    name = "lib",
    srcs = ["src/lib.rs"],
    crate_name = "oak_functions_client",
    deps = [
        "//micro_rpc",
        "//oak_client",
        "//oak_client/tonic:oak_client_tonic",
        "//oak_proto_rust/grpc",
        "@oak_crates_index//:anyhow",
        "@oak_crates_index//:prost",
        "@oak_crates_index//:tonic",
    ],
)

rust_binary(
    name = "oak_functions_client",
    srcs = ["src/main.rs"],
    deps = [
        ":lib",
        "//oak_client",
        "//oak_functions_abi",
        "@oak_crates_index//:anyhow",
        "@oak_crates_index//:clap",
        "@oak_crates_index//:env_logger",
        "@oak_crates_index//:regex",
        "@oak_crates_index//:tokio",
    ],
)
