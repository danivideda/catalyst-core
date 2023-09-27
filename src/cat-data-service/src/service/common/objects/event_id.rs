//! Defines the ID of an Event.
//!
use poem_openapi::{types::Example, NewType};
use serde::Deserialize;

/// The Numeric ID of a Voting Event
#[derive(NewType, Deserialize)]
#[oai(example = true)]
pub(crate) struct EventId(pub i32);

impl Example for EventId {
    fn example() -> Self {
        Self(11)
    }
}

impl From<EventId> for event_db::types::event::EventId {
    fn from(event_id: EventId) -> Self {
        event_db::types::event::EventId(event_id.0)
    }
}

impl From<event_db::types::event::EventId> for EventId {
    fn from(event_id: event_db::types::event::EventId) -> Self {
        Self(event_id.0)
    }
}
