-- Your SQL goes here
 CREATE TABLE IF NOT EXISTS transactions (
     id SERIAL PRIMARY KEY,
     wallet_id INT NOT NULL REFERENCES wallets(id) ON DELETE CASCADE,
     transaction_type VARCHAR(10) NOT NULL CHECK (transaction_type IN ('Deposit', 'Withdrawal')),
     amount BIGINT NOT NULL,
     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
 );