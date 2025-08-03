use reqwest::Client;
use serde_json::Value;
use crate::models::{ProductInput, ModelError};
use thiserror::Error;

#[derive(Debug)]
pub struct DataCollector {
    client: Client,
}

impl DataCollector {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
    
    pub async fn fetch_from_barcode(&self, barcode: &str) -> Result<ProductInput, ModelError> {
        let url = format!("https://world.openfoodfacts.org/api/v0/product/{}.json", barcode);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(ModelError::BarcodeNotFound);
        }
        
        let json: Value = response.json().await?;
        self.parse_openfoodfacts(json)
    }
    
    fn parse_openfoodfacts(&self, json: Value) -> Result<ProductInput, ModelError> {
        let product = json["product"].clone();
        let today = chrono::Utc::now().date_naive();
        
        Ok(ProductInput {
            original_name: product["product_name"].as_str().unwrap_or("").to_string(),
            imported_name: product["product_name_en"]
                .as_str()
                .unwrap_or(product["product_name"].as_str().unwrap_or(""))
                .to_string(),
            local_name: None,
            barcode: product["code"].as_str().unwrap_or("").to_string(),
            brand: product["brands"].as_str().unwrap_or("Unknown").to_string(),
            category: product["categories"].as_str()
                .unwrap_or("Other")
                .split(',')
                .next()
                .unwrap_or("Other")
                .to_string(),
            weight: product["quantity"].as_str().unwrap_or("0g").to_string(),
            origin_country: product["countries"].as_str()
                .unwrap_or("Unknown")
                .split(',')
                .next()
                .unwrap_or("Unknown")
                .to_string(),
            supplier: "Imported".to_string(),
            purchase_price: 0.0,
            wholesale_price: 0.0,
            retail_price: 0.0,
            production_date: today.format("%Y-%m-%d").to_string(),
            expiry_date: today.format("%Y-%m-%d").to_string(),
            batch_id: 1,
            stock_quantity: 0,
            monthly_sales: 0,
            min_threshold: 10,
        })
    }
}