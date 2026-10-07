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

// Option<T> means: I either have a value or I don't.
// It has two variants:
// Some(value)
// None
fn find_product(id: i32) -> Option<String> {
    if id == 1 {
        return Some(String::from("Mechanical Keyboard"));
    }
    None
}
fn product_search_result(option: &Option<String>) {
    match option {
        Some(product_name) => {
            println!("Found product: {}", product_name)
        }
        None => {
            println!("Product not found")
        }
    }
}
fn main() {
    product_search_result(&find_product(1));
    product_search_result(&find_product(99));
}
