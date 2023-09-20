"""Block0 configuration stored in the 'genesis.yaml' file.

This file is used to generate `block0.bin`.
"""
import calendar
from datetime import datetime
from pathlib import Path
from typing import Literal, List
from aiofile import async_open
from pydantic import BaseModel, Field, field_serializer
import yaml

from voting_node.models.initial_fragments import Cert, Fund, Token

from ..models import LeaderHostInfo


class LinearFee(BaseModel):
    constant: int = 0
    coefficient: int = 0
    certificate: int = 0


class Block0Configuration(BaseModel):
    block_date: datetime = Field(default_factory=lambda: datetime.utcnow())
    discrimination: Literal["production", "test"] = "production"
    block0_consensus: Literal["bft", "genesis"] = "bft"
    consensus_leader_ids: List[LeaderHostInfo] = []
    linear_fees: LinearFee = LinearFee()
    proposal_expiration: int = 100
    slots_per_epoch: int = 900
    slot_duration: int = 4
    kes_update_speed: int = 46800
    consensus_genesis_praos_active_slot_coeff: str = "0.500"
    block_content_max_size: int = 20971520
    epoch_stability_depth: int = 102400
    treasury: int = 1000000
    committees: List[str] = []

    @field_serializer("block_date")
    def serialize_block_date(self, dt: datetime, _info):
        return int(calendar.timegm(dt.utctimetuple()))

    @field_serializer("consensus_leader_ids")
    def serialize_consensus_leader_ids(self, host_info: List[LeaderHostInfo], _info):
        return [host.consensus_leader_id for host in host_info]


class Genesis(BaseModel):
    blockchain_configuration: Block0Configuration = Block0Configuration()
    initial: List[Fund | Token | Cert] = []

    async def save_to_yaml_file(self, path: Path = Path("./genesis.yaml")):
        yaml_str = yaml.safe_dump(self.model_dump())
        afp = async_open(path, "w")
        await afp.write(yaml_str)
        await afp.close()
