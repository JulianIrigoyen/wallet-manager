### Requirement:Develop a small Rust application that models a simple cryptocurrency wallet balance manager.


The application will receive a list of wallet transactions (represented by structs or enums) that
include the transaction type (deposit or withdrawal), wallet address, and amount. Your task is to
calculate the total balance for a given wallet.

# This solution Features:
 - HTTP Server that:
   - receive a list of wallet transactions that include the transaction type (deposit or withdrawal), wallet address, and amount and calculates the total balance for a given wallet.
 - Extensible, self explanatory design.
 - Popular libraries like diesel, tokyo, actix-web or serde.
 - Persistence requires using a containerized PostgreSQL database with minimal initial setup. 


# Instructions for Submission + Comments:
● Write your solution in a single Rust file or project.
  - Wrote the solution in a single project, but multiple files.
  
● Include at least one test case to demonstrate the functionality.
  - A comprehensive integration test can be executed with cargo test after bootstrapping the DB.
  - cURL commands for e2e testing of the available features is included below. 

● Comment your code to explain your logic and decisions.
  - The code is heavily commented to comply with this requirement. 

● Submit your code via a GitHub link
  - https://github.com/JulianIrigoyen/wallet-manager

● Ensure your code is clean, maintainable, and follows best practices for Rust
development.


### Running:

- Start docker container with Postgres DB: ``` docker-compose up --build -d```
- Setup DB: ```diesel setup```
- Migrate DB: ```diesel migration run```
  - Diesel uses the connection string for the containerized DB defined in .env.
  - Other diesel configs are defined in diesel.toml

- Build the project: ```cargo build```
- Start the app: ```cargo run```
  - Exposes an HTTP server at ```http://localhost:3693/api
    - POST     /transactions             > inserts a transaction using the provided JSON model.
    - DELETE   /transactions             > deletes all transactions in the database. 
    - GET      /{walletAddress}/balance > get the balance of a wallet. 
  - 

- Test: ```cargo test```


Commands:
- psql -h localhost -U postgres -c ryz 
- 


# Insert transactions
```
curl -X POST http://localhost:3693/api/transactions \
-H "Content-Type: application/json" \
-d '{"transactions":[{"wallet_address":"wallet1","transaction_type":"Deposit","amount":100},{"wallet_address":"wallet1","transaction_type":"Withdraw","amount":30},{"wallet_address":"wallet2","transaction_type":"Deposit","amount":50}]}'
```

# Delete all transactions
```
curl -X DELETE http://localhost:3693/api/transactions
```

# Get balance
```
curl http://localhost:3693/api/wallet1/balance
```