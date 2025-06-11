use ed25519_dalek::Signer;
use crate::customerror::CustomError;
use crate::utxo::{Transaction, TransactionInput, TransactionOutput, UTX0Ledger};
use crate::wallet::Wallet;

mod wallet;
mod utxo;
mod customerror;

// Coinbase Transaction
pub fn create_coinbase(wallet: &Wallet, amount: u64) -> Transaction {
    let recipient = wallet.address();
    Transaction{
        inputs: vec![],
        outputs: vec![TransactionOutput{
            amount,
            recipient: recipient.clone(),
        }]
    }
}

// Build signed transaction
pub fn create_signed_transaction(sender: &Wallet, receiver_address: &str, amount: u64, ledger: &mut UTX0Ledger) -> Result<Transaction, CustomError> {
    let sender_address = sender.address();
    if sender_address == receiver_address {
        return Err(CustomError::BadRequest);
    }

    match ledger.find_spendable_utxos(&sender_address, amount) {
        Ok(value) => {
            let total: u64 = value.iter().map(|utxo| utxo.amount).sum();
            let mut inputs = Vec::new();
            for utxo in value {
                let data = format!("{}:{}", utxo.txid, utxo.index);
                let signature = sender.signing_key.sign(data.as_bytes());
                inputs.push(TransactionInput{
                    txid: utxo.txid.clone(),
                    index: utxo.index,
                    signature: hex::encode(signature.to_bytes()),
                    pubkey: sender_address.clone()
                });
            }

            let mut outputs = vec![TransactionOutput{
                amount,
                recipient: sender_address.clone(),
            }];

            if total > amount {
                outputs.push(TransactionOutput{
                    amount: total - amount,
                    recipient: receiver_address.to_string(),
                });
            };

            Ok(Transaction{
                inputs,
                outputs
            })
        },
        Err(e) => Err(e)
    }
}

pub fn simulate() {
    let mut ledger = UTX0Ledger::new();
    let alice = Wallet::new();
    let bob = Wallet::new();

    let coinbase = create_coinbase(&alice, 100);
    // coinbase.print();
    let txid = coinbase.hash();
    // println!("hash: {}", txid);
    ledger.apply_transaction(&coinbase, &txid);

    println!("======== BEFORE =========");
    println!("Alice balance: {:.}", ledger.get_balance(&alice.address()));
    ledger.print_by_address(&alice.address());
    println!("Bob balance: {:.}", ledger.get_balance(&bob.address()));
    ledger.print_by_address(&bob.address());
    // ledger.print();

    match create_signed_transaction(&alice, &bob.address(), 23, &mut ledger) {
        Ok(tx) => {
            // tx.print();
            let new_txid = tx.hash();
            // println!("new hash: {}", new_txid);
            ledger.apply_transaction(&tx, &new_txid);

            println!("\n======== AFTER =========");
            println!("Alice balance: {:.}", ledger.get_balance(&alice.address()));
            ledger.print_by_address(&alice.address());
            println!("Bob balance: {:.}", ledger.get_balance(&bob.address()));
            ledger.print_by_address(&bob.address());
            // ledger.print();
        }
        Err(CustomError::WalletNotFound) => println!("[ERROR]: Sender's wallet has not been added to the ledger"),
        Err(CustomError::ExceedsBalance) => println!("[ERROR]: Exceeds balance"),
        Err(CustomError::BadRequest) => println!("[ERROR]: Can not send to the same account as the sender"),
    }
}

fn main() {
    simulate()
}
