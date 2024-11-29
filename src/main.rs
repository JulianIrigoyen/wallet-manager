mod models;
mod server;
mod db;

use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Json;
use actix_web::{App, HttpServer};
use reqwest::{Client, header};
use serde::{Serialize, Deserialize};
use serde_json::{from_value, json, Value};
use rand::Rng;

use crate::db::db_session_manager::DbSessionManager;
use crate::models::transaction::{DbTransaction, Transaction, TransactionType};
use crate::server::http_server::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Isolates db connection, and for this simple use case, also the business logic (keeps track of wallets, transactions and balances)
    let db_manager = DbSessionManager::new();
    // check the connection because its always good to know if we can connect. Ideally, we'd fail fast and fix DB issues to prevent runtime errors
    db_manager.verify_connection()?; // check db connection health and migration status by checking the existing tables.
    // db_manager.verify_connection2()?; // this one is more intricate as it tests inserts as well


    // exposed for demo purposes. Its always nice to have an extendable, easy to use API
    run_server().await.expect("Failed to start server");

    Ok(())
}


/**
Simple but comprehensive test. Needs the DB container to work.
 - Connects to the DB, covering db connectivity.
 - Creates 3 wallets.
 - Creates 3 transactions for each wallet.
 - Inserts, covering the DB insert functionality.
 - Asserts that the balance for each wallet is correct,
    covering the wallet insert, and balance tracking.
*/
#[test]
fn test_calculate_wallet_balance() {
    let db = DbSessionManager::new();
    db.delete_transactions().unwrap();

    let mut rng = rand::thread_rng();
    let wallets = vec!["0x1", "0x2", "0x3"];
    let mut expected_balances = std::collections::HashMap::new();

    // create test txs
    for wallet in &wallets {
        let mut balance = 0;
        for _ in 0..3 {
            let tx_type = if rng.gen_bool(0.5) {
                TransactionType::Deposit
            } else {
                TransactionType::Withdraw
            };

            let amount = rng.gen_range(10..100);
            balance += match tx_type {
                TransactionType::Deposit => amount.clone(),
                TransactionType::Withdraw => -amount.clone(),
            };

            let tx = Transaction {
                wallet_address: wallet.to_string(),
                transaction_type: tx_type,
                amount,
            };

            db.insert_transactions(vec![tx]).unwrap();
        }
        expected_balances.insert(wallet.to_string(), balance);
    }

    // assert
    for wallet in wallets {
        if let Ok(Some(balance)) = db.calculate_balance(wallet) {
            println!("Wallet: {}, Expected: {}, Got: {}",
                     wallet, expected_balances[wallet], balance);
            assert_eq!(balance, expected_balances[wallet]);
        }
    }
}