mod models;
mod code_generator;
mod database;
mod inventory_manager;
mod web;

use models::Product;
use database::Database;
use inventory_manager::InventoryManager;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Food Imports DB")]
#[command(version = "0.1.0")]
#[command(about = "Food Import Products Management System")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the web server
    Web,
    /// Add a new product via CLI
    Add,
    /// Show inventory alerts via CLI
    Alerts,
    /// List all products via CLI
    List,
    /// Generate inventory report via CLI
    Report,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Web => {
            println!("🌐 Starting web interface...");
            web::start_web_server().await?;
        }
        
        Commands::Add => {
            let db = Database::new().await?;
            println!("Adding sample product...");
            
            let product = Product {
                id: None,
                original_name: "حمص حب".to_string(),
                imported_name: "Chickpeas".to_string(),
                local_name: Some("Kikärtor".to_string()),
                barcode: "5281234567890".to_string(),
                internal_code: String::new(), // Will be generated
                alternative_code: String::new(), // Will be generated
                brand: "Shatoura".to_string(),
                category: "Legumes".to_string(),
                weight: "900g".to_string(),
                origin_country: "Lebanon".to_string(),
                supplier: "XYZ Import AB".to_string(),
                purchase_price: 1.50,
                wholesale_price: 2.20,
                retail_price: 3.00,
                production_date: "2025-01-01".to_string(),
                expiry_date: "2025-08-01".to_string(),
                batch_id: 12,
                stock_quantity: 1000,
                monthly_sales: 200,
                min_threshold: 200,
            };
            
            match db.add_product(product).await {
                Ok(id) => println!("Product added with ID: {}", id),
                Err(e) => println!("Error adding product: {}", e),
            }
        }
        
        Commands::Alerts => {
            let db = Database::new().await?;
            println!("Checking inventory alerts...");
            
            match db.get_all_products().await {
                Ok(products) => {
                    let alerts = InventoryManager::check_inventory(&products);
                    
                    if alerts.is_empty() {
                        println!("No alerts found!");
                    } else {
                        println!("Found {} alerts:", alerts.len());
                        for alert in alerts {
                            println!("[{}] {}: {}", alert.severity, alert.alert_type, alert.message);
                        }
                    }
                }
                Err(e) => println!("Error fetching products: {}", e),
            }
        }
        
        Commands::List => {
            let db = Database::new().await?;
            println!("Listing all products...");
            
            match db.get_all_products().await {
                Ok(products) => {
                    if products.is_empty() {
                        println!("No products found!");
                    } else {
                        println!("Found {} products:", products.len());
                        for product in products {
                            println!("- {} ({}) - Stock: {} - Internal Code: {}", 
                                product.imported_name, 
                                product.barcode,
                                product.stock_quantity,
                                product.internal_code
                            );
                        }
                    }
                }
                Err(e) => println!("Error fetching products: {}", e),
            }
        }
        
        Commands::Report => {
            let db = Database::new().await?;
            println!("Generating inventory report...");
            
            match db.get_all_products().await {
                Ok(products) => {
                    let report = InventoryManager::generate_report(&products);
                    println!("{}", report);
                }
                Err(e) => println!("Error generating report: {}", e),
            }
        }
    }

    Ok(())
}