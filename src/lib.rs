#![cfg_attr(not(feature = "native"), no_std)]

use sovereign_sdk::da::DaSpec;
#[cfg(feature = "native")]
mod service;
mod verifier;

pub struct DaLayerSpec;

impl DaSpec for DaLayerSpec {
    type SlotHash;

    type Address;

    type BlockHeader;

    type BlobTransaction;

    type InclusionMultiProof;

    type CompletenessProof;
}
