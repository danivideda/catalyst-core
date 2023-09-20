"""Vote plan definition."""

from typing import List, Literal
from pydantic import BaseModel

from voting_node.models.token import TokenId


class BlockDate(BaseModel):
    epoch: int
    slot_id: int


class ProposalDef(BaseModel):
    external_id: str
    options: int
    action: Literal["off_chain"]


class VotePlan(BaseModel):
    payload_type: Literal["private", "public"]
    vote_start: BlockDate
    vote_end: BlockDate
    committee_end: BlockDate
    proposals: List[ProposalDef]
    committe_member_public_keys: List[str]
    voting_token: TokenId
