use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use bincode;
use bincode::config::standard;
use bincode::{Decode, Encode};
use sha2::{Digest, Sha256};
use crate::customerror::CustomError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO {
    pub txid: String,
    pub index: u32,
    pub amount: u64,
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
    pub amount: u64,
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
    
    pub fn print(&self) {
        println!("{:#?}", self);
    }
}

#[derive(Debug)]
pub struct UTX0Ledger {
    pub utxos: HashMap<String, Vec<UTXO>> // pubkey_hash -> UTX0s
}

impl UTX0Ledger {
    pub fn new() -> Self {
        UTX0Ledger {
            utxos: HashMap::new(),
        }
    }

    pub fn apply_transaction(&mut self, transaction: &Transaction, txid: &str) {
        for (i, output) in transaction.outputs.iter().enumerate() {
            let utxo = UTXO {
                txid: txid.to_string(),
                index: i as u32,
                amount: output.amount,
                recipient: output.recipient.clone(),
            };

            self.utxos.entry(output.recipient.clone()).or_default().push(utxo);
        }

        // remove spent input
        for input in &transaction.inputs {
            if let Some(list) = self.utxos.get_mut(&input.pubkey) {
                list.retain(|utxo| utxo.txid != input.txid || utxo.index != input.index);
            }
        }

    }

    pub fn get_balance(&self, pubkey_hash: &str) -> u64 {
        self.utxos.get(pubkey_hash)
            .map(|utxos| utxos.iter()
                .map(|utxo| utxo.amount as u64).sum())
            .unwrap_or(0)
    }

    pub fn find_spendable_utxos(&self, pubkey_hash: &str, amount: u64) -> Result<Vec<UTXO>, CustomError> {
        let mut selected = Vec::new();
        let mut total = 0;
        if let Some(utxos) = self.utxos.get(pubkey_hash) {
            for utxo in utxos {
                selected.push(utxo.clone());
                total += utxo.amount;
                if total >= amount {
                    return Ok(selected);
                }
            }

            return Err(CustomError::ExceedsBalance);
        }

        Err(CustomError::WalletNotFound)
    }

    pub fn print_by_address(&self, address: &str) {
        if let Some(utxos) = self.utxos.get(address) {
            for utxo in utxos {
                println!("{:#?}", utxo);
            }
        }
    }

    pub fn print(&self) {
        println!("{:#?}", self);
    }

}