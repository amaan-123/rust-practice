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

// struct Product {
//     name: String,
//     available_quantity: i32,
// }
// fn new(name: String, quantity: i32) -> Product {
//     Product {
//         name,
//         available_quantity: quantity,
//     }
// }
// fn inspect_product(product: &Product) {
//     println!("in inspect_product; name as: {}", product.name);
// }
// fn modify_product(product: &mut Product, increment: i32) {
//     println!(
//         "in modify_product; pre-increment: {}",
//         product.available_quantity
//     );
//     product.available_quantity += increment;
//     println!(
//         "in modify_product; post-increment: {}",
//         product.available_quantity
//     );
// }
// fn consume_product(product: Product) -> Product {
//     println!(
//         "in consume_product; ownership has moved here from main for: {}. Returning now.",
//         product.name
//     );
//     product
// }
// fn main() {
//     let mut product = new(String::from("Mechanical Keyboard"), 90);
//     inspect_product(&product);
//     println!(
//         "in main; name: {}, qty:{}",
//         product.name, product.available_quantity
//     );
//     modify_product(&mut product, 10);
//     println!(
//         "in main; name: {}, qty:{}",
//         product.name, product.available_quantity
//     );
//     product = consume_product(product);
//     println!(
//         "in main; name: {}, qty:{}",
//         product.name, product.available_quantity
//     );
// }

// // Enums and matching
// enum ReservationError {
//     ProductNotFound,
//     InvalidQuantity,
//     InsufficientInventory { available: i32, requested: i32 }, // Rust enums can also carry data
// }

// fn describe_error(error: &ReservationError) {
//     match error {
//         &ReservationError::ProductNotFound => {
//             println!("Product was not found");
//         }
//         &ReservationError::InvalidQuantity => {
//             println!("Quantity must be > 0");
//         }
//         &ReservationError::InsufficientInventory {
//             available,
//             requested,
//         } => {
//             println!("Available: {} < Requested: {}", available, requested);
//         }
//     }
// }

// fn main() {
//     // Creating enum values; Notice the syntax `EnumName::Variant``
//     let mut error = ReservationError::ProductNotFound;
//     describe_error(&error);
//     error = ReservationError::InvalidQuantity;
//     describe_error(&error);
//     error = ReservationError::InsufficientInventory {
//         available: 10,
//         requested: 20,
//     };
//     describe_error(&error);
// }

// // Result<T, E>
// // This operation either succeeds with i32 or fails with a String message displayed
// fn divide(a: i32, b: i32) -> Result<i32, String> {
//     if b == 0 {
//         return Err(String::from("Division by 0 not allowed"));
//     } else {
//         return Ok(a / b);
//     }
// }
// fn print_result(result: &Result<i32, String>) {
//     match result {
//         Ok(quotient) => {
//             println!("Result is: {}", quotient)
//         }

//         Err(msg) => {
//             println!("{}", msg)
//         }
//     }
// }
// fn main() {
//     // Result variant: Ok(value)
//     let mut result = divide(10, 2);
//     print_result(&result);
//     // Result variant: Err(error)
//     result = divide(10, 0);
//     print_result(&result);
//     // Result variant: Ok(value)
//     result = divide(20, 4);
//     print_result(&result);
// }

// // Ok(value) & Err(error) exercise with Result<T, E>
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
//     fn try_reserve(&mut self, quantity: i32) -> Result<(), ReservationError> {
//         if quantity <= 0 {
//             return Err(ReservationError::InvalidQuantity);
//         }

//         if quantity > self.available_quantity {
//             return Err(ReservationError::InsufficientInventory {
//                 available: self.available_quantity,
//                 requested: quantity,
//             });
//         }

//         self.available_quantity -= quantity;
//         Ok(())
//     }
// }
// enum ReservationError {
//     InvalidQuantity,
//     InsufficientInventory { available: i32, requested: i32 },
// }
// fn reservation_check(result: &Result<(), ReservationError>) {
//     match result {
//         Ok(()) => {
//             println!("Reservation succeeded")
//         }
//         Err(ReservationError::InvalidQuantity) => {
//             println!("Reservation failed: invalid qty")
//         }
//         Err(ReservationError::InsufficientInventory {
//             available,
//             requested,
//         }) => {
//             println!(
//                 "Reservation failed: Available:{} < Requested: {}",
//                 available, requested
//             )
//         }
//     }
// }
// fn main() {
//     let mut product = Product::new(String::from("Mechanical Keyboard"), 90);
//     let mut reservation_result = product.try_reserve(3);
//     reservation_check(&reservation_result);
//     reservation_result = product.try_reserve(0);
//     reservation_check(&reservation_result);
//     reservation_result = product.try_reserve(100);
//     reservation_check(&reservation_result);
// }

// // ? means roughly:
// // If the operation succeeded, give me its value. If it failed, immediately return that error from my current function.
// fn divide(a: i32, b: i32) -> Result<i32, String> {
//     if b == 0 {
//         return Err(String::from("Division by 0 not allowed"));
//     } else {
//         return Ok(a / b);
//     }
// }
// fn calculate(a: i32, b: i32) -> Result<i32, String> {
//     let division_output = divide(a, b)?;
//     Ok(division_output + 10)
// }
// fn print_result(result: Result<i32, String>) {
//     match result {
//         Ok(value) => {
//             println!("Result is: {}", value)
//         }

//         Err(msg) => {
//             println!("{}", msg)
//         }
//     }
// }
// fn main() {
//     print_result(calculate(20, 2));
//     print_result(calculate(20, 0));
// }

// // Option<T> means: I either have a value or I don't.
// // It has two variants:
// // Some(value)
// // None
// fn find_product(id: i32) -> Option<String> {
//     if id == 1 {
//         return Some(String::from("Mechanical Keyboard"));
//     }
//     None
// }
// fn product_search_result(option: &Option<String>) {
//     match option {
//         Some(product_name) => {
//             println!("Found product: {}", product_name)
//         }
//         None => {
//             println!("Product not found")
//         }
//     }
// }
// fn main() {
//     product_search_result(&find_product(1));
//     product_search_result(&find_product(99));
// }

// // One last Option concept: `if let``
// // You don't always need a full `match``.
// // If you only care about one variant:
// if let Some(product) = find_product(1) {
//     println!("Found: {}", product);
// }
// // This means:
// // If the value is Some, give me the contained value and execute this block.
// // You can also handle None:
// if let Some(product) = find_product(1) {
//     println!("Found: {}", product);
// } else {
//     println!("Not found");
// }

// fn print_product(product: Option<String>) {
//     if let Some(product_name) = product {
//         println!("Product: {}", product_name);
//     }
//     //None handled
// }
// fn main() {
//     print_product(Some(String::from("Laptop")));
//     print_product(None);
// }

// // `unwrap_or``
// // Use it when you want a default value if Option is None.
// fn main() {
//     // //Some
//     // let product = Some(String::from("Laptop"));
//     // let name = product.unwrap_or(String::from("Unknown"));
//     // println!("{}", name); // Laptop

//     //None
//     let product: Option<String> = None;
//     let name = product.unwrap_or(String::from("Unknown"));
//     println!("{}", name); // Unknown
// }

// fn main() {
//     // `map` lets you transform the value inside an `Option` without manually `matching` it.
//     let product = Some(String::from("Laptop")); // Some(String)
//     let length = product.map(|name| name.len()); //map
//     println!("{:?}", length); // Result:Some(6)
//     // If the original value is None:
//     let product: Option<String> = None;
//     let length = product.map(|name| name.len());
//     println!("{:?}", length); // Result:None
// }

// fn get_product_name(product: Option<String>) -> String {
//     product
//         .map(|name| format!("Product: {}", name))
//         .unwrap_or(String::from("Product: Unknown"))
// }
// fn main() {
//     println!("{}", get_product_name(Some(String::from("Laptop"))));
//     println!("{}", get_product_name(None));
// }

// // A Vec<T> is Rust's growable array.
// fn main() {
//     // let mut numbers: Vec<i32> = Vec::new();
//     // numbers.push(10);
//     // numbers.push(20);
//     // numbers.push(30);

//     let numbers = vec![10, 20, 30];
//     println!("{}", numbers[0]);
//     let zeroth = numbers.get(0);
//     println!("{:?}", zeroth);
//     match numbers.get(0) {
//         Some(value) => println!("{}", value),
//         None => println!("No element"),
//     }
// }

// // Iterating with for
// fn main() {
//     let products = vec![
//         String::from("Laptop"),
//         String::from("Phone"),
//         String::from("Tablet"),
//     ];

//     // // moves the elements out of products.
//     // // So you cannot subsequently use products.
//     // for product in products {
//     //     println!("{}", product);
//     // }

//     // to borrow instead, use &products
//     for product in &products {
//         println!("{}", product);
//     }
//     println!("{:?}", products);
// }

// // `.iter()`
// // This is another way of borrowing the elements:
// fn main() {
//     let products = vec![
//         String::from("Laptop"),
//         String::from("Phone"),
//         String::from("Tablet"),
//     ];
//     for product in products.iter() {
//         println!("{}", product);
//     }
// }

// fn main() {
//     let products = vec![
//         String::from("Laptop"),
//         String::from("Phone"),
//         String::from("Tablet"),
//     ];
//     // Print every product using .iter()
//     for product in products.iter() {
//         println!("{}", product)
//     }
//     // Print the number of products
//     println!("{}", products.len())
// }

// // Modifying a Vec
// fn main() {
//     let mut products = vec![String::from("Laptop"), String::from("Phone")];
//     products.push(String::from("Tablet"));

//     let removed = products.pop();
//     match removed {
//         Some(product) => println!("Removed: {}", product),
//         None => println!("Nothing to remove"),
//     }
// }

// // HashMap<K, V>
// use std::collections::HashMap;
// fn main() {
//     let mut products = HashMap::new();
//     products.insert(1, String::from("Laptop"));
//     products.insert(2, String::from("Phone"));

//     //Lookup
//     if let Some(product) = products.get(&1) {
//         println!("{}", product);
//     }
// }

// use std::collections::HashMap;
// fn main() {
//     let mut products: HashMap<i32, String> = HashMap::new();
//     products.insert(1, String::from("Laptop"));
//     products.insert(2, String::from("Phone"));
//     products.insert(3, String::from("Tablet"));

//     lookup_product(&products, 2);
//     lookup_product(&products, 99);
// }
// fn lookup_product(products: &HashMap<i32, String>, key: i32) {
//     if let Some(product) = products.get(&key) {
//         println!("{}", product);
//     } else {
//         println!("Product not found");
//     }
// }

// // Iterator pipeline
// fn main() {
//     let numbers = vec![1, 2, 3, 4, 5, 6];
//     let even_numbers: Vec<i32> = numbers
//         .iter()
//         .filter(|number| **number % 2 == 0)
//         .copied()
//         .collect();
//     // That's a little more syntax than we need right now because .iter() gives references.
//     // Let's use strings instead:
//     let products = vec![
//         String::from("Laptop"),
//         String::from("Phone"),
//         String::from("Tablet"),
//     ];
//     let long_names: Vec<&String> = products
//         .iter()
//         .filter(|product| product.len() > 5)
//         .collect();
// }

// struct Product {
//     name: String,
//     available_quantity: i32,
// }
// fn main() {
//     let products = vec![
//         Product {
//             name: String::from("Laptop"),
//             available_quantity: 5,
//         },
//         Product {
//             name: String::from("Phone"),
//             available_quantity: 0,
//         },
//         Product {
//             name: String::from("Tablet"),
//             available_quantity: 3,
//         },
//     ];

//     let available_products: Vec<&String> = products
//         .iter()
//         .filter(|product| product.available_quantity > 0)
//         .map(|product| &product.name)
//         .collect();
//     println!("{:?}", available_products)
// }

// Trait - rough like C# interface
// trait Printable {
//     fn print(&self);
// }
// struct Product {
//     name: String,
// }
// struct Reservation {
//     id: i32,
// }
// impl Printable for Product {
//     fn print(&self) {
//         println!("Product: {}", self.name);
//     }
// }
// impl Printable for Reservation {
//     fn print(&self) {
//         println!("Reservation: {}", self.id);
//     }
// }

// // Think of `derive` as:
// // Rust, generate the standard implementation of this trait for me.
// #[derive(Debug, Clone)]
// struct Product {
//     name: String,
//     available_quantity: i32,
// }
// fn main() {
//     let product = Product {
//         name: String::from("Keyboard"),
//         available_quantity: 20,
//     };
//     println!("{:?}", product); // Without Debug trait, this wouldn't work.
//     let another_product = product.clone(); // Without Clone trait, this wouldn't work.
//     println!("{:?}", another_product);
// }

// trait Printable {
//     fn print(&self);
// }
// // #[derive(Debug)]
// struct Product {
//     name: String,
//     available_quantity: i32,
// }
// impl Product {
//     fn new(product_name: String, qty: i32) -> Self {
//         Self {
//             name: product_name,
//             available_quantity: qty,
//         }
//     }
// }
// impl Printable for Product {
//     fn print(&self) {
//         println!("{}", self.name)
//         // println!("{:?}", self)
//     }
// }
// fn main() {
//     let product = Product::new(String::from("Keyboard"), 90);
//     product.print();
// }

// // Rust struct <--> JSON
// // The library we'll use is Serde.
// // Serialize is   : Rust → JSON
// // Deserialize is : JSON → Rust
// use serde::{Deserialize, Serialize};
// #[derive(Debug, Serialize, Deserialize)]
// struct Product {
//     name: String,
//     #[serde(rename = "availableQuantity")]
//     available_quantity: i32,
// }
// fn main() {
//     let product = Product {
//         name: String::from("Laptop"),
//         available_quantity: 10,
//     };

//     // Because converting to/from JSON can fail (e.g., malformed JSON, mismatched types), Serde functions return a `Result`.
//     // In the actual service's recoverable request paths, we'll handle `Result` properly, consistent with the assignment requirements.

//     // "serialize"
//     // unwrap() is acceptable as a quick learning/example shortcut.
//     let json = serde_json::to_string(&product).unwrap();
//     println!("{}", json);

//     // "deserialize"
//     let json = r#"{"name":"Phone","availableQuantity":20}"#;
//     let product: Product = serde_json::from_str(json).unwrap();
//     println!("{:?}", product);
// }

// // Module 6 — Modules, `use`, `pub` & Project Structure
// mod products;
// use products::{create_product, Product};
// fn main() {
//     let product: Product = create_product(String::from("Laptop"));
//     println!("{}", product.name);
// }

// Module 7: Error Design
// // Here is a practical example showing how a database unique constraint violation becomes a domain error:
// // 1. Your domain errors
// #[derive(Debug)]
// pub enum ReservationError {
//     DuplicateRequest,
//     DatabaseFailure,
// }

// // 2. A mocked database error (representing sqlx::Error)
// pub enum DbError {
//     UniqueConstraintViolation,
//     ConnectionFailed,
// }

// // 3. The Translation Logic
// // This tells Rust exactly how to convert DbError into ReservationError
// impl From<DbError> for ReservationError {
//     fn from(error: DbError) -> Self {
//         match error {
//             // Map the specific DB constraint error to our domain idempotency error
//             DbError::UniqueConstraintViolation => ReservationError::DuplicateRequest,
//             // Fallback for general database crashes
//             _ => ReservationError::DatabaseFailure,
//         }
//     }
// }

// // 4. The automatic conversion in action
// fn insert_reservation(request_id: &str) -> Result<(), ReservationError> {
//     // Simulating a database call that fails because the request_id already exists
//     let db_result: Result<(), DbError> = Err(DbError::UniqueConstraintViolation);

//     // The '?' operator sees a DbError. It checks if a `From` implementation exists,
//     // automatically translates it into ReservationError::DuplicateRequest,
//     // and returns it up the chain to your HTTP handler.
//     db_result?;

//     Ok(())
// }

// // Module 8: Async + Tokio
// // calling an async Rust function produces a `Future`. It doesn't execute the function to completion by itself.
// // You need an asynchronous runtime, such as Tokio, to execute the future.
// #[tokio::main]
// async fn main() {
//     let product = get_product().await;
//     println!("{}", product);
// }

// async fn get_product() -> String {
//     String::from("Laptop")
// }

// // Module 8: structure concurrent tasks and propagate their errors:
// // 1. Error Propagation with ?
// // This simulates a database transaction that might fail
// async fn process_ticket(ticket_id: i32) -> Result<String, &'static str> {
//     // If some_db_call().await? fails, it immediately returns the error here.
//     if ticket_id % 2 == 0 {
//         Ok(format!("Ticket {} saved", ticket_id))
//     } else {
//         Err("Constraint violation")
//     }
// }

// #[tokio::main]
// async fn main() {
//     // 2. tokio::spawn (Firing concurrent tasks)
//     // We instantly fire off two background tasks. They run simultaneously.
//     let task_1 = tokio::spawn(async move {
//         // The task awaits the DB call and returns the Result
//         process_ticket(1).await
//     });

//     let task_2 = tokio::spawn(async move {
//         process_ticket(2).await
//     });

//     // 3. tokio::join! (Waiting for concurrent execution)
//     // The main thread pauses here until both tasks are completely finished.
//     let (result_1, result_2) = tokio::join!(task_1, task_2);

//     // Because Tokio tasks themselves can crash (panic), the result is wrapped
//     // in a JoinError. We unwrap the task result, then look at our domain Result.
//     println!("Task 1: {:?}", result_1.unwrap()); // Err("Constraint violation")
//     println!("Task 2: {:?}", result_2.unwrap()); // Ok("Ticket 2 saved")
// }

// // *Note on loops: `tokio::join!` requires you to name every single task explicitly. For dynamic concurrency (like firing a loop of 20 identical requests for the assignment test), you would push the `tokio::spawn` handles into a `Vec` and wait for all of them using `futures::future::join_all(handles).await`.*

// Module 10 — Three practical points for Serde + JSON
// 1. Use separate request and response types
// For example: Notice that the server generates the ID; the client doesn't supply it.
use serde::Deserialize;
#[derive(Deserialize)]
struct CreateProductRequest {
    name: String,
    available_quantity: i32,
}

use serde::Serialize;
#[derive(Serialize)]
struct ProductResponse {
    id: i64,
    name: String,
    available_quantity: i32,
}

fn main() {
    // 2. Deserialization can fail
    let json = r#"{"name":"Laptop","available_quantity":"ten"}"#;
    let result: Result<CreateProductRequest, serde_json::Error> = serde_json::from_str(json);
}
