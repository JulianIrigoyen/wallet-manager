use std::sync::Arc;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use crate::db::db_session_manager::DbSessionManager;

/// REST pattern

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/{wallet_id}/balance").route(web::get().to(get_balance))
        );
}

async fn get_balance(wallet_id: web::Path<String>) -> impl Responder {
    let db = Arc::new(DbSessionManager::new());
    let wallet = wallet_id.into_inner();
    match db.calculate_balance(&wallet) {
        Ok(Some(balance)) => HttpResponse::Ok().json(json!({
           "wallet_id": wallet,
           "balance": balance
       })),
        Ok(None) => HttpResponse::NotFound().json("Wallet not found"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e))
    }
}