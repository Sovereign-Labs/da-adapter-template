use core::{future::Future, pin::Pin};

use sovereign_sdk::services::da::DaService;

use crate::DaLayerSpec;

pub struct DaProvider;

impl DaService for DaProvider {
    type Spec = DaLayerSpec;

    type Future<T> = Pin<Box<dyn Future<Output = Result<T, Self::Error>>>>;

    type FilteredBlock;

    type Error;

    // Make an RPC call to the node to get the finalized block at the given height, if one exists.
    // If no such block exists, block until one does.
    fn get_finalized_at(&self, height: u64) -> Self::Future<Self::FilteredBlock> {
        todo!()
    }

    // Make an RPC call to the node to get the block at the given height
    // If no such block exists, block until one does.
    fn get_block_at(&self, height: u64) -> Self::Future<Self::FilteredBlock> {
        todo!()
    }

    // Extract the blob transactions relevant to a particular rollup from a block.
    // This method is usually (but not always) parameterized by some configuration option,
    // such as the rollup's namespace on Celestia. If configuration is needed, it should be provided
    // to the DaProvider struct through its constructor.
    fn extract_relevant_txs(
        &self,
        block: Self::FilteredBlock,
    ) -> Vec<<Self::Spec as sovereign_sdk::da::DaSpec>::BlobTransaction> {
        todo!()
    }

    // Extract the list blob transactions relevant to a particular rollup from a block, along with inclusion and
    // completeness proofs for that set of transactions. The output of this method will be passed to the verifier.
    //
    // Like extract_relevant_txs, This method is usually (but not always) parameterized by some configuration option,
    // such as the rollup's namespace on Celestia. If configuration is needed, it should be provided
    // to the DaProvider struct through its constructor.
    fn extract_relevant_txs_with_proof(
        &self,
        block: Self::FilteredBlock,
    ) -> (
        Vec<<Self::Spec as sovereign_sdk::da::DaSpec>::BlobTransaction>,
        <Self::Spec as sovereign_sdk::da::DaSpec>::InclusionMultiProof,
        <Self::Spec as sovereign_sdk::da::DaSpec>::CompletenessProof,
    ) {
        todo!()
    }
}
