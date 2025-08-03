use sqlx::{SqlitePool, Row};
use crate::models::{Product, AppError};
use crate::code_generator::{generate_internal_code, generate_alternative_code};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, AppError> {
        let pool = SqlitePool::connect("sqlite:products.db").await?;
        
        // Create table if not exists
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS products (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                original_name TEXT NOT NULL,
                imported_name TEXT NOT NULL,
                local_name TEXT,
                barcode TEXT NOT NULL UNIQUE,
                internal_code TEXT NOT NULL UNIQUE,
                alternative_code TEXT NOT NULL UNIQUE,
                brand TEXT NOT NULL,
                category TEXT NOT NULL,
                weight TEXT NOT NULL,
                origin_country TEXT NOT NULL,
                supplier TEXT NOT NULL,
                purchase_price REAL NOT NULL,
                wholesale_price REAL NOT NULL,
                retail_price REAL NOT NULL,
                production_date TEXT NOT NULL,
                expiry_date TEXT NOT NULL,
                batch_id INTEGER NOT NULL,
                stock_quantity INTEGER NOT NULL,
                monthly_sales INTEGER NOT NULL,
                min_threshold INTEGER NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&pool)
        .await?;
        
        Ok(Self { pool })
    }
    
    pub async fn add_product(&self, mut product: Product) -> Result<i64, AppError> {
        let counter = self.count_products().await? + 1;
        
        // Generate codes
        product.internal_code = generate_internal_code(&product);
        product.alternative_code = generate_alternative_code(&product, counter);
        
        let result = sqlx::query(
            r#"
            INSERT INTO products (
                original_name, imported_name, local_name, barcode, internal_code, alternative_code,
                brand, category, weight, origin_country, supplier, purchase_price, wholesale_price,
                retail_price, production_date, expiry_date, batch_id, stock_quantity, monthly_sales,
                min_threshold
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&product.original_name)
        .bind(&product.imported_name)
        .bind(&product.local_name)
        .bind(&product.barcode)
        .bind(&product.internal_code)
        .bind(&product.alternative_code)
        .bind(&product.brand)
        .bind(&product.category)
        .bind(&product.weight)
        .bind(&product.origin_country)
        .bind(&product.supplier)
        .bind(product.purchase_price)
        .bind(product.wholesale_price)
        .bind(product.retail_price)
        .bind(&product.production_date)
        .bind(&product.expiry_date)
        .bind(product.batch_id)
        .bind(product.stock_quantity)
        .bind(product.monthly_sales)
        .bind(product.min_threshold)
        .execute(&self.pool)
        .await?;
        
        Ok(result.last_insert_rowid())
    }
    
    pub async fn get_product_by_barcode(&self, barcode: &str) -> Result<Product, AppError> {
        let row = sqlx::query("SELECT * FROM products WHERE barcode = ?")
            .bind(barcode)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(Product {
            id: Some(row.get("id")),
            original_name: row.get("original_name"),
            imported_name: row.get("imported_name"),
            local_name: row.get("local_name"),
            barcode: row.get("barcode"),
            internal_code: row.get("internal_code"),
            alternative_code: row.get("alternative_code"),
            brand: row.get("brand"),
            category: row.get("category"),
            weight: row.get("weight"),
            origin_country: row.get("origin_country"),
            supplier: row.get("supplier"),
            purchase_price: row.get("purchase_price"),
            wholesale_price: row.get("wholesale_price"),
            retail_price: row.get("retail_price"),
            production_date: row.get("production_date"),
            expiry_date: row.get("expiry_date"),
            batch_id: row.get("batch_id"),
            stock_quantity: row.get("stock_quantity"),
            monthly_sales: row.get("monthly_sales"),
            min_threshold: row.get("min_threshold"),
        })
    }
    
    pub async fn get_all_products(&self) -> Result<Vec<Product>, AppError> {
        let rows = sqlx::query("SELECT * FROM products")
            .fetch_all(&self.pool)
            .await?;
        
        let mut products = Vec::new();
        for row in rows {
            products.push(Product {
                id: Some(row.get("id")),
                original_name: row.get("original_name"),
                imported_name: row.get("imported_name"),
                local_name: row.get("local_name"),
                barcode: row.get("barcode"),
                internal_code: row.get("internal_code"),
                alternative_code: row.get("alternative_code"),
                brand: row.get("brand"),
                category: row.get("category"),
                weight: row.get("weight"),
                origin_country: row.get("origin_country"),
                supplier: row.get("supplier"),
                purchase_price: row.get("purchase_price"),
                wholesale_price: row.get("wholesale_price"),
                retail_price: row.get("retail_price"),
                production_date: row.get("production_date"),
                expiry_date: row.get("expiry_date"),
                batch_id: row.get("batch_id"),
                stock_quantity: row.get("stock_quantity"),
                monthly_sales: row.get("monthly_sales"),
                min_threshold: row.get("min_threshold"),
            });
        }
        
        Ok(products)
    }
    
    async fn count_products(&self) -> Result<i32, AppError> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM products")
            .fetch_one(&self.pool)
            .await?;
        
        Ok(row.get("count"))
    }
}