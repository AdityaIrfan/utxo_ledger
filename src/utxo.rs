use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use bincode;
use bincode::config::standard;
use bincode::{Decode, Encode};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO {
    pub txid: String,
    pub index: u32,
    pub amount: f64,
    pub recipient: String,
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct TransactionInput {
    pub txid: String,
    pub index: u32,
    pub signature: String,
    pub pubkey: String,
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct TransactionOutput {
    pub amount: f64,
    pub recipient: String,
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn hash(&self) -> String {
        let encoded = bincode::encode_to_vec(self, standard()).unwrap();
        let hash = Sha256::digest(&encoded);
        hex::encode(hash)
    }
}

pub struct UTX0Ledger {
    pub utxos: HashMap<String, Vec<UTXO>> // pubkey_hash -> UTX0s
}

impl UTX0Ledger {
    pub fn new() -> Self {
        UTX0Ledger {
            utxos: HashMap::new(),
        }
    }

    pub fn apply_transaction() {
        
    }
}