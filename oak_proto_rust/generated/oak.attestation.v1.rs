/// The Transparent Release attachment for Oak Stage 0. Measurements
/// are produced with:
/// <https://github.com/project-oak/oak/tree/main/snp_measurement>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct FirmwareAttachment {
    /// Maps number of vCPUs to measurement of the modified firmware binary.
    #[prost(btree_map = "int32, message", tag = "1")]
    pub configs: ::prost::alloc::collections::BTreeMap<i32, super::super::HexDigest>,
}
/// The Transparent Release attachment for Oak Containers Linux kernel.
/// Measurements are produced with:
/// <https://github.com/project-oak/oak/tree/main/oak_kernel_measurement>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct KernelAttachment {
    /// Digest of the kernel image part of the bzImage.
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<super::super::HexDigest>,
    /// Digest of the setup data part of the bzImage.
    #[prost(message, optional, tag = "2")]
    pub setup_data: ::core::option::Option<super::super::HexDigest>,
}
/// All the related measurements for Stage 0.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct Stage0Measurements {
    /// Kernel setup data digest.
    #[prost(bytes = "vec", tag = "1")]
    pub setup_data_digest: ::prost::alloc::vec::Vec<u8>,
    /// Kernel digest.
    #[prost(bytes = "vec", tag = "2")]
    pub kernel_measurement: ::prost::alloc::vec::Vec<u8>,
    /// Initial RAM disk digest.
    #[prost(bytes = "vec", tag = "3")]
    pub ram_disk_digest: ::prost::alloc::vec::Vec<u8>,
    /// E820 table digest.
    #[prost(bytes = "vec", tag = "4")]
    pub memory_map_digest: ::prost::alloc::vec::Vec<u8>,
    /// ACPI table generation digest
    #[prost(bytes = "vec", tag = "5")]
    pub acpi_digest: ::prost::alloc::vec::Vec<u8>,
    /// Kernel Command line.
    #[prost(string, tag = "6")]
    pub kernel_cmdline: ::prost::alloc::string::String,
}
/// All the related measurements for Oak Container's Stage 1.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct Stage1Measurements {
    /// System image digest.
    #[prost(message, optional, tag = "1")]
    pub system_image: ::core::option::Option<super::super::RawDigest>,
}
/// All the related measurements for Oak Container's Stage 1.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct OrchestratorMeasurements {
    #[prost(message, optional, tag = "1")]
    pub container_image: ::core::option::Option<super::super::RawDigest>,
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<super::super::RawDigest>,
}
/// Represents an event intended for inclusion in attestation.
/// For example, in an attested measured boot, each event is a reference to the
/// code identity of the boot layer being launched next.
/// An Event message contain what's necessary for an attestation verifier to
/// verify the Event against a Reference Value.
/// TODO: b/333748757 - Make other CB layers use this definition.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct Event {
    /// Represents what is contained in the event. For example, the tag for
    /// TaskConfig for the Layer 2 is "layer2".
    /// TODO: b/333748757 - Consider making the tag a UUID instead of string.
    #[prost(string, tag = "1")]
    pub tag: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<::prost_types::Any>,
}
/// A sequence of Events intended for inclusion in attestation evidence.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct EventLog {
    /// TODO: b/333748757 - Remove 'events' once 'encoded_events' is in google3.
    /// Deprecated: Use encoded_events instead.
    #[deprecated]
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// Holds serialized instances of the the Event message. The serialized form
    /// is used instead of `repeated Event events` as proto serialization is non-
    /// deterministic and attestation evidence contains signatures over the digest
    /// of serialized events.
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub encoded_events: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Endorsement for a specific  \[`Event`\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct EventEndorsement {
    /// Combination of endorsements for a specific event.
    /// There can be multiple endorsements per single event because each event
    /// might contain multiple binary measurements.
    #[prost(message, optional, tag = "1")]
    pub event_endorsement: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct EventEndorsements {
    /// Holds serialized instances of the the \[`EventEndorsement`\] message.
    /// Each item with an index `i` corresponds to a specific event with an index
    /// `i` in the \[`EventLog`\].
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub encoded_event_endorsements: ::prost::alloc::vec::Vec<
        ::prost::alloc::vec::Vec<u8>,
    >,
}
/// Evidence generated by the Layer0.
///
/// Since this layer is the initial layer for our architecture and it is
/// measured during boot, its identity is represented by an attestation report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct RootLayerEvidence {
    /// The platform providing the attestation report.
    #[prost(enumeration = "TeePlatform", tag = "1")]
    pub platform: i32,
    /// TEE-specific attestation report acting as a non-standard certificate for
    /// the Layer0 ECA public key.
    #[prost(bytes = "vec", tag = "2")]
    pub remote_attestation_report: ::prost::alloc::vec::Vec<u8>,
    /// Serialized ECA public key for Layer0 that is signed by the remote
    /// attestation report.
    ///
    /// Represented as a SEC1 encoded point.
    /// <<https://www.secg.org/sec1-v2.pdf#page=16>>
    #[prost(bytes = "vec", tag = "3")]
    pub eca_public_key: ::prost::alloc::vec::Vec<u8>,
}
/// DICE layer evidence containing a certificate signed by the previous layer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct LayerEvidence {
    /// Certificate signing current layer's measurements and the ECA key.
    ///
    /// Represented as a CBOR/COSE/CWT ECA certificate.
    /// <<https://www.rfc-editor.org/rfc/rfc8392.html>>
    #[prost(bytes = "vec", tag = "1")]
    pub eca_certificate: ::prost::alloc::vec::Vec<u8>,
}
/// Keys used by the application to derive encryption session keys and to sign
/// arbitrary data. Each of the certificates contains the final layer's
/// measurement as additional claims.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ApplicationKeys {
    /// Certificate signing the encryption public key.
    ///
    /// Represented as a CBOR/COSE/CWT ECA certificate.
    /// <<https://www.rfc-editor.org/rfc/rfc8392.html>>
    #[prost(bytes = "vec", tag = "1")]
    pub encryption_public_key_certificate: ::prost::alloc::vec::Vec<u8>,
    /// Certificate signing the signing public key.
    ///
    /// Represented as a CBOR/COSE/CWT ECA certificate.
    /// <<https://www.rfc-editor.org/rfc/rfc8392.html>>
    #[prost(bytes = "vec", tag = "2")]
    pub signing_public_key_certificate: ::prost::alloc::vec::Vec<u8>,
    /// Certificate signing the group encryption public key as part of Key
    /// Provisioning.
    ///
    /// Represented as a CBOR/COSE/CWT ECA certificate.
    /// <<https://www.rfc-editor.org/rfc/rfc8392.html>>
    #[prost(bytes = "vec", tag = "3")]
    pub group_encryption_public_key_certificate: ::prost::alloc::vec::Vec<u8>,
    /// Certificate signing the group signing public key as part of Key
    /// Provisioning.
    ///
    /// Represented as a CBOR/COSE/CWT ECA certificate.
    /// <<https://www.rfc-editor.org/rfc/rfc8392.html>>
    #[prost(bytes = "vec", tag = "4")]
    pub group_signing_public_key_certificate: ::prost::alloc::vec::Vec<u8>,
}
/// Attestation Evidence used by the client to the identity of firmware and
/// software running inside a Trusted Execution Environment.
///
/// The name is chosen to match the RATS terminology:
/// <<https://datatracker.ietf.org/doc/html/rfc9334#name-evidence>>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct Evidence {
    /// Layer0 attestation evidence.
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerEvidence>,
    /// Layer1..LayerN-1 attestation evidence.
    #[prost(message, repeated, tag = "2")]
    pub layers: ::prost::alloc::vec::Vec<LayerEvidence>,
    /// Application keys signed by the penultimate layer’s ECA key.
    ///
    /// We are not signing these keys with the last layer's ECA key, because it
    /// is the application layer and the these keys are never shared with it.
    /// The last layer uses an API to the previous layer to:
    /// - Derive session keys from the encryption key
    /// - Sign arbitrary data with the signing key
    #[prost(message, optional, tag = "3")]
    pub application_keys: ::core::option::Option<ApplicationKeys>,
    #[prost(message, optional, tag = "4")]
    pub event_log: ::core::option::Option<EventLog>,
}
/// This proto defines the layered DICE Attestation Evidence.
///
/// DICE provides a mechanism for combining software measurements and
/// corresponding certificates into a chain, where each element (called layer)
/// represents a piece of software loaded into the VMs memory. These layers are
/// loaded sequentially, i.e. the previous layer loads the next layer. The
/// previous layer is also responsible for measuring the next layer, generating
/// a private key and a certificate for it.
///
/// <<https://trustedcomputinggroup.org/wp-content/uploads/TCG_DICE_Attestation_Architecture_r22_02dec2020.pdf>>
/// <<https://trustedcomputinggroup.org/wp-content/uploads/DICE-Layering-Architecture-r19_pub.pdf>>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost_derive::Enumeration)]
#[repr(i32)]
pub enum TeePlatform {
    Unspecified = 0,
    AmdSevSnp = 1,
    IntelTdx = 2,
    None = 3,
}
impl TeePlatform {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TeePlatform::Unspecified => "TEE_PLATFORM_UNSPECIFIED",
            TeePlatform::AmdSevSnp => "AMD_SEV_SNP",
            TeePlatform::IntelTdx => "INTEL_TDX",
            TeePlatform::None => "TEE_PLATFORM_NONE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TEE_PLATFORM_UNSPECIFIED" => Some(Self::Unspecified),
            "AMD_SEV_SNP" => Some(Self::AmdSevSnp),
            "INTEL_TDX" => Some(Self::IntelTdx),
            "TEE_PLATFORM_NONE" => Some(Self::None),
            _ => None,
        }
    }
}
/// Message for passing embedded certificate authority information between
/// layers. Will never appear in the evidence that is sent to the client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct CertificateAuthority {
    /// ECA private key that will be used by a layer to sign a certificate for the
    /// next layer.
    #[prost(bytes = "vec", tag = "1")]
    pub eca_private_key: ::prost::alloc::vec::Vec<u8>,
}
/// Message that is sent between DICE layers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct DiceData {
    #[prost(message, optional, tag = "1")]
    pub evidence: ::core::option::Option<Evidence>,
    #[prost(message, optional, tag = "2")]
    pub certificate_authority: ::core::option::Option<CertificateAuthority>,
}
/// The versions of the components in the AMD SEV-SNP platform Trusted Compute
/// Base (TCB).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct TcbVersion {
    /// The current security version number (SVN) of the secure processor (PSP)
    /// bootloader.
    #[prost(uint32, tag = "1")]
    pub boot_loader: u32,
    /// The current SVN of the PSP operating system.
    #[prost(uint32, tag = "2")]
    pub tee: u32,
    /// The current SVN of the SNP firmware.
    #[prost(uint32, tag = "3")]
    pub snp: u32,
    /// The lowest current patch level of all the CPU cores.
    #[prost(uint32, tag = "4")]
    pub microcode: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct SkipVerification {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct VerifyingKey {
    /// The type of the verifying key.
    #[prost(enumeration = "KeyType", tag = "1")]
    pub r#type: i32,
    /// To distinguish between keys in a key set. The ID is unique within a
    /// key set.
    #[prost(uint32, tag = "2")]
    pub key_id: u32,
    /// The key serialized in raw format. The key type is needed to interpret
    /// the contents.
    #[prost(bytes = "vec", tag = "3")]
    pub raw: ::prost::alloc::vec::Vec<u8>,
}
/// Set of keys currently needed for verification. Will contain one element
/// most of the time, but there may be more during key rotation/revocation.
/// To revoke a key, simply add a new one, allow a grace period, and then
/// don’t pass the old key here.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct VerifyingKeySet {
    #[prost(message, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<VerifyingKey>,
}
/// Reference values that control how the endorsement is verified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct VerifyingKeyReferenceValue {
    /// Nothing set is an error, need to have something set. An empty key set
    /// inside reference values is invalid.
    #[prost(oneof = "verifying_key_reference_value::Type", tags = "1, 2")]
    pub r#type: ::core::option::Option<verifying_key_reference_value::Type>,
}
/// Nested message and enum types in `VerifyingKeyReferenceValue`.
pub mod verifying_key_reference_value {
    /// Nothing set is an error, need to have something set. An empty key set
    /// inside reference values is invalid.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Type {
        /// Deliberately disables verification: it will pass this check rather
        /// than error out.
        #[prost(message, tag = "1")]
        Skip(super::SkipVerification),
        /// Default case: verify with this key set. This also requires that the
        /// instance in question is present and signed.
        #[prost(message, tag = "2")]
        Verify(super::VerifyingKeySet),
    }
}
/// Specifies a list of claim types. Claims are assertions about artifacts made
/// by the endorsing entity. An overview of the claims maintained by Oak can be
/// found at: <https://github.com/project-oak/oak/tree/main/docs/tr/claim>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ClaimReferenceValue {
    #[prost(string, repeated, tag = "1")]
    pub claim_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Verifies the transparency log entry, including signatures and the digest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct EndorsementReferenceValue {
    /// The endorser's public verifying key for signature verification. The
    /// attestation verification requires that all endorsements need to be
    /// signed, therefore this cannot be empty.
    ///
    /// Deprecated - use field `endorser` instead.
    /// TODO: b/379253152 - Cut support of deprecated field and remove it.
    #[deprecated]
    #[prost(bytes = "vec", tag = "1")]
    pub endorser_public_key: ::prost::alloc::vec::Vec<u8>,
    /// Rekor's public verifying key for log entry verification. Needs to be
    /// set when a log entry is present that should be verified. If it is not set,
    /// then log entry verification is skipped.
    ///
    /// Deprecated - use field `rekor` instead.
    /// TODO: b/379253152 - Cut support of deprecated field and remove it.
    #[deprecated]
    #[prost(bytes = "vec", tag = "2")]
    pub rekor_public_key: ::prost::alloc::vec::Vec<u8>,
    /// Verifies the endorsement. Since the signed endorsement is required to
    /// be present, this cannot be skipped as part of this message. (It may
    /// still be possible to skip the endorsement verification entirely, but
    /// elsewhere.
    #[prost(message, optional, tag = "3")]
    pub endorser: ::core::option::Option<VerifyingKeySet>,
    /// Claims that are required to be present in the endorsement.
    #[prost(message, optional, tag = "4")]
    pub required_claims: ::core::option::Option<ClaimReferenceValue>,
    /// Verifies the Rekor log entry, if present and requested.
    #[prost(message, optional, tag = "5")]
    pub rekor: ::core::option::Option<VerifyingKeyReferenceValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct BinaryReferenceValue {
    #[prost(oneof = "binary_reference_value::Type", tags = "1, 2, 3")]
    pub r#type: ::core::option::Option<binary_reference_value::Type>,
}
/// Nested message and enum types in `BinaryReferenceValue`.
pub mod binary_reference_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Type {
        /// Deliberately skips a verification step, rather than failing. For example:
        ///      root_layer { stage0 { skip {} } }
        /// will skip the verification, contrasting
        ///      root_layer { stage0 {} }
        /// which will make the verification fail.
        #[prost(message, tag = "1")]
        Skip(super::SkipVerification),
        /// Verifies the endorsement of the underlying binary.
        #[prost(message, tag = "2")]
        Endorsement(super::EndorsementReferenceValue),
        /// Explicitly verifies digests if the client has them.
        #[prost(message, tag = "3")]
        Digests(super::Digests),
    }
}
/// Similar to the `Digests` message, but allows to specify digests for the
/// split components of the bzImage separately.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct KernelDigests {
    /// Contains admissible digests for the kernel image part of the kernel.
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<Digests>,
    /// Contains admissible digests for the setup data part of the kernel.
    #[prost(message, optional, tag = "3")]
    pub setup_data: ::core::option::Option<Digests>,
}
/// Follows the lines of `BinaryReferenceValue`, but provides a custom proto
/// to facilitate the digest matching.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct KernelBinaryReferenceValue {
    #[prost(oneof = "kernel_binary_reference_value::Type", tags = "1, 2, 3")]
    pub r#type: ::core::option::Option<kernel_binary_reference_value::Type>,
}
/// Nested message and enum types in `KernelBinaryReferenceValue`.
pub mod kernel_binary_reference_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        Skip(super::SkipVerification),
        #[prost(message, tag = "2")]
        Endorsement(super::EndorsementReferenceValue),
        #[prost(message, tag = "3")]
        Digests(super::KernelDigests),
    }
}
/// Reference value for a file including its digests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct FileReferenceValue {
    /// Allowable digests for the file.
    #[prost(message, optional, tag = "1")]
    pub digests: ::core::option::Option<Digests>,
    /// Absolute path to the file in question, or just the file name. Relative
    /// paths are not supported.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
/// Verifies that a particular string is equal to at least one of the specified
/// ones. No checks are performed if this is empty.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct StringReferenceValue {
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct Regex {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
/// A match in at least one value is considered a success. At least one value
/// must be specified, otherwise verification fails.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct StringLiterals {
    #[prost(string, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct RegexReferenceValue {
    #[prost(oneof = "regex_reference_value::Type", tags = "1, 2")]
    pub r#type: ::core::option::Option<regex_reference_value::Type>,
}
/// Nested message and enum types in `RegexReferenceValue`.
pub mod regex_reference_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        Skip(super::SkipVerification),
        #[prost(message, tag = "2")]
        Regex(super::Regex),
    }
}
/// Reference value to match text via endorsement, or directly via constants
/// or a regular expression.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct TextReferenceValue {
    #[prost(oneof = "text_reference_value::Type", tags = "1, 4, 2, 3")]
    pub r#type: ::core::option::Option<text_reference_value::Type>,
}
/// Nested message and enum types in `TextReferenceValue`.
pub mod text_reference_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        Skip(super::SkipVerification),
        #[prost(message, tag = "4")]
        Endorsement(super::EndorsementReferenceValue),
        #[prost(message, tag = "2")]
        Regex(super::Regex),
        #[prost(message, tag = "3")]
        StringLiterals(super::StringLiterals),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct RootLayerReferenceValues {
    /// Switches between AMD SEV-SNP and Intel TDX based on TeePlatform value.
    /// Verification is skipped when not running in a TEE.
    #[prost(message, optional, tag = "1")]
    pub amd_sev: ::core::option::Option<AmdSevReferenceValues>,
    #[prost(message, optional, tag = "2")]
    pub intel_tdx: ::core::option::Option<IntelTdxReferenceValues>,
    /// When insecure is set no verification of the TEE platform is performed. This
    /// can be used when not running in a TEE or when the client is agnostic about
    /// the platform and doesn't care about the hardware verification.
    #[prost(message, optional, tag = "3")]
    pub insecure: ::core::option::Option<InsecureReferenceValues>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct AmdSevReferenceValues {
    /// Minimum accepted versions of all TCB components.
    #[prost(message, optional, tag = "5")]
    pub min_tcb_version: ::core::option::Option<TcbVersion>,
    /// If true, will skip the check that the TEE is not in debug mode.
    #[prost(bool, tag = "3")]
    pub allow_debug: bool,
    /// Verifies the stage0 binary implicitly contained in the root layer.
    #[prost(message, optional, tag = "4")]
    pub stage0: ::core::option::Option<BinaryReferenceValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct IntelTdxReferenceValues {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct InsecureReferenceValues {}
/// Verifies that the field contains at least one of the given digests.
/// No checks are performed if this is empty. A match in at least one
/// digest is considered a success.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct Digests {
    #[prost(message, repeated, tag = "1")]
    pub digests: ::prost::alloc::vec::Vec<super::super::RawDigest>,
}
/// Reference values of the kernel layer, as measured by stage0.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct KernelLayerReferenceValues {
    /// Verifies the kernel based on endorsement.
    #[prost(message, optional, tag = "1")]
    pub kernel: ::core::option::Option<KernelBinaryReferenceValue>,
    /// Verifies the kernel command line, i.e. the parameters passed to the
    /// kernel on boot.
    #[prost(message, optional, tag = "9")]
    pub kernel_cmd_line_text: ::core::option::Option<TextReferenceValue>,
    /// Fields are deprecated and kept only for backwards compatibility. They are
    /// not being used by the verifier. Remove ASAP.
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub kernel_setup_data: ::core::option::Option<BinaryReferenceValue>,
    #[deprecated]
    #[prost(message, optional, tag = "7")]
    pub kernel_image: ::core::option::Option<BinaryReferenceValue>,
    #[deprecated]
    #[prost(message, optional, tag = "8")]
    pub kernel_cmd_line_regex: ::core::option::Option<RegexReferenceValue>,
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub kernel_cmd_line: ::core::option::Option<BinaryReferenceValue>,
    /// Verifies the stage1 binary if running as Oak Containers.
    #[prost(message, optional, tag = "4")]
    pub init_ram_fs: ::core::option::Option<BinaryReferenceValue>,
    #[prost(message, optional, tag = "5")]
    pub memory_map: ::core::option::Option<BinaryReferenceValue>,
    #[prost(message, optional, tag = "6")]
    pub acpi: ::core::option::Option<BinaryReferenceValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct SystemLayerReferenceValues {
    /// Verifies the system image binary based on endorsement.
    #[prost(message, optional, tag = "1")]
    pub system_image: ::core::option::Option<BinaryReferenceValue>,
}
/// Represents an application running under Oak Restricted Kernel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ApplicationLayerReferenceValues {
    /// Verifies the application binary based on endorsement.
    #[prost(message, optional, tag = "1")]
    pub binary: ::core::option::Option<BinaryReferenceValue>,
    /// Verifies configuration with respect to the application binary.
    #[prost(message, optional, tag = "2")]
    pub configuration: ::core::option::Option<BinaryReferenceValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ContainerLayerReferenceValues {
    /// Verifies the container binary based on endorsement.
    #[prost(message, optional, tag = "1")]
    pub binary: ::core::option::Option<BinaryReferenceValue>,
    /// Verifies configuration with respect to the container binary.
    #[prost(message, optional, tag = "2")]
    pub configuration: ::core::option::Option<BinaryReferenceValue>,
}
/// Represents digest of an event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct EventReferenceValues {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<BinaryReferenceValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct OakRestrictedKernelReferenceValues {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerReferenceValues>,
    #[prost(message, optional, tag = "2")]
    pub kernel_layer: ::core::option::Option<KernelLayerReferenceValues>,
    #[prost(message, optional, tag = "3")]
    pub application_layer: ::core::option::Option<ApplicationLayerReferenceValues>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct OakContainersReferenceValues {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerReferenceValues>,
    #[prost(message, optional, tag = "2")]
    pub kernel_layer: ::core::option::Option<KernelLayerReferenceValues>,
    #[prost(message, optional, tag = "3")]
    pub system_layer: ::core::option::Option<SystemLayerReferenceValues>,
    #[prost(message, optional, tag = "4")]
    pub container_layer: ::core::option::Option<ContainerLayerReferenceValues>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct CbReferenceValues {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerReferenceValues>,
    #[prost(message, optional, tag = "2")]
    pub kernel_layer: ::core::option::Option<EventReferenceValues>,
    #[prost(message, optional, tag = "3")]
    pub system_layer: ::core::option::Option<EventReferenceValues>,
    #[prost(message, optional, tag = "4")]
    pub application_layer: ::core::option::Option<EventReferenceValues>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ReferenceValues {
    #[prost(oneof = "reference_values::Type", tags = "1, 2, 3")]
    pub r#type: ::core::option::Option<reference_values::Type>,
}
/// Nested message and enum types in `ReferenceValues`.
pub mod reference_values {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        OakRestrictedKernel(super::OakRestrictedKernelReferenceValues),
        #[prost(message, tag = "2")]
        OakContainers(super::OakContainersReferenceValues),
        #[prost(message, tag = "3")]
        Cb(super::CbReferenceValues),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost_derive::Enumeration)]
#[repr(i32)]
pub enum KeyType {
    /// A verifying key without a defined type is invalid.
    Undefined = 0,
    /// An ECDSA key with curve P-256 and SHA2_256 hashing.
    /// An overview of key formats can be found at:
    /// <https://www.iana.org/assignments/cose/cose.xhtml#algorithms>
    EcdsaP256Sha256 = 1,
}
impl KeyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyType::Undefined => "KEY_TYPE_UNDEFINED",
            KeyType::EcdsaP256Sha256 => "KEY_TYPE_ECDSA_P256_SHA256",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KEY_TYPE_UNDEFINED" => Some(Self::Undefined),
            "KEY_TYPE_ECDSA_P256_SHA256" => Some(Self::EcdsaP256Sha256),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct Endorsement {
    /// The format of the serialized endorsement.
    #[prost(enumeration = "endorsement::Format", tag = "1")]
    pub format: i32,
    /// The serialized endorsement, e.g. serialized JSON for an in-toto
    /// statement.
    #[prost(bytes = "vec", tag = "2")]
    pub serialized: ::prost::alloc::vec::Vec<u8>,
    /// Can pass the endorsed subject when needed and when it is sufficiently
    /// small. In most use cases this field will remain empty.
    #[prost(bytes = "vec", tag = "3")]
    pub subject: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Endorsement`.
pub mod endorsement {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost_derive::Enumeration
    )]
    #[repr(i32)]
    pub enum Format {
        /// Undefined and hence invalid format of the endorsement.
        EndorsementFormatUndefined = 0,
        /// Endorsement is a JSON in-toto statement (all variants and versions).
        EndorsementFormatJsonIntoto = 1,
    }
    impl Format {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Format::EndorsementFormatUndefined => "ENDORSEMENT_FORMAT_UNDEFINED",
                Format::EndorsementFormatJsonIntoto => "ENDORSEMENT_FORMAT_JSON_INTOTO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ENDORSEMENT_FORMAT_UNDEFINED" => Some(Self::EndorsementFormatUndefined),
                "ENDORSEMENT_FORMAT_JSON_INTOTO" => {
                    Some(Self::EndorsementFormatJsonIntoto)
                }
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct Signature {
    /// The ID of the key in a key set that was used to generate the
    /// signature.
    #[prost(uint32, tag = "1")]
    pub key_id: u32,
    /// The raw signature. The type and format of the key used to generate it
    /// can be inferred from `key_id`.
    #[prost(bytes = "vec", tag = "2")]
    pub raw: ::prost::alloc::vec::Vec<u8>,
}
/// A signed endorsement which is optionally published.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct SignedEndorsement {
    /// The underlying unsigned endorsement.
    #[prost(message, optional, tag = "1")]
    pub endorsement: ::core::option::Option<Endorsement>,
    /// The signature over `endorsement.serialized`. Unsigned endorsements are
    /// not supported.
    #[prost(message, optional, tag = "2")]
    pub signature: ::core::option::Option<Signature>,
    /// The Rekor log entry about the endorsement or empty if there is no log
    /// entry.
    #[prost(bytes = "vec", tag = "3")]
    pub rekor_log_entry: ::prost::alloc::vec::Vec<u8>,
}
/// A Transparent Release endorsement for a binary which includes the actual
/// endorsement, a signature over it, and optionally a transparency log entry.
/// Don't use this message in new code, use `SignedEndorsement` instead.
/// `SignedEndorsement` supersedes `TransparentReleaseEndorsement`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct TransparentReleaseEndorsement {
    /// JSON string comtaining the endorsement statement for the underlying binary.
    /// The format is described here:
    /// <https://project-oak.github.io/oak/tr/endorsement/v1>
    #[prost(bytes = "vec", tag = "1")]
    pub endorsement: ::prost::alloc::vec::Vec<u8>,
    /// The data hashed as endorsement subject can be inlined here when needed.
    /// Can be the binary or the attachment, depending on the usage specified
    /// in the endorsement.
    #[prost(bytes = "vec", tag = "4")]
    pub subject: ::prost::alloc::vec::Vec<u8>,
    /// The signature for the endorsement.
    #[prost(bytes = "vec", tag = "2")]
    pub endorsement_signature: ::prost::alloc::vec::Vec<u8>,
    /// The log entry as proof of inclusion of the endorsement statement in Rekor.
    #[prost(bytes = "vec", tag = "3")]
    pub rekor_log_entry: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct RootLayerEndorsements {
    /// The serialized TEE certificate(s). The details of the format and how the
    /// certificate(s) are encoded into this byte array are implementation
    /// specific. In case of AMD-SEV-SNP (as described in
    /// <https://www.amd.com/system/files/TechDocs/57230.pdf>) there are three
    /// different certificates packaged in two different files. We only include
    /// the machine-specific VCEK certificate since the AMD Root Key (ARK) and
    /// AMD SEV Key (ASK) are long-lived.
    #[prost(bytes = "vec", tag = "1")]
    pub tee_certificate: ::prost::alloc::vec::Vec<u8>,
    /// Endorsement of the Stage0 binary.
    #[prost(message, optional, tag = "2")]
    pub stage0: ::core::option::Option<TransparentReleaseEndorsement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct KernelLayerEndorsements {
    #[prost(message, optional, tag = "1")]
    pub kernel: ::core::option::Option<TransparentReleaseEndorsement>,
    #[prost(message, optional, tag = "2")]
    pub kernel_cmd_line: ::core::option::Option<TransparentReleaseEndorsement>,
    #[prost(message, optional, tag = "4")]
    pub init_ram_fs: ::core::option::Option<TransparentReleaseEndorsement>,
    #[prost(message, optional, tag = "5")]
    pub memory_map: ::core::option::Option<TransparentReleaseEndorsement>,
    #[prost(message, optional, tag = "6")]
    pub acpi: ::core::option::Option<TransparentReleaseEndorsement>,
    /// Field is deprecated and kept only for backwards compatibility. Remove ASAP.
    #[deprecated]
    #[prost(message, optional, tag = "7")]
    pub kernel_image: ::core::option::Option<TransparentReleaseEndorsement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct SystemLayerEndorsements {
    #[prost(message, optional, tag = "1")]
    pub system_image: ::core::option::Option<TransparentReleaseEndorsement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ApplicationLayerEndorsements {
    #[prost(message, optional, tag = "1")]
    pub binary: ::core::option::Option<TransparentReleaseEndorsement>,
    #[prost(message, optional, tag = "2")]
    pub configuration: ::core::option::Option<TransparentReleaseEndorsement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ContainerLayerEndorsements {
    #[prost(message, optional, tag = "1")]
    pub binary: ::core::option::Option<TransparentReleaseEndorsement>,
    #[prost(message, optional, tag = "2")]
    pub configuration: ::core::option::Option<TransparentReleaseEndorsement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct OakRestrictedKernelEndorsements {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerEndorsements>,
    #[prost(message, optional, tag = "2")]
    pub kernel_layer: ::core::option::Option<KernelLayerEndorsements>,
    #[prost(message, optional, tag = "3")]
    pub application_layer: ::core::option::Option<ApplicationLayerEndorsements>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct OakContainersEndorsements {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerEndorsements>,
    #[prost(message, optional, tag = "2")]
    pub kernel_layer: ::core::option::Option<KernelLayerEndorsements>,
    #[prost(message, optional, tag = "3")]
    pub system_layer: ::core::option::Option<SystemLayerEndorsements>,
    #[prost(message, optional, tag = "4")]
    pub container_layer: ::core::option::Option<ContainerLayerEndorsements>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct CbEndorsements {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerEndorsements>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct AmdSevSnpEndorsement {
    /// The serialized TEE certificate(s). The details of the format and how the
    /// certificate(s) are encoded into this byte array are implementation
    /// specific. In case of AMD-SEV-SNP (as described in
    /// <https://www.amd.com/system/files/TechDocs/57230.pdf>), there are three
    /// different certificates packaged in two different files. We only include
    /// the machine-specific VCEK certificate since the AMD Root Key (ARK) and
    /// AMD SEV Key (ASK) are long-lived.
    #[prost(bytes = "vec", tag = "1")]
    pub tee_certificate: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct FirmwareEndorsement {
    /// Endorsement of the stage0 firmware binary.
    #[prost(message, optional, tag = "1")]
    pub firmware: ::core::option::Option<SignedEndorsement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct KernelEndorsement {
    #[prost(message, optional, tag = "1")]
    pub kernel: ::core::option::Option<SignedEndorsement>,
    #[prost(message, optional, tag = "2")]
    pub kernel_cmd_line: ::core::option::Option<SignedEndorsement>,
    #[prost(message, optional, tag = "3")]
    pub init_ram_fs: ::core::option::Option<SignedEndorsement>,
    #[prost(message, optional, tag = "4")]
    pub memory_map: ::core::option::Option<SignedEndorsement>,
    #[prost(message, optional, tag = "5")]
    pub acpi: ::core::option::Option<SignedEndorsement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct SystemEndorsement {
    #[prost(message, optional, tag = "1")]
    pub system_image: ::core::option::Option<SignedEndorsement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ApplicationEndorsement {
    #[prost(message, optional, tag = "1")]
    pub binary: ::core::option::Option<SignedEndorsement>,
    #[prost(message, optional, tag = "2")]
    pub configuration: ::core::option::Option<SignedEndorsement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ContainerEndorsement {
    #[prost(message, optional, tag = "1")]
    pub binary: ::core::option::Option<SignedEndorsement>,
    #[prost(message, optional, tag = "2")]
    pub configuration: ::core::option::Option<SignedEndorsement>,
}
/// Endorsement contains statements that some entity (e.g. a hardware provider)
/// vouches for the integrity of claims about the TEE or the software running
/// on it.
///
/// The name is chosen to match the RATS terminology:
/// <https://www.rfc-editor.org/rfc/rfc9334.html#name-endorsements>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct Endorsements {
    #[prost(message, optional, tag = "4")]
    pub event_endorsements: ::core::option::Option<EventEndorsements>,
    #[prost(oneof = "endorsements::Type", tags = "1, 2, 3")]
    pub r#type: ::core::option::Option<endorsements::Type>,
}
/// Nested message and enum types in `Endorsements`.
pub mod endorsements {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        OakRestrictedKernel(super::OakRestrictedKernelEndorsements),
        #[prost(message, tag = "2")]
        OakContainers(super::OakContainersEndorsements),
        #[prost(message, tag = "3")]
        Cb(super::CbEndorsements),
    }
}
/// Represents a verification result. Can be extended to return certain
/// measurements and other detail to the client for further processing.
/// Nomenclature follows RFC 9334.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct AttestationResults {
    /// Indicates whether the verification passed and perhaps more.
    #[prost(enumeration = "attestation_results::Status", tag = "1")]
    pub status: i32,
    /// Provides the reason why verification did not pass, on non-success status.
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    /// Contains the verified public key for encryption whenever the status
    /// indicates success. The key is serialized as an X25519 octet string.
    ///
    /// Deprecated: will be replaced by the
    /// `extracted_evidence.encryption_public_key` field. For now both are
    /// populated.
    #[deprecated]
    #[prost(bytes = "vec", tag = "3")]
    pub encryption_public_key: ::prost::alloc::vec::Vec<u8>,
    /// Contains the verified public key for signing whenever the status
    /// indicates success. The key is serialized using the SEC 1
    /// Elliptic-Curve-Point-to-Octet-String conversion.
    ///
    /// Deprecated: will be replaced by the `extracted_evidence.signing_public_key`
    /// field. For now both are populated.
    #[deprecated]
    #[prost(bytes = "vec", tag = "4")]
    pub signing_public_key: ::prost::alloc::vec::Vec<u8>,
    /// Contains the evidence values whenever the status indicates success.
    #[prost(message, optional, tag = "5")]
    pub extracted_evidence: ::core::option::Option<ExtractedEvidence>,
    /// Detailed attestation verification results each event.
    #[prost(message, repeated, tag = "6")]
    pub event_attestation_results: ::prost::alloc::vec::Vec<EventAttestationResults>,
}
/// Nested message and enum types in `AttestationResults`.
pub mod attestation_results {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost_derive::Enumeration
    )]
    #[repr(i32)]
    pub enum Status {
        Unspecified = 0,
        Success = 1,
        GenericFailure = 2,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::Success => "STATUS_SUCCESS",
                Status::GenericFailure => "STATUS_GENERIC_FAILURE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "STATUS_SUCCESS" => Some(Self::Success),
                "STATUS_GENERIC_FAILURE" => Some(Self::GenericFailure),
                _ => None,
            }
        }
    }
}
/// Attestation verification results for an individual event.
/// TODO: b/366419879 - Implement descriptive per-event attestation results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct EventAttestationResults {}
/// Specifies a temporal range of validity for an endorsement.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct Validity {
    /// The time the endorsement first became valid. In milliseconds UTC since
    /// Unix Epoch.
    #[prost(int64, tag = "1")]
    pub not_before: i64,
    /// The time the endorsement was last valid. In milliseconds UTC since
    /// Unix Epoch.
    #[prost(int64, tag = "2")]
    pub not_after: i64,
}
/// Details about the endorsement statement which can be passed across FFI
/// boundaries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct EndorsementDetails {
    /// Digest of the first subject in the endorsement.
    #[prost(message, optional, tag = "1")]
    pub subject_digest: ::core::option::Option<super::super::RawDigest>,
    /// Validity of the verified endorsement.
    #[prost(message, optional, tag = "2")]
    pub validity: ::core::option::Option<Validity>,
}
/// Evidence values extracted from attestation evidence during verification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ExtractedEvidence {
    /// Contains the public key for encryption. The key is serialized as an X25519
    /// octet string.
    #[prost(bytes = "vec", tag = "4")]
    pub encryption_public_key: ::prost::alloc::vec::Vec<u8>,
    /// Contains the public key for signing. The key is serialized using the SEC 1
    /// Elliptic-Curve-Point-to-Octet-String conversion.
    #[prost(bytes = "vec", tag = "5")]
    pub signing_public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "extracted_evidence::EvidenceValues", tags = "1, 2, 3, 7625")]
    pub evidence_values: ::core::option::Option<extracted_evidence::EvidenceValues>,
}
/// Nested message and enum types in `ExtractedEvidence`.
pub mod extracted_evidence {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum EvidenceValues {
        #[prost(message, tag = "1")]
        OakRestrictedKernel(super::OakRestrictedKernelData),
        #[prost(message, tag = "2")]
        OakContainers(super::OakContainersData),
        #[prost(message, tag = "3")]
        Cb(super::CbData),
        #[prost(message, tag = "7625")]
        Standalone(super::OakStandaloneData),
    }
}
/// Values extracted from the root layer evidence.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct RootLayerData {
    #[prost(oneof = "root_layer_data::Report", tags = "1, 2, 3")]
    pub report: ::core::option::Option<root_layer_data::Report>,
}
/// Nested message and enum types in `RootLayerData`.
pub mod root_layer_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Report {
        /// Values extracted from an AMD SEV-SNP attestation report.
        #[prost(message, tag = "1")]
        SevSnp(super::AmdAttestationReport),
        /// Values extracted from an Intel TDX attestation report.
        #[prost(message, tag = "2")]
        Tdx(super::IntelTdxAttestationReport),
        /// Values extracted from a fake report when not running in a TEE.
        #[prost(message, tag = "3")]
        Fake(super::FakeAttestationReport),
    }
}
/// Values extracted from an AMD SEV-SNP attestation report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct AmdAttestationReport {
    /// The custom bytes that were passed to the report when it was requested.
    #[prost(bytes = "vec", tag = "1")]
    pub report_data: ::prost::alloc::vec::Vec<u8>,
    /// This represents the actual, up-to-date TCB version of the currently running
    /// SEV-SNP firmware. It reflects the latest security patches and updates
    /// applied to the firmware.
    #[prost(message, optional, tag = "2")]
    pub current_tcb: ::core::option::Option<TcbVersion>,
    /// This is the TCB version reported in attestation reports. It can be set
    /// independently from the current_tcb by the hypervisor. This allows for a
    /// smoother transition when updating firmware, giving guest owners time to
    /// retrieve new VCEK certificates before switching to a new VCEK derived from
    /// the updated TCB.
    /// See Section 3.4 of the AMD SEV-SNP specification:
    /// <https://www.amd.com/content/dam/amd/en/documents/epyc-technical-docs/specifications/56860.pdf>
    #[prost(message, optional, tag = "7")]
    pub reported_tcb: ::core::option::Option<TcbVersion>,
    /// Whether the VM was booted in debug mode.
    #[prost(bool, tag = "3")]
    pub debug: bool,
    /// The measurement of the initial memory and CPU state of the VM before
    /// startup. This implicitly includes the measurement of the Stage 0 firmware
    /// binary.
    #[prost(bytes = "vec", tag = "4")]
    pub initial_measurement: ::prost::alloc::vec::Vec<u8>,
    /// The hardware ID of the AMD SEV-SNP platform that generated the attestation
    /// report.
    #[prost(bytes = "vec", tag = "5")]
    pub hardware_id: ::prost::alloc::vec::Vec<u8>,
    /// The VM Protection Leve (VMPL) that was active when the attestation report
    /// was generated.
    #[prost(uint32, tag = "6")]
    pub vmpl: u32,
}
/// Values extracted from an Intel TDX attestation report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct IntelTdxAttestationReport {
    /// The custom bytes that were passed to the report when it was requested.
    #[prost(bytes = "vec", tag = "1")]
    pub report_data: ::prost::alloc::vec::Vec<u8>,
}
/// Values extracted from a fake attestation report when not running in a TEE.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct FakeAttestationReport {
    /// The custom bytes that were passed to the report when it was requested.
    #[prost(bytes = "vec", tag = "1")]
    pub report_data: ::prost::alloc::vec::Vec<u8>,
}
/// Values extracted from the the kernel layer evidence, as measured by stage0.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct KernelLayerData {
    /// Measured digests of the image part of the kernel.
    #[prost(message, optional, tag = "1")]
    pub kernel_image: ::core::option::Option<super::super::RawDigest>,
    /// Measured digests of the setup data part of the kernel.
    #[prost(message, optional, tag = "3")]
    pub kernel_setup_data: ::core::option::Option<super::super::RawDigest>,
    /// Measured digests of the command-line that was passed to the kernel
    /// during startup.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub kernel_cmd_line: ::core::option::Option<super::super::RawDigest>,
    /// Command-line that was passed to the kernel during startup. If absent,
    /// verification will only succeed with the corresponding reference value set
    /// to skip (for compatibility with the legacy version of the evidence
    /// producing code). Empty value corresponds to the kernel being run with an
    /// empty command line.
    #[prost(string, optional, tag = "7")]
    pub kernel_raw_cmd_line: ::core::option::Option<::prost::alloc::string::String>,
    /// Measured digests of the initial RAM disk.
    #[prost(message, optional, tag = "4")]
    pub init_ram_fs: ::core::option::Option<super::super::RawDigest>,
    /// Measured digests of the physical memory map.
    #[prost(message, optional, tag = "5")]
    pub memory_map: ::core::option::Option<super::super::RawDigest>,
    /// Measured digests of the commands for building the ACPI tables.
    #[prost(message, optional, tag = "6")]
    pub acpi: ::core::option::Option<super::super::RawDigest>,
}
/// Values extracted from the evidence that represents an application running
/// under the Oak Restricted Kernel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ApplicationLayerData {
    /// Measurement RawDigest of the application binary.
    #[prost(message, optional, tag = "1")]
    pub binary: ::core::option::Option<super::super::RawDigest>,
    /// Measurement RawDigest of the application configuration.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<super::super::RawDigest>,
}
/// Values extracted from the evidence that represents the Oak Containers system
/// image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct SystemLayerData {
    /// Measurement RawDigest of the system image.
    #[prost(message, optional, tag = "1")]
    pub system_image: ::core::option::Option<super::super::RawDigest>,
}
/// Values extracted from the evidence that represents the Container Runtime
/// Bundle used in Oak Containers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ContainerLayerData {
    /// Measurement RawDigest of the container bundle.
    #[prost(message, optional, tag = "1")]
    pub bundle: ::core::option::Option<super::super::RawDigest>,
    /// Measurement RawDigest of the configuration used by the container.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<super::super::RawDigest>,
}
/// Values extracted from the evidence that represents an event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct EventData {
    /// Measurement RawDigest of an event.
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<super::super::RawDigest>,
}
/// Values extracted from the evidence for a restricted kernel application.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct OakRestrictedKernelData {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerData>,
    #[prost(message, optional, tag = "2")]
    pub kernel_layer: ::core::option::Option<KernelLayerData>,
    #[prost(message, optional, tag = "3")]
    pub application_layer: ::core::option::Option<ApplicationLayerData>,
}
/// Values extracted from the evidence for an Oak Containers instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct OakContainersData {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerData>,
    #[prost(message, optional, tag = "2")]
    pub kernel_layer: ::core::option::Option<KernelLayerData>,
    #[prost(message, optional, tag = "3")]
    pub system_layer: ::core::option::Option<SystemLayerData>,
    #[prost(message, optional, tag = "4")]
    pub container_layer: ::core::option::Option<ContainerLayerData>,
}
/// Reserved for future use.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct CbData {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerData>,
    #[prost(message, optional, tag = "2")]
    pub kernel_layer: ::core::option::Option<EventData>,
    #[prost(message, optional, tag = "3")]
    pub system_layer: ::core::option::Option<EventData>,
    #[prost(message, optional, tag = "4")]
    pub application_layer: ::core::option::Option<EventData>,
}
/// Oak Standalone currently skips all attestation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct OakStandaloneData {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct VerificationSkipped {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct RawDigests {
    #[prost(message, repeated, tag = "1")]
    pub digests: ::prost::alloc::vec::Vec<super::super::RawDigest>,
    /// This field is optional, and only used for some optional
    /// optimizations like client-side caching of verified expected values.
    #[prost(message, optional, tag = "2")]
    pub validity: ::core::option::Option<Validity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ExpectedDigests {
    #[prost(oneof = "expected_digests::Type", tags = "1, 2")]
    pub r#type: ::core::option::Option<expected_digests::Type>,
}
/// Nested message and enum types in `ExpectedDigests`.
pub mod expected_digests {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Type {
        /// If the reference value was set to SkipVerification, we represent that
        /// here.
        #[prost(message, tag = "1")]
        Skipped(super::VerificationSkipped),
        /// One or more digests that should be considered a valid match against an
        /// actual value.
        #[prost(message, tag = "2")]
        Digests(super::RawDigests),
    }
}
/// The expected values for kernel image and setup data, computed from previously
/// provided endorsements and reference values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct KernelExpectedValues {
    /// Allowable digests for the image.
    #[prost(message, optional, tag = "1")]
    pub image: ::core::option::Option<ExpectedDigests>,
    /// Allowable digests for the setup data.
    #[prost(message, optional, tag = "2")]
    pub setup_data: ::core::option::Option<ExpectedDigests>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct AmdSevExpectedValues {
    #[prost(message, optional, tag = "1")]
    pub stage0_expected: ::core::option::Option<ExpectedDigests>,
    /// Minimum accepted versions of all TCB components.
    #[prost(message, optional, tag = "2")]
    pub min_tcb_version: ::core::option::Option<TcbVersion>,
    /// If true, will skip the check that the TEE is not in debug mode.
    #[prost(bool, tag = "3")]
    pub allow_debug: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct IntelTdxExpectedValues {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct InsecureExpectedValues {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ExpectedRegex {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ExpectedStringLiterals {
    #[prost(string, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct TextExpectedValue {
    #[prost(oneof = "text_expected_value::Type", tags = "1, 2, 3")]
    pub r#type: ::core::option::Option<text_expected_value::Type>,
}
/// Nested message and enum types in `TextExpectedValue`.
pub mod text_expected_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Type {
        /// If the reference value was set to SkipVerification, we represent that
        /// here.
        #[prost(message, tag = "1")]
        Skipped(super::VerificationSkipped),
        #[prost(message, tag = "2")]
        Regex(super::ExpectedRegex),
        #[prost(message, tag = "3")]
        StringLiterals(super::ExpectedStringLiterals),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct RootLayerExpectedValues {
    /// Switches between AMD SEV-SNP and Intel TDX based on TeePlatform value.
    /// Verification is skipped when not running in a TEE.
    #[prost(message, optional, tag = "1")]
    pub amd_sev: ::core::option::Option<AmdSevExpectedValues>,
    #[prost(message, optional, tag = "2")]
    pub intel_tdx: ::core::option::Option<IntelTdxExpectedValues>,
    /// When insecure is set no verification of the TEE platform is performed. This
    /// can be used when not running in a TEE or when the client is agnostic about
    /// the platform and doesn't care about the hardware verification.
    #[prost(message, optional, tag = "3")]
    pub insecure: ::core::option::Option<InsecureExpectedValues>,
}
/// Reference values of the kernel layer, as measured by stage0.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct KernelLayerExpectedValues {
    /// Verifies the kernel based on endorsement.
    #[prost(message, optional, tag = "1")]
    pub kernel: ::core::option::Option<KernelExpectedValues>,
    /// Verifies the kernel command line, i.e. the parameters passed to the
    /// kernel on boot.
    #[prost(message, optional, tag = "2")]
    pub kernel_cmd_line_text: ::core::option::Option<TextExpectedValue>,
    /// Verifies the stage1 binary if running as Oak Containers.
    #[prost(message, optional, tag = "3")]
    pub init_ram_fs: ::core::option::Option<ExpectedDigests>,
    #[prost(message, optional, tag = "4")]
    pub memory_map: ::core::option::Option<ExpectedDigests>,
    #[prost(message, optional, tag = "5")]
    pub acpi: ::core::option::Option<ExpectedDigests>,
}
/// The expected binary digests for a system layer image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct SystemLayerExpectedValues {
    /// The allowable digest values for a system layer image.
    #[prost(message, optional, tag = "1")]
    pub system_image: ::core::option::Option<ExpectedDigests>,
}
/// The expected bundle and configuration digests for a container layer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ContainerLayerExpectedValues {
    /// The allowable digest values for a container bundle.
    #[prost(message, optional, tag = "1")]
    pub bundle: ::core::option::Option<ExpectedDigests>,
    /// The allowable digest values for a configuration passed into a container.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<ExpectedDigests>,
}
/// The expected binary and configuration digests for an application layer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ApplicationLayerExpectedValues {
    /// The allowable digest values for an application binary.
    #[prost(message, optional, tag = "1")]
    pub binary: ::core::option::Option<ExpectedDigests>,
    /// The allowable digest values for a configuration passed to the application
    /// binary.
    #[prost(message, optional, tag = "2")]
    pub configuration: ::core::option::Option<ExpectedDigests>,
}
/// Represents digest of an event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct EventExpectedValues {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<ExpectedDigests>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct OakRestrictedKernelExpectedValues {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerExpectedValues>,
    #[prost(message, optional, tag = "2")]
    pub kernel_layer: ::core::option::Option<KernelLayerExpectedValues>,
    #[prost(message, optional, tag = "3")]
    pub application_layer: ::core::option::Option<ApplicationLayerExpectedValues>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct OakContainersExpectedValues {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerExpectedValues>,
    #[prost(message, optional, tag = "2")]
    pub kernel_layer: ::core::option::Option<KernelLayerExpectedValues>,
    #[prost(message, optional, tag = "3")]
    pub system_layer: ::core::option::Option<SystemLayerExpectedValues>,
    #[prost(message, optional, tag = "4")]
    pub container_layer: ::core::option::Option<ContainerLayerExpectedValues>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct CbExpectedValues {
    #[prost(message, optional, tag = "1")]
    pub root_layer: ::core::option::Option<RootLayerExpectedValues>,
    #[prost(message, optional, tag = "2")]
    pub kernel_layer: ::core::option::Option<EventExpectedValues>,
    #[prost(message, optional, tag = "3")]
    pub system_layer: ::core::option::Option<EventExpectedValues>,
    #[prost(message, optional, tag = "4")]
    pub application_layer: ::core::option::Option<EventExpectedValues>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost_derive::Message)]
pub struct ExpectedValues {
    #[prost(oneof = "expected_values::Type", tags = "1, 2, 3")]
    pub r#type: ::core::option::Option<expected_values::Type>,
}
/// Nested message and enum types in `ExpectedValues`.
pub mod expected_values {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost_derive::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        OakRestrictedKernel(super::OakRestrictedKernelExpectedValues),
        #[prost(message, tag = "2")]
        OakContainers(super::OakContainersExpectedValues),
        #[prost(message, tag = "3")]
        Cb(super::CbExpectedValues),
    }
}
