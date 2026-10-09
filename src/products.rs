pub struct Product {
    pub name: String,
}

pub fn create_product(name: String) -> Product {
    Product { name }
}