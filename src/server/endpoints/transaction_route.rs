use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use crate::models::transaction::{Transaction, WalletTransactionsRequest};
use crate::db::db_session_manager::DbSessionManager;

/**

    Pattern to initialize the HTTP routes of the server:
     - Defines init_routes function, which is to be passed to the HTTP server.

     - Defines the handler of a POST request to /transactions.
        - Handler collects unique wallet Ids to prepare wallet summary.
        - Calls DB manager to insert wallets and transactions.
        - Calls DB manager to calculate the balance for all wallets in the summary.

     - Defines the handler of a DELETE request to /transactions.
*/

/// Initializes the transaction routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/transactions")
                .route(web::post().to(handle_transactions))
                .route(web::delete().to(delete_transactions))
        )
        .service(web::resource("/hello").route(web::get().to(hello_world)));
}

async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}



async fn handle_transactions(
    request: web::Json<WalletTransactionsRequest>,
) -> impl Responder {
    let db = Arc::new(DbSessionManager::new());

    // Get unique addresses
    let wallet_addresses: Vec<String> = request.transactions.iter()
        .map(|tx| tx.wallet_address.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    match db.insert_transactions(request.transactions.clone()) {
        Ok(_) => {
            let balances: Vec<String> = wallet_addresses.iter()
                .filter_map(|wallet| {
                    db.calculate_balance(wallet)
                        .ok()
                        .flatten()
                        .map(|balance| format!("Balance for {}: {}", wallet, balance))
                })
                .collect();

            HttpResponse::Ok().json(balances)
        },
        Err(err) => {
            eprintln!("Error inserting transactions: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}


async fn delete_transactions(
) -> impl Responder {
    let db = Arc::new(DbSessionManager::new());
    match db.delete_transactions() {
        Ok(_) => HttpResponse::Ok().json("Transactions deleted successfully"),
        Err(err) => {
            eprintln!("Error deleting transactions: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}