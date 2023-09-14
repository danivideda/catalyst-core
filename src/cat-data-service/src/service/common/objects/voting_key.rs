use poem_openapi::{types::Example, NewType};
use serde::Deserialize;

#[derive(NewType, Deserialize)]
#[oai(example = true)]
pub struct VotingKey(pub String);

impl Example for VotingKey {
    fn example() -> Self {
        Self("a6a3c0447aeb9cc54cf6422ba32b294e5e1c3ef6d782f2acff4a70694c4d1663".into())
    }
}
