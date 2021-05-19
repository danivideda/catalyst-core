use chain_impl_mockchain::fragment::FragmentId;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// This error is reserved for fragments that were rejected by the mempool at the time of sending
/// them to mempool. If a fragment ended up being included to mempool, it will be listed in
/// fragment logs and all further errors would be listed in fragment logs as well.
#[derive(Debug, Serialize, Deserialize)]
pub enum FragmentRejectionReason {
    FragmentAlreadyInLog,
    FragmentInvalid,
    PreviousFragmentInvalid,
    PoolOverflow { pool_number: usize },
}

/// Information about a fragment rejected by the mempool. This is different from being rejected by
/// the ledger during an attempt to apply this fragment.
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct RejectedFragmentInfo {
    #[serde_as(as = "DisplayFromStr")]
    pub id: FragmentId,
    pub reason: FragmentRejectionReason,
}

/// The summary of an attempt to add transactions to mempool for further processing.
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct FragmentsProcessingSummary {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub accepted: Vec<FragmentId>,
    pub rejected: Vec<RejectedFragmentInfo>,
}
