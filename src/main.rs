use axum::{Router, routing::{get, patch, }};
use std::sync::{Arc, Mutex};

mod routes;
mod models;
use routes::product::{get_all_products, Db,create_product,patch_product, delete_product };

#[tokio::main]
async fn main() {
    println!("Server running...");

    let db: Db = Arc::new(Mutex::new(Vec::new()));

    let router1 = Router::new()
        .route("/", get(get_all_products)
        .post(create_product))
        .route("/product/:id", patch(patch_product)
        .delete(delete_product))
        
        .with_state(db.clone());

    let address = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router1)
        .await
        .unwrap();
}