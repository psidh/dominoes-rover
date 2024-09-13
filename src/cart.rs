use crate::pizza::Pizza;
use std::io::{self, Write};

pub struct CartItem {
    pub pizza: Pizza,
    pub quantity: i32,
}

pub fn get_quantity() -> i32 {
    loop {
        let mut quantity = String::new();
        print!("Enter the quantity: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut quantity)
            .expect("Failed to read input");

        match quantity.trim().parse::<i32>() {
            Ok(qty) => return qty,
            Err(_) => println!("Invalid quantity. Please enter a valid number."),
        }
    }
}

pub fn add_to_cart(pizza_name: &str, pizzas: &Vec<Pizza>, cart: &mut Vec<CartItem>) {
    for pizza in pizzas {
        if pizza.name.to_lowercase() == pizza_name.to_lowercase() {
            let quantity: i32 = get_quantity();
            cart.push(CartItem {
                pizza: pizza.clone(),
                quantity,
            });
            println!("Added {} to your cart!\n", pizza.name);
            return;
        }
    }
    println!("Pizza not found in the menu.\n");
}

pub fn make_order(user: &crate::user::User, cart: &Vec<CartItem>) {
    println!("--- Order Confirmation ---\n");
    println!(
        "Name: {}\nAddress: {}\nPhone: {}\n",
        user.name, user.address, user.phone
    );
    println!("Your order:");

    if cart.is_empty() {
        println!("Your cart is empty.");
        return;
    }

    for cart_item in cart {
        println!(
            "Name: {}\nCategory: {}\nPrice: {}\nIs Veg: {}\nSize: {:?}\nQuantity: {}\n",
            cart_item.pizza.name,
            cart_item.pizza.category,
            cart_item.pizza.price,
            cart_item.pizza.is_veg,
            cart_item.pizza.size,
            cart_item.quantity
        );
    }
    println!("Thank you for your order!");
}

pub fn get_choice() -> String {
    loop {
        let mut choice = String::new();
        print!("Please enter your choice (1 for pizza, 2 to place order, 3 to exit): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim().to_string();

        if choice == "1" || choice == "2" || choice == "3" {
            return choice;
        } else {
            println!("Invalid choice, please enter 1, 2, or 3.");
        }
    }
}

pub fn get_confirmation(prompt: &str) -> bool {
    loop {
        let mut confirm = String::new();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut confirm)
            .expect("Failed to read input");

        match confirm.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Invalid input. Please enter 'y' for yes or 'n' for no."),
        }
    }
}
