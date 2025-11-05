// create struct first

use uuid::Uuid;

#[derive(serde::Serialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub message: String
}
#[derive(serde::Deserialize)]
pub struct CreateProduct{
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
}

// pub struct UpdateProduct

#[derive(serde::Deserialize)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
}