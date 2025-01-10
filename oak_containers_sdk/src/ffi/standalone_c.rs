//
// Copyright 2024 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! A few bindings to SDK-provided functionality for C++ callers.
//!
//! This is not a comprehensive set of SDK functionality; it's just to bridge
//! any gaps we have between our current C++ and Rust featureset.
use std::os::raw::{c_uchar, c_void};

use oak_containers_sdk::standalone::StandaloneOrchestrator;
use oak_crypto::encryption_key::EncryptionKey;
use prost::Message;

/// C bindings for generating standalone endorsed evidence.
/// Currently only supports the default configuration.
///
/// The provided callback will be called with the serialized EndorsedEvidence
/// proto generated by rust. Within the scope of the callback, you should
/// process the data however you'd like; but do not hold onto it, it will become
/// invalid when the callback scope is exited.
///
/// The callback also receive a caller-provided context object of the callers
/// choosing; this can contain the resources needed to properly handle the data.
///
/// This code is in extreme protoype phase, and will likely change dramatically;
/// do not depend on this for anything non-experimental
///
/// # Safety
///
///   * The semantics of `callback_context` are defined by the provided
///     callback. Ensure that the callback imlementation does not hold onto the
///     memory pointed to by `data` longer than the scope of the callback
///     invocation.
///   * `private_key` is not null.
///   * `public_key` is not null.
#[no_mangle]
pub unsafe extern "C" fn standalone_endorsed_evidence(
    callback_context: *mut c_void,
    private_key: *const Bytes,
    public_key: *const Bytes,
    callback: unsafe extern "C" fn(callback_context: *mut c_void, data: *const Bytes),
) -> bool {
    // Read private key from args.
    // We need to copy it because `deserialize` will zero out the bytes, but we do
    // not want to clear out the C++ caller's private key.
    // This private key isn't really needed at all; so we could refactor this later
    // to avoid passing it at all.
    let mut private_key_bytes =
        std::slice::from_raw_parts((*private_key).data, (*private_key).len).to_vec();
    let private_key = EncryptionKey::deserialize(private_key_bytes.as_mut_slice())
        .expect("Failed ot deserialize private key");

    // Read public key from args.
    let public_key_bytes =
        std::slice::from_raw_parts((*public_key).data, (*public_key).len).to_vec();

    let orchestrator = StandaloneOrchestrator::builder()
        .encryption_key_pair(Some((private_key, public_key_bytes)))
        .build()
        .expect("failed to build standalone orchestrator");

    let endorsed_evidence = orchestrator.get_endorsed_evidence();
    let serialized_endorsed_evidence = Message::encode_to_vec(&endorsed_evidence);

    let ffi_evidence = Bytes {
        data: serialized_endorsed_evidence.as_ptr(),
        len: serialized_endorsed_evidence.len(),
    };
    unsafe {
        callback(callback_context, &ffi_evidence);
    }

    true
}

/// A basic wrapper around C-provided bytes of known length.
#[repr(C)]
pub struct Bytes {
    pub data: *const c_uchar,
    pub len: usize,
}