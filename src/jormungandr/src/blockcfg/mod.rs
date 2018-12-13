//! This module provides the different abstractions for the different
//! part of the blockchain.
//!
//! It has been split into 3 components:
//!
//! 1. chain: all the components that chains blocks together;
//! 2. ledger: the transaction model of a blockchain;
//! 3. consensus: the consensus model of the blockchain.
//!

use crate::secure;

pub mod chain;
pub mod ledger;
pub mod update;
// TODO: pub mod consensus;

pub trait BlockConfig {
    type Block: chain::Block<Hash = Self::BlockHash>
              + ledger::HasTransaction<Transaction = Self::Transaction>;
    type BlockHash;
    type BlockHeader;
    type Transaction: ledger::Transaction<Id = Self::TransactionId>;
    type TransactionId;
    type GenesisData;

    type Ledger: ledger::Ledger<Transaction = Self::Transaction>
               + update::Update<Block = Self::Block>;


    fn make_block(
        secret_key: &secure::NodeSecret,
        public_key: &secure::NodePublic,
        ledger: &Self::Ledger,
        block_id: <Self::Block as chain::Block>::Id,
        transactions: Vec<Self::Transaction>,
    ) -> Self::Block;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cardano;
impl BlockConfig for Cardano {
    type Block = chain::cardano::Block;
    type BlockHash = chain::cardano::BlockHash;
    type BlockHeader = chain::cardano::Header;
    type Transaction = chain::cardano::Transaction;
    type TransactionId = chain::cardano::TransactionId;
    type GenesisData = chain::cardano::GenesisData;
    type Ledger = cardano::block::verify_chain::ChainState;

    fn make_block(
        secret_key: &secure::NodeSecret,
        public_key: &secure::NodePublic,
        ledger: &Self::Ledger,
        block_id: <Self::Block as chain::Block>::Id,
        transactions: Vec<Self::Transaction>,
    ) -> Self::Block
    {
        use cardano::{block::*};
        use cbor_event::{Value};
        use blockcfg::update::{Update};
        use cardano::hash::Blake2b256;

        let previous_hash = ledger.get_tip();

        match block_id {
            BlockDate::Boundary(_) => unimplemented!(),
            BlockDate::Normal(block_id) => {
        let pm = ledger.protocol_magic;
        let bver = BlockVersion::new(1,0,0);
        let sver = SoftwareVersion::new(env!("CARGO_PKG_NAME"), 1).unwrap();

        let sig = secret_key.sign_block();

        let body = normal::Body {
            tx: normal::TxPayload::new(transactions),
            ssc: normal::SscPayload::fake(),
            delegation: normal::DlgPayload(Value::U64(0)),
            update: update::UpdatePayload {
                proposal: None,
                votes: Vec::new(),
            },
        };
        let extra = Value::U64(0);

        let body_proof = normal::BodyProof::generate_from_body(&body);
        let extra_bytes = cbor!(&extra).unwrap();

        let hdr = normal::BlockHeader {
            protocol_magic: pm,
            previous_header: previous_hash.clone(),
            body_proof: body_proof,
            consensus: normal::Consensus {
                slot_id: block_id,
                leader_key: public_key.block_publickey.clone(),
                chain_difficulty: ChainDifficulty::from(0u64),
                block_signature: sig,
            },
            extra_data: HeaderExtraData {
                block_version: bver,
                software_version: sver,
                attributes: BlockHeaderAttributes(Value::U64(0)),
                extra_data_proof: Blake2b256::new(&extra_bytes),
            },
        };
        let b = normal::Block {
            header: hdr,
            body: body,
            extra: extra,
        };

        Block::MainBlock(b)
            }
        }

    }
}
