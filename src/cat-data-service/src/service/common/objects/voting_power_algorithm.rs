//! Defines voting power algorithm.
//!
use poem_openapi::{types::Example, Enum};

/// The Voting Power Algorithm.
#[derive(Enum)]
pub(crate) enum VotingPowerAlgorithm {
    /// Linear Voting Power in Staked ADA, With a minimum limit and maximum relative threshold.
    #[oai(rename = "threshold_staked_ADA")]
    ThresholdStakedADA,
}

impl Example for VotingPowerAlgorithm {
    fn example() -> Self {
        Self::ThresholdStakedADA
    }
}

impl From<event_db::types::event::VotingPowerAlgorithm> for VotingPowerAlgorithm {
    fn from(value: event_db::types::event::VotingPowerAlgorithm) -> Self {
        match value {
            event_db::types::event::VotingPowerAlgorithm::ThresholdStakedADA => {
                VotingPowerAlgorithm::ThresholdStakedADA
            }
        }
    }
}
