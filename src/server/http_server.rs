use actix_web::{App, HttpServer, web};
use std::sync::Arc;

use crate::server::endpoints::wallet_route::init_routes as init_wallet_routes;
use crate::server::endpoints::transaction_route::init_routes as init_transaction_routes;

/**
Starts the HTTP server
*/
pub async fn run_server()-> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .configure(init_transaction_routes)
                .configure(init_wallet_routes)
            )
    })
        .bind("127.0.0.1:3693")?
        .run()
        .await
}
