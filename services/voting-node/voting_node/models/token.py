"""Voting Token.

A voting token is to distribute funds for voting keys associated to a given voting group (e.g. "direct", "rep").

>>> from voting_node.models.token import TokenId
>>> # To create a new random token id
>>> token_id = TokenId()
>>> # To create a TokenId from an existing one
>>> TokenId(token_id='00000000000000000000000000000000000000000000000000000000.d8e5bfd1bdfccba68ed2b22c577d4e6126602a241229d4fc6198b7f9c61baf6d').model_dump()
'00000000000000000000000000000000000000000000000000000000.d8e5bfd1bdfccba68ed2b22c577d4e6126602a241229d4fc6198b7f9c61baf6d'
>>> # or,
>>> TokenId.model_validate({'token_id':'00000000000000000000000000000000000000000000000000000000.d8e5bfd1bdfccba68ed2b22c577d4e6126602a241229d4fc6198b7f9c61baf6d'}).model_dump()
'00000000000000000000000000000000000000000000000000000000.d8e5bfd1bdfccba68ed2b22c577d4e6126602a241229d4fc6198b7f9c61baf6d'

"""
import secrets

from pydantic import BaseModel, Field, field_validator, model_serializer


class TokenId(BaseModel):
    """The token id for a voting group."""

    token_id: str = Field(
        default_factory=lambda: f"{bytes(28).hex()}.{secrets.token_bytes(32).hex()}", min_length=121, max_length=121
    )
    """The token id is a string composed of the hex representation for a 28-byte policy hash (defaults to all zeroes),
    and the hex representation for a random 32-byte token name.
    """

    @field_validator("token_id")
    @classmethod
    def token_id_consists_of_policy_hash_and_token_name(cls, v: str) -> str:
        """Validates correct length and expected structure."""
        try:
            (policy_hash, token_name) = v.split(".")
        except:
            raise ValueError("invalid token_id")

        if len(policy_hash) != 56:
            raise ValueError("invalid policy hash length")

        try:
            bytes.fromhex(policy_hash)
        except:
            raise ValueError("invalid policy hash")

        if len(token_name) != 64:
            raise ValueError("invalid token name length")

        try:
            bytes.fromhex(policy_hash)
        except:
            raise ValueError("invalid token name")

        return v

    @model_serializer
    def token_string(self) -> str:
        """Returns the token id string."""
        return self.token_id
