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
