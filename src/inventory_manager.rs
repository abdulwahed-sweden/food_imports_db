use crate::models::Product;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryAlert {
    pub product_name: String,
    pub alert_type: String,
    pub message: String,
    pub severity: String,
}

pub struct InventoryManager;

impl InventoryManager {
    pub fn check_inventory(products: &[Product]) -> Vec<InventoryAlert> {
        let mut alerts = Vec::new();
        
        for product in products {
            // Check expiry (simplified - just check if expiry is in near future)
            if Self::is_expiring_soon(&product.expiry_date) {
                alerts.push(InventoryAlert {
                    product_name: product.imported_name.clone(),
                    alert_type: "Expiry Warning".to_string(),
                    message: format!("Product {} expires soon ({})", product.imported_name, product.expiry_date),
                    severity: "High".to_string(),
                });
            }
            
            // Check low stock
            if product.stock_quantity < product.min_threshold {
                alerts.push(InventoryAlert {
                    product_name: product.imported_name.clone(),
                    alert_type: "Low Stock".to_string(),
                    message: format!("Product {} has low stock: {} units", product.imported_name, product.stock_quantity),
                    severity: if product.stock_quantity == 0 { "Critical".to_string() } else { "Medium".to_string() },
                });
            }
            
            // Check waste risk (simplified calculation)
            if product.monthly_sales > 0 && product.stock_quantity > (product.monthly_sales * 3) {
                alerts.push(InventoryAlert {
                    product_name: product.imported_name.clone(),
                    alert_type: "Waste Risk".to_string(),
                    message: format!("Product {} has excess stock: {} units", product.imported_name, product.stock_quantity),
                    severity: "Medium".to_string(),
                });
            }
        }
        
        alerts
    }
    
    fn is_expiring_soon(expiry_date: &str) -> bool {
        // Simplified check - in real implementation, you'd parse dates properly
        // For now, just check if the year is 2025 (assuming current year is 2025)
        expiry_date.starts_with("2025")
    }
    
    pub fn generate_report(products: &[Product]) -> String {
        let total_products = products.len();
        let total_stock: i32 = products.iter().map(|p| p.stock_quantity).sum();
        let low_stock_count = products.iter().filter(|p| p.stock_quantity < p.min_threshold).count();
        
        format!(
            "Inventory Report:\n\
            - Total Products: {}\n\
            - Total Stock Units: {}\n\
            - Products with Low Stock: {}\n\
            - Average Stock per Product: {:.1}",
            total_products,
            total_stock,
            low_stock_count,
            if total_products > 0 { total_stock as f64 / total_products as f64 } else { 0.0 }
        )
    }
}