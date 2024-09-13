use std::io::{self, Write};
use std::{thread, time};
mod cart;
mod pizza;
mod user;

use crate::cart::{add_to_cart, make_order, CartItem};
use crate::pizza::{display_menu, show_pizza};

fn main() {
    let mut cart: Vec<CartItem> = Vec::new();
    let pizzas = show_pizza();
    for _ in 0..=10 {
        println!();
    }

    let ascii_art = r#"
     _____   ____   __  __  ___ ___    ______     _____  
    |  __ \ |  _ \ |  \/  | | | |  \  | |  _ \  / ____| 
    | |  | || | | || \  / | | | | `  \| | | | || (___   
    | |  | || | | || |\/| | | | | | \ ` | | | | \___ \  
    | |__| || |_| || |  | | | | | |  \  | | | | ____) | 
    |_____/ |____/ |_|  |_| |_| |_|   \_|____/ |_____/ 
                        
 
                                               
      ____ _     ___     ____ ___ ___  ____     ____
     / ___| |   |_ _|   /      |   |  /    \   /    |
    | |   | |    | |   |____   |___| |      |  |____| 
    | |___| |___ | |        |  |   | |      |  |
     \____|_____|___|  \___/  _|_ _|_ \____/  _|_
                                           
        "#;

    println!("{}", ascii_art);

    // Spinner animation function
    fn spinner_animation() {
        let spinner_chars = vec!['|', '/', '-', '\\'];
        let delay = time::Duration::from_millis(100);

        for i in 0..20 {  // Example loop to show spinner for a short time
            print!("\r{}", spinner_chars[i % spinner_chars.len()]);
            io::stdout().flush().unwrap();
            thread::sleep(delay);
        }
    }

    // Use spinner animation while waiting for user input
    let user = user::get_user_details(); // Use function from `user.rs`

    loop {
        let choice = cart::get_choice(); // Use function from `cart.rs`

        match choice.as_str() {
            "1" => {
                display_menu(&pizzas);

                let mut pizza_choice = String::new();
                print!("Enter the name of the pizza you want to add to the cart: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin()
                    .read_line(&mut pizza_choice)
                    .expect("Failed to read input");

                let pizza_choice = pizza_choice.trim();
                // Show spinner while processing
                spinner_animation();
                println!("Adding pizza to cart");
                println!(" - Done!");

                add_to_cart(pizza_choice, &pizzas, &mut cart);
            }
            "2" => {
                if cart::get_confirmation("Do you want to confirm your order? (y/n) ") {
                    print!("Processing your order");
                    spinner_animation();
                    println!(" - Order confirmed!");

                    make_order(&user, &cart);
                } else {
                    println!("Order cancelled");
                }
            }
            "3" => {
                if cart::get_confirmation("Are you sure you want to exit? (y/n) ") {
                    println!("Exiting. Thank you for visiting Domino's CLI shop!");
                    break;
                }
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
