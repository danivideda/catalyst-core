"""Initial fragments for genesis.

>>> import json
>>> import yaml
>>> from pydantic import TypeAdapter
>>> from voting_node.models.initial_fragments import InitialFragments, Fund, FundFragment, Token, TokenFragment
>>>
>>>
>>> fund_fragment = Fund(fund=[
...     FundFragment.model_validate({
...         "address": "ca1q4ccde0ax2yv7yx7t7uy6v94yd95pc2u5nrs6wkcn660xp2pj7w2g29q23a",
...         "value": 9508
...     })
... ])
>>> token_fragment = Token(token=TokenFragment.model_validate({
...         "token_id": TokenId(token_id="00000000000000000000000000000000000000000000000000000000.80e38c705071df24015513f5c19d7dc1077cf9943440a8120c5011c64c888cda"),
...         "to": fund_fragment.fund
...     }))
>>> cert_fragment = Cert(cert="signedcert...")
>>>
>>> initials = InitialFragments(initial=[fund_fragment, token_fragment, cert_fragment])
>>> print(yaml.safe_dump(initials.model_dump()))
initial:
- fund:
  - address: ca1q4ccde0ax2yv7yx7t7uy6v94yd95pc2u5nrs6wkcn660xp2pj7w2g29q23a
    value: 9508
- token:
    to:
    - address: ca1q4ccde0ax2yv7yx7t7uy6v94yd95pc2u5nrs6wkcn660xp2pj7w2g29q23a
      value: 9508
    token_id: 00000000000000000000000000000000000000000000000000000000.80e38c705071df24015513f5c19d7dc1077cf9943440a8120c5011c64c888cda
- cert: signedcert...
<BLANKLINE>
"""

from typing import List

from pydantic import BaseModel, ConfigDict
from .token import TokenId


class FundFragment(BaseModel):
    address: str
    value: int


class TokenFragment(BaseModel):
    token_id: TokenId
    to: List[FundFragment]


class Token(BaseModel):
    token: TokenFragment


class Fund(BaseModel):
    fund: List[FundFragment]


class Cert(BaseModel):
    cert: str


class InitialFragments(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True)
    """Pydantic model configuration parameters."""

    initial: List[Fund | Token | Cert]
