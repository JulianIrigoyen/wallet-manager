use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::sql_types::VarChar;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql};
use diesel::Insertable;
use std::io::Write;
use diesel::FromSqlRow;

use crate::db::schema::transactions;

#[derive(Serialize, Deserialize, Debug, Clone, FromSqlRow)]
#[diesel(sql_type = VarChar)]
pub enum TransactionType {
    Deposit,
    Withdraw,
}

impl ToSql<VarChar, diesel::pg::Pg> for TransactionType {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, diesel::pg::Pg>) -> serialize::Result {
        match self {
            TransactionType::Deposit => out.write_all(b"Deposit")?,
            TransactionType::Withdraw => out.write_all(b"Withdraw")?,
        }
        Ok(serialize::IsNull::No)
    }
}

impl FromSql<VarChar, diesel::pg::Pg> for TransactionType {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Deposit" => Ok(TransactionType::Deposit),
            b"Withdraw" => Ok(TransactionType::Withdraw),
            _ => Err("Unexpected value for TransactionType".into()),
        }
    }
}

/// Represents a transaction received from the API
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub wallet_address: String,          // Wallet address
    pub transaction_type: TransactionType,        // "Deposit" or "Withdraw"
    pub amount: i64,                     // Transaction amount
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletTransactionsRequest {
    pub transactions: Vec<Transaction>,
}

#[derive(Insertable, Queryable, Debug, Clone)]
#[diesel(table_name = transactions)]
pub struct DbTransaction {
    pub wallet_address: String,
    pub transaction_type: String,
    pub amount: i64,
}

impl From<Transaction> for DbTransaction {
    fn from(transaction: Transaction) -> Self {
        DbTransaction {
            wallet_address: transaction.wallet_address,
            transaction_type: transaction.transaction_type.to_string(),
            amount: transaction.amount,
        }
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "Deposit"),
            TransactionType::Withdraw => write!(f, "Withdraw"),
        }
    }
}