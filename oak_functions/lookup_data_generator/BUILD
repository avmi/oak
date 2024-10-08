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
    srcs = [
        "src/data.rs",
        "src/lib.rs",
    ],
    crate_name = "lookup_data_generator",
    deps = [
        "//oak_proto_rust",
        "@oak_crates_index//:anyhow",
        "@oak_crates_index//:bytes",
        "@oak_crates_index//:prost",
        "@oak_crates_index//:rand",
    ],
)

rust_binary(
    name = "lookup_data_generator",
    srcs = ["src/main.rs"],
    deps = [
        ":lib",
        "@oak_crates_index//:anyhow",
        "@oak_crates_index//:clap",
        "@oak_crates_index//:rand",
    ],
)
