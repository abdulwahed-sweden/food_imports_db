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
);

CREATE INDEX IF NOT EXISTS idx_products_barcode ON products(barcode);
CREATE INDEX IF NOT EXISTS idx_products_expiry ON products(expiry_date);
CREATE INDEX IF NOT EXISTS idx_products_stock ON products(stock_quantity);