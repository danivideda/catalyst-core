//! Defines the event summary.
//!
use super::event_id::EventId;
use chrono::{DateTime, Utc};
use poem_openapi::{types::Example, Object};

/// A Summary of an individual Voting Event.
#[derive(Object)]
#[oai(example = true)]
pub(crate) struct EventSummary {
    /// The Numeric ID of a Voting Event
    id: EventId,

    /// The Name of a Voting Event.
    name: String,

    /// Date-Time when the Voting Event commences.
    #[oai(skip_serializing_if_is_none = true)]
    starts: Option<DateTime<Utc>>,

    /// Date-Time when the Voting Event is expected to finish.
    #[oai(skip_serializing_if_is_none = true)]
    ends: Option<DateTime<Utc>>,

    /// Last time registrations and Voting power were checked.
    /// If not present, no registration or voting power records exist for this event.
    #[oai(skip_serializing_if_is_none = true)]
    reg_checked: Option<DateTime<Utc>>,

    /// True if the event is finished and no changes can be made to it.
    /// Does not Including payment of rewards or funding of projects.
    #[oai(rename = "final")]
    is_final: bool,
}

impl Example for EventSummary {
    fn example() -> Self {
        Self {
            id: EventId::example(),
            name: "Event Name".to_string(),
            starts: None,
            ends: None,
            reg_checked: None,
            is_final: false,
        }
    }
}

impl From<event_db::types::event::EventSummary> for EventSummary {
    fn from(value: event_db::types::event::EventSummary) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            starts: value.starts,
            ends: value.ends,
            reg_checked: value.reg_checked,
            is_final: value.is_final,
        }
    }
}
