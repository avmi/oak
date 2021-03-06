#
# Copyright 2019 The Project Oak Authors
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

load("@rules_cc//cc:defs.bzl", "cc_toolchain", "cc_toolchain_suite")
load(":cc_toolchain_config.bzl", "cc_toolchain_config")
load(":emcc_toolchain_config.bzl", "emcc_toolchain_config")

package(
    default_visibility = ["//visibility:public"],
    licenses = ["notice"],
)

cc_toolchain_config(
    name = "clang_toolchain_config_wasm32",
    cpu = "wasm32",
)

cc_toolchain_config(
    name = "clang_toolchain_config_k8",
    cpu = "k8",
)

cc_toolchain_config(
    name = "clang_toolchain_config_armv8",
    cpu = "armv8",
)

cc_toolchain_suite(
    name = "clang",
    toolchains = {
        "wasm32": ":clang_toolchain_wasm32",
        "k8": ":clang_toolchain_k8",
        "armv8": ":clang_toolchain_armv8",
    },
)

emcc_toolchain_config(
    name = "emscripten_toolchain_config_wasm",
)

cc_toolchain_suite(
    name = "emscripten",
    toolchains = {
        "wasm": ":emscripten_toolchain_wasm",
    },
)

# Include all the files that we install for clang plus the scrips to call into it.
filegroup(
    name = "clang_files",
    srcs = [
        "ar.sh",
        "clang.sh",
        "@clang//:all_files",
    ],
)

# Include all clang files plus the gcc_arm sysroot and libraries.
# We keep this separated to avoid downloading gcc_arm unless needed.
filegroup(
    name = "clang_with_arm_sysroot_files",
    srcs = [
        "ar.sh",
        "clang.sh",
        "@clang//:all_files",
        "@gcc_arm//:all_files",
    ],
)

# Include all the files that we install for emscripten.
filegroup(
    name = "emscripten_files",
    srcs = [
        "emar.sh",
        "emcc.sh",
        "@emsdk//:all",
    ],
)

cc_toolchain(
    name = "clang_toolchain_wasm32",
    all_files = ":clang_files",
    ar_files = ":clang_files",
    compiler_files = ":clang_files",
    dwp_files = ":empty",
    linker_files = ":clang_files",
    objcopy_files = ":empty",
    strip_files = ":empty",
    supports_param_files = 0,
    toolchain_config = ":clang_toolchain_config_wasm32",
    toolchain_identifier = "clang-toolchain_wasm32",
)

cc_toolchain(
    name = "clang_toolchain_k8",
    all_files = ":clang_files",
    ar_files = ":clang_files",
    compiler_files = ":clang_files",
    dwp_files = ":empty",
    linker_files = ":clang_files",
    objcopy_files = ":empty",
    strip_files = ":empty",
    supports_param_files = 0,
    toolchain_config = ":clang_toolchain_config_k8",
    toolchain_identifier = "clang-toolchain_k8",
)

cc_toolchain(
    name = "clang_toolchain_armv8",
    all_files = ":clang_with_arm_sysroot_files",
    ar_files = ":clang_with_arm_sysroot_files",
    compiler_files = ":clang_with_arm_sysroot_files",
    dwp_files = ":empty",
    linker_files = ":clang_with_arm_sysroot_files",
    objcopy_files = ":empty",
    strip_files = ":empty",
    supports_param_files = 0,
    toolchain_config = ":clang_toolchain_config_armv8",
    toolchain_identifier = "clang-toolchain_armv8",
)

cc_toolchain(
    name = "emscripten_toolchain_wasm",
    all_files = ":emscripten_files",
    compiler_files = ":emscripten_files",
    dwp_files = ":empty",
    linker_files = ":emscripten_files",
    objcopy_files = ":empty",
    strip_files = ":empty",
    supports_param_files = 0,
    toolchain_config = ":emscripten_toolchain_config_wasm",
    toolchain_identifier = "emscripten-toolchain_wasm",
)
