use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::{Error, PooledConnection};
use crate::db::schema::transactions;
use crate::models::transaction::{DbTransaction, Transaction};
use diesel::PgConnection;
use diesel::sql_types::{Integer, Text, BigInt};
use diesel::deserialize::QueryableByName;

// types required to work with diesel
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(QueryableByName, Debug)]
struct TableName {
    #[sql_type = "Text"]
    table_name: String,
}

#[derive(QueryableByName, Debug)]
struct DbTransactionRow {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    wallet_address: String,
    #[diesel(sql_type = Text)]
    transaction_type: String,
    #[diesel(sql_type = BigInt)]
    amount: i64,
}

#[derive(QueryableByName, Debug)]
struct DbBalance {
    #[diesel(sql_type = BigInt)]
    balance: i64,
}

pub struct DbSessionManager {
    pool: DbPool,
}

// DB Manager, handles DB connectivity, reads and writes transactions and calculates wallet balances.
impl DbSessionManager {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in a .env file at the root of the project. ");
        println!("Connecting to: {}", database_url);

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .unwrap_or_else(|err| {
                eprintln!("Error creating connection pool: {:?}", err);
                panic!("Failed to create pool.");
            });
        DbSessionManager { pool }
    }

    // Intense DB connection verification because docker was un-behaving
    pub fn verify_connection(&self) -> QueryResult<()> {
        let mut conn = self.get_connection();
        let tables = diesel::sql_query("
        SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public'
    ").load::<TableName>(&mut conn)?;

        println!("Existing tables: {:?}", tables);
        Ok(())
    }
    pub fn verify_connection2(&self) -> QueryResult<()> {
        let mut conn = self.get_connection();

        let initial_transactions = diesel::sql_query("SELECT * FROM transactions")
            .load::<DbTransactionRow>(&mut conn)?;
        println!("Before insert: {:?}", initial_transactions);

        // diesel::sql_query("DELETE FROM transactions;")
        //     .execute(&mut conn)?;

        diesel::sql_query("INSERT INTO transactions (wallet_address, transaction_type, amount) VALUES ('test_wallet', 'Deposit', 100)")
            .execute(&mut conn)?;

        diesel::sql_query("INSERT INTO transactions (wallet_address, transaction_type, amount) VALUES ('test_wallet', 'Withdraw', 50)")
            .execute(&mut conn)?;

        diesel::sql_query("INSERT INTO transactions (wallet_address, transaction_type, amount) VALUES ('test_wallet', 'Deposit', 369)")
            .execute(&mut conn)?;

        let after_transactions = diesel::sql_query("SELECT * FROM transactions")
            .load::<DbTransactionRow>(&mut conn)?;
        println!("After insert: {:?}", after_transactions);

        Ok(())
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get connection")
    }

    // Inserts transactions into the DB. Casts Transaction model from requirements doc  into the Insertable DbTransaction (see src/models/transaction.rs)
    pub fn insert_transactions(&self, transactions: Vec<Transaction>) -> QueryResult<()> {
        let mut conn = self.get_connection();
        let db_transactions: Vec<DbTransaction> = transactions.into_iter().map(DbTransaction::from).collect();

        println!("Attempting to insert transactions: {:?}", db_transactions);

        match diesel::insert_into(transactions::table)
            .values(&db_transactions)
            .execute(&mut conn) {
            Ok(_) => println!("Insert OK"),
            Err(e) => {
                println!("Insert error: {:?}", e);
                return Err(e);
            }
        };

        Ok(())
    }

    /// Cleans the transactions table, great for testing.
    pub fn delete_transactions(&self) -> QueryResult<()> {
        let mut conn = self.get_connection();
        let initial_transactions = diesel::sql_query("SELECT * FROM transactions")
            .load::<DbTransactionRow>(&mut conn)?;
        println!("Before delete: {:?}", initial_transactions);
        let _ = diesel::sql_query("DELETE FROM transactions;").execute(&mut conn)?;

        println!("DELETED ALL VALUES IN TX TABLE ");
        let after_transactions = diesel::sql_query("SELECT * FROM transactions")
            .load::<DbTransactionRow>(&mut conn)?;
        println!("After delete: {:?}", after_transactions);

        Ok(())
    }

    /**
        Calculates the balance for a given wallet based on historic transactions.
            - Finds all transactions for that wallet.
            - Folds the collection and sums or subtracts tx amounts based on tx type.
    */

    pub fn calculate_balance(&self, wallet_address: &str) -> QueryResult<Option<i64>> {
        let mut conn = self.get_connection();

        let transactions = diesel::sql_query(
            "SELECT * FROM transactions WHERE wallet_address = $1"
        )
            .bind::<Text, _>(wallet_address)
            .load::<DbTransactionRow>(&mut conn)?;

        let balance = transactions.iter().fold(0i64, |acc, tx| {
            match tx.clone().transaction_type.as_str() {
                "Deposit" => acc + tx.amount.clone(),
                "Withdraw" => acc - tx.amount.clone(),
                _ => acc
            }
        });

        Ok(Some(balance))
    }
}
