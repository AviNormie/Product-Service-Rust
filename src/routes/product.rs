use axum::{extract::State, Json};
use crate::models::product::{CreateProduct, Product, UpdateProduct};
use std::sync::{Arc, Mutex};
use axum::extract::Path;


// our shared in-memory DB
pub type Db = Arc<Mutex<Vec<Product>>>;

// Get all products
pub async fn get_all_products(State(db): State<Db>) -> Json<Vec<Product>> {
    let products = db.lock().unwrap();
    let products_copy = products.clone();
    Json(products_copy)
}

// create a new product
pub async fn create_product(State(db): State<Db>, Json(payload): Json<CreateProduct> ) -> Json<Product>{
    let new_products = Product{
            id: uuid::Uuid::new_v4(),
            name: payload.name,
            description:payload.description ,
            price: payload.price,
            message:"Created Successfully".to_string(),
    };
     db.lock().unwrap().push(new_products.clone());
    Json(new_products)
}

//update a product

pub async fn patch_product(
    Path(id): Path<uuid::Uuid>,
    State(db): State<Db>,
    Json(payload): Json<UpdateProduct>,
) -> Json<Option<Product>> {
    let mut products = db.lock().unwrap();

    if let Some(product) = products.iter_mut().find(|p| p.id == id) {
        if let Some(name) = payload.name {
            product.name = name;
        }
        if let Some(desc) = payload.description {
            product.description = Some(desc);
        }
        if let Some(price) = payload.price {
            product.price = price;
        }

        return Json(Some(product.clone()));
    }

    Json(None)
    }


//delete a product with id

pub async fn delete_product(Path(id): Path<uuid::Uuid>,State(db): State<Db>,) -> Json<String> {
    // logic here
    return Json("{deletedProduct} deleted successfully".to_string());
}
