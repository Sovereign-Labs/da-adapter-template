use sovereign_sdk::da::DaVerifier;

use crate::DaLayerSpec;

pub struct Verifier;

impl DaVerifier for Verifier {
    type Spec = DaLayerSpec;

    type Error;

    // Verify that the given list of blob transactions is complete and correct.
    fn verify_relevant_tx_list(
        &self,
        block_header: &<Self::Spec as sovereign_sdk::da::DaSpec>::BlockHeader,
        txs: &[<Self::Spec as sovereign_sdk::da::DaSpec>::BlobTransaction],
        inclusion_proof: <Self::Spec as sovereign_sdk::da::DaSpec>::InclusionMultiProof,
        completeness_proof: <Self::Spec as sovereign_sdk::da::DaSpec>::CompletenessProof,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
