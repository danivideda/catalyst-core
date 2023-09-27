//! Defines voting power settings.
//!
use super::voting_power_algorithm::VotingPowerAlgorithm;
use poem_openapi::{types::Example, Object};
use rust_decimal::prelude::ToPrimitive;

/// The Settings Used to configure the voting power calculation.
#[derive(Object)]
pub struct VotingPowerSettings {
    /// Voting power algorithm.
    alg: VotingPowerAlgorithm,

    /// Minimum staked funds required for a valid voter registration.
    /// This amount is in Whole ADA. If not present, there is no minimum.
    ///
    /// Valid for `alg`:
    /// * `threshold_staked_ADA`
    #[oai(skip_serializing_if_is_none = true, validator(minimum(value = "0")))]
    min_ada: Option<i64>,

    /// Maximum Percentage of total registered voting power allowed for voting power.
    /// For example `1.23` = `1.23%` of total registered staked ADA as maximum voting power.
    /// If not present, there is no maximum percentage.
    ///
    /// Valid for `alg`:
    /// * `threshold_staked_ADA`
    #[oai(
        skip_serializing_if_is_none = true,
        validator(minimum(value = "0"), maximum(value = "100"))
    )]
    max_pct: Option<f64>,
}

impl Example for VotingPowerSettings {
    fn example() -> Self {
        Self {
            alg: VotingPowerAlgorithm::example(),
            min_ada: Some(1),
            max_pct: Some(1.23),
        }
    }
}

impl TryFrom<event_db::types::event::VotingPowerSettings> for VotingPowerSettings {
    type Error = String;
    fn try_from(value: event_db::types::event::VotingPowerSettings) -> Result<Self, Self::Error> {
        let max_pct = if let Some(max_pct) = value.max_pct {
            Some(
                max_pct
                    .to_f64()
                    .ok_or_else(|| format!("cannot convert decimal to f64: {}", max_pct))?,
            )
        } else {
            None
        };
        Ok(Self {
            alg: value.alg.into(),
            min_ada: value.min_ada,
            max_pct,
        })
    }
}
