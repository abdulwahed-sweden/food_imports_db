use crate::models::Product;

pub fn generate_internal_code(product: &Product) -> String {
    let country = extract_country_code(&product.origin_country);
    let category = extract_category_code(&product.category);
    let brand = extract_brand_code(&product.brand);
    let weight = extract_weight_digits(&product.weight);
    let expiry = extract_expiry_code(&product.expiry_date);
    
    format!("{}-{}-{}-{}-{:03}-{}", 
        country, category, brand, weight, product.batch_id, expiry)
}

pub fn generate_alternative_code(product: &Product, counter: i32) -> String {
    let country = extract_country_code(&product.origin_country);
    let category = extract_category_code(&product.category);
    let brand = extract_brand_code(&product.brand);
    let weight = extract_weight_digits(&product.weight);
    
    format!("{}{}{}{}{:02}", 
        country, 
        &category[..2.min(category.len())], 
        &brand[..3.min(brand.len())], 
        format!("{:0>3}", weight), 
        counter)
}

fn extract_country_code(country: &str) -> String {
    match country.to_lowercase().as_str() {
        "lebanon" => "LB".to_string(),
        "turkey" => "TR".to_string(),
        "india" => "IN".to_string(),
        "pakistan" => "PK".to_string(),
        "tunisia" => "TN".to_string(),
        "morocco" => "MA".to_string(),
        "egypt" => "EG".to_string(),
        _ => country[..2.min(country.len())].to_uppercase(),
    }
}

fn extract_category_code(category: &str) -> String {
    match category.to_lowercase().as_str() {
        "legumes" | "beans" => "LEG".to_string(),
        "grains" | "rice" => "GRA".to_string(),
        "oils" => "OIL".to_string(),
        "spices" => "SPI".to_string(),
        "dairy" => "DAI".to_string(),
        _ => category[..3.min(category.len())].to_uppercase(),
    }
}

fn extract_brand_code(brand: &str) -> String {
    brand.replace(' ', "")[..3.min(brand.len())].to_uppercase()
}

fn extract_weight_digits(weight: &str) -> String {
    weight.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn extract_expiry_code(expiry: &str) -> String {
    // Assumes format YYYY-MM-DD, extracts YYYYMM
    if expiry.len() >= 7 {
        format!("{}{}", &expiry[..4], &expiry[5..7])
    } else {
        "202501".to_string()
    }
}