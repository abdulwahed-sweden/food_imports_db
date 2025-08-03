use actix_web::{web, App, HttpServer, Result, HttpResponse, middleware::Logger};
use actix_cors::Cors;
use crate::database::Database;
use crate::models::Product;
use crate::inventory_manager::InventoryManager;
use serde_json::json;
use std::sync::Arc;

pub async fn start_web_server() -> std::io::Result<()> {
    env_logger::init();
    
    let db = Arc::new(Database::new().await.expect("Failed to connect to database"));
    
    println!("🚀 Starting web server at http://localhost:8080");
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .route("/", web::get().to(dashboard))
            .route("/api/products", web::get().to(get_products))
            .route("/api/products", web::post().to(add_product))
            .route("/api/products/{barcode}", web::get().to(get_product))
            .route("/api/alerts", web::get().to(get_alerts))
            .route("/api/stats", web::get().to(get_stats))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn dashboard() -> Result<HttpResponse> {
    let html = include_str!("dashboard.html");
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}

async fn get_products(db: web::Data<Arc<Database>>) -> Result<HttpResponse> {
    match db.get_all_products().await {
        Ok(products) => Ok(HttpResponse::Ok().json(products)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch products: {}", e)
        })))
    }
}

async fn add_product(
    product: web::Json<Product>,
    db: web::Data<Arc<Database>>
) -> Result<HttpResponse> {
    let mut new_product = product.into_inner();
    
    // Set default values
    if new_product.production_date.is_empty() {
        new_product.production_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    }
    if new_product.expiry_date.is_empty() {
        new_product.expiry_date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    }
    
    match db.add_product(new_product).await {
        Ok(id) => Ok(HttpResponse::Ok().json(json!({
            "id": id,
            "message": "Product added successfully"
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": format!("Failed to add product: {}", e)
        })))
    }
}

async fn get_product(
    path: web::Path<String>,
    db: web::Data<Arc<Database>>
) -> Result<HttpResponse> {
    let barcode = path.into_inner();
    
    match db.get_product_by_barcode(&barcode).await {
        Ok(product) => Ok(HttpResponse::Ok().json(product)),
        Err(_) => Ok(HttpResponse::NotFound().json(json!({
            "error": "Product not found"
        })))
    }
}

async fn get_alerts(db: web::Data<Arc<Database>>) -> Result<HttpResponse> {
    match db.get_all_products().await {
        Ok(products) => {
            let alerts = InventoryManager::check_inventory(&products);
            Ok(HttpResponse::Ok().json(alerts))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch alerts: {}", e)
        })))
    }
}

async fn get_stats(db: web::Data<Arc<Database>>) -> Result<HttpResponse> {
    match db.get_all_products().await {
        Ok(products) => {
            let total_products = products.len();
            let total_stock: i32 = products.iter().map(|p| p.stock_quantity).sum();
            let low_stock_count = products.iter().filter(|p| p.stock_quantity < p.min_threshold).count();
            let alerts = InventoryManager::check_inventory(&products);
            let expiring_soon = alerts.iter().filter(|a| a.alert_type == "Expiry Warning").count();
            
            Ok(HttpResponse::Ok().json(json!({
                "total_products": total_products,
                "total_stock": total_stock,
                "low_stock_count": low_stock_count,
                "active_alerts": alerts.len(),
                "expiring_soon": expiring_soon
            })))
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Failed to fetch stats: {}", e)
        })))
    }
}