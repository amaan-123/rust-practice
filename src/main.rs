// Write a function that takes a quantity (i32) and returns whether it's valid for a reservation
fn main() {
    let available_inventory = 20;
    let quantities = [5, 0, -3, 1, 100];
    for qty in quantities {
        println!("qty={} valid={}", qty, validate_quantity(qty));
        if validate_quantity(qty) {
            println!(
                "Can we reserve requested quantity for you: {}",
                can_reserve(qty, available_inventory)
            );
        }
    }
}

fn can_reserve(requested_quantity: i32, available_inventory: i32) -> bool {
    requested_quantity <= available_inventory
}

fn validate_quantity(qty: i32) -> bool {
    qty > 0
}
