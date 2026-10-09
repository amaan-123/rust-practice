// // This file is related to earlier_practice: Ctrl+F for:
// // Module 6 — Modules, `use`, `pub` & Project Structure
// // mod products;

pub struct Product {
    pub name: String,
}

pub fn create_product(name: String) -> Product {
    Product { name }
}
