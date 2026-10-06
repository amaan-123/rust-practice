// // Write a function that takes a quantity (i32) and returns whether it's valid for a reservation
// fn main() {
//     let available_inventory = 20;
//     let quantities = [5, 0, -3, 1, 100];
//     for qty in quantities {
//         println!("qty={} valid={}", qty, validate_quantity(qty));
//         if validate_quantity(qty) {
//             println!(
//                 "Can we reserve requested quantity for you: {}",
//                 can_reserve(qty, available_inventory)
//             );
//         }
//     }
// }
// fn can_reserve(requested_quantity: i32, available_inventory: i32) -> bool {
//     requested_quantity <= available_inventory
// }
// fn validate_quantity(qty: i32) -> bool {
//     qty > 0
// }

// // Borrowing: use the value without taking ownership
// // immutable
// fn print_product_name(name: &str) {
//     println!("Product: {}", name);
// }
// fn main() {
//     let product_name = String::from("Mechanical Keyboard");

//     print_product_name(&product_name);

//     println!("Still available: {}", product_name);
// }

// // Mutable borrowing — &mut
// fn add_suffix(name: &mut String) {
//     name.push_str(" Pro");
// }
// fn main() {
//     let mut product_name = String::from("Mechanical Keyboard");

//     add_suffix(&mut product_name);

//     println!("{}", product_name);
// }

// Compiles
// fn main() {
//     let mut product_name = String::from("Mechanical Keyboard");

//     let name1 = &product_name;
//     let name2 = &product_name;

//     println!("{} {}", name1, name2);
// }

// Not compile
// fn main() {
//     let mut product_name = String::from("Mechanical Keyboard");

//     let name1 = &product_name;
//     let name2 = &product_name;

//     let name3 = &mut product_name; //simultaneous mut. & immut.

//     println!("{} {} {}", name1, name2, name3);
// }
// Compiles
// fn main() {
//     let mut name = String::from("Keyboard");

//     let name1 = &name;
//     println!("{}", name1);

//     // Because by the time the mutable borrow is created, the immutable borrow is no longer being used
//     let name2 = &mut name;
//     name2.push_str(" Pro");
// }

// // Another Mutable borrowing e.g., * (dereferencing) used
// // Compiles
// fn add_stock(quantity: &mut i32) {
//     *quantity += 5;
// }
// fn main() {
//     let mut available_quantity = 10;

//     add_stock(&mut available_quantity);

//     println!("Available: {}", available_quantity);
// }

// fn main() {
//     let name = String::from("Keyboard");

//     let name2 = name;

//     // name is invalid
//     // println!("{}", name);
//     // name2 owns the String
//     println!("{}", name2);
// }

// // One Rust surprise: Copy
// // Compiles
// // Simple value types
// //     i32, bool, char, etc.
// //          ↓
// //        Copy

// // Owned heap data
// //     String, Vec<T>, etc.
// //          ↓
// //        Move
// fn main() {
//     let x = 10;
//     let y = x;

//     println!("x = {}", x);
//     println!("y = {}", y);
// }

// // &String vs &str
// // &str means: "I need to read some string data."
// // It doesn't require the caller to specifically have a String

// fn print_product_name(name: &str) {
//     println!("Product: {}", name);
// }
// fn main() {
//     let product_name = String::from("Mechanical Keyboard");

//     print_product_name(&product_name);
//     // &str can also work with a string literal:
//     print_product_name("Literal");
//     println!("Still available: {}", product_name);
// }

// fn read_product_name(product_name: &str) {
//     println!("I have read product name as: {}", product_name)
// }
// fn increase_available_qty(available: &mut i32, increment: i32) {
//     println!("Quantity available before: {}", available);
//     *available += increment;
//     println!("Quantity available after: {}", available);
// }
// fn main() {
//     struct Product {
//         name: String,
//         available_quantity: i32,
//     }
//     let mut product = Product {
//         name: String::from("Mechanical Keyboard"),
//         available_quantity: 90,
//     };
//     read_product_name(&product.name);
//     increase_available_qty(&mut product.available_quantity, 10);
// }

// Convert above code so that Product owns its behavior.
// `impl` gives the type its methods

// fn main() {
//     struct Product {
//         name: String,
//         available_quantity: i32,
//     }
//     impl Product {
//         // think of `self` as the current `Product` object
//         fn read_product_name(&self) {
//             //&self -> I want to inspect this Product.
//             println!("I have read product name as: {}", self.name)
//         }
//         fn increase_available_qty(&mut self, increment: i32) {
//             // &mut self -> I want to modify this Product.
//             println!("Quantity available before: {}", self.available_quantity);
//             self.available_quantity += increment;
//             println!("Quantity available after: {}", self.available_quantity);
//         }
//     }

//     let mut product = Product {
//         name: String::from("Mechanical Keyboard"),
//         available_quantity: 90,
//     };
//     product.read_product_name();
//     product.increase_available_qty(10);
// }

// // Constructors and Self
// struct Product {
//     name: String,
//     available_quantity: i32,
// }
// impl Product {
//     fn new(name: String, quantity: i32) -> Self {
//         Self {
//             name, // Because the parameter has exactly the same name as the struct field:
//             available_quantity: quantity,
//         }
//     }
//     fn read_product_name(&self) {
//         println!("I have read product name as: {}", self.name)
//     }
//     fn increase_available_qty(&mut self, increment: i32) {
//         println!("Quantity available before: {}", self.available_quantity);
//         self.available_quantity += increment;
//         println!("Quantity available after: {}", self.available_quantity);
//     }
// }
// fn main() {
//     let mut product = Product::new(String::from("Mechanical Keyboard"), 90);
//     product.read_product_name();
//     product.increase_available_qty(10);
// }

// struct Product {
//     name: String,
//     available_quantity: i32,
// }
// impl Product {
//     fn new(name: String, quantity: i32) -> Self {
//         Self {
//             name,
//             available_quantity: quantity,
//         }
//     }
//     /// `inspect_product(product: &Product)`
//     /// → only reads
//     fn inspect_product(&self) {
//         println!("I have read product name as: {}", self.name);
//     }
//     /// `modify_product(product: &mut Product)`
//     /// → changes available quantity
//     fn modify_product(&mut self, increment: i32) {
//         println!("Quantity available before: {}", self.available_quantity);
//         self.available_quantity += increment;
//         println!("Quantity available after: {}", self.available_quantity);
//     }
//     /// `consume_product(product: Product)`
//     /// → takes ownership
//     fn consume_product(self) {
//         println!("I have snatcheed ownership of product: {}", self.name);
//     }
// }
// fn main() {
//     let mut product = Product::new(String::from("Mechanical Keyboard"), 90);
//     product.inspect_product();
//     println!("name: {}, qty:{}", product.name, product.available_quantity);
//     product.modify_product(10);
//     println!("name: {}, qty:{}", product.name, product.available_quantity);
//     // product.consume_product();
//     // println!("name: {}, qty:{}", product.name, product.available_quantity);
// }

struct Product {
    name: String,
    available_quantity: i32,
}
fn new(name: String, quantity: i32) -> Product {
    Product {
        name,
        available_quantity: quantity,
    }
}
fn inspect_product(product: &Product) {
    println!("in inspect_product; name as: {}", product.name);
}
fn modify_product(product: &mut Product, increment: i32) {
    println!(
        "in modify_product; pre-increment: {}",
        product.available_quantity
    );
    product.available_quantity += increment;
    println!(
        "in modify_product; post-increment: {}",
        product.available_quantity
    );
}
fn consume_product(product: Product) -> Product {
    println!(
        "in consume_product; ownership has moved here from main for: {}. Returning now.",
        product.name
    );
    product
}
fn main() {
    let mut product = new(String::from("Mechanical Keyboard"), 90);
    inspect_product(&product);
    println!(
        "in main; name: {}, qty:{}",
        product.name, product.available_quantity
    );
    modify_product(&mut product, 10);
    println!(
        "in main; name: {}, qty:{}",
        product.name, product.available_quantity
    );
    product = consume_product(product);
    println!(
        "in main; name: {}, qty:{}",
        product.name, product.available_quantity
    );
}
