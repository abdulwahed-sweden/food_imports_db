use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: Option<i64>,
    pub original_name: String,
    pub imported_name: String,
    pub local_name: Option<String>,
    pub barcode: String,
    pub internal_code: String,
    pub alternative_code: String,
    pub brand: String,
    pub category: String,
    pub weight: String,
    pub origin_country: String,
    pub supplier: String,
    pub purchase_price: f64,
    pub wholesale_price: f64,
    pub retail_price: f64,
    pub production_date: String,  // Simplified as string
    pub expiry_date: String,      // Simplified as string
    pub batch_id: i32,
    pub stock_quantity: i32,
    pub monthly_sales: i32,
    pub min_threshold: i32,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Product not found")]
    NotFound,
}