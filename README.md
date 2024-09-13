# 🍕 Domino's CLI Pizza Ordering System

Command-line interface (CLI) pizza ordering system built with Rust. This project allows users to view a menu of pizzas, add them to a cart, and place an order with confirmation. It's an example of using modularized Rust code to handle user inputs, display information, and process orders.

## Features

- **Menu Display:** View a list of pizzas, categorized by veg and non-veg, with details like price, size, and whether it's vegetarian.
- **Add to Cart:** Select pizzas from the menu and specify the quantity to add to your cart.
- **User Information:** Enter customer details (name, phone number, address) for order confirmation.
- **Order Confirmation:** Review your order, and confirm it before checkout.
- **Error Handling:** Gracefully handles incorrect input for phone numbers, quantities, and other user inputs.

## Table of Contents

- [🍕 Domino's CLI Pizza Ordering System](#-dominos-cli-pizza-ordering-system)
  - [Features](#features)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Project Structure](#project-structure)
  - [Modules](#modules)
    - [`main.rs`](#mainrs)
    - [`user.rs`](#userrs)
    - [`pizza.rs`](#pizzars)
    - [`cart.rs`](#cartrs)
  - [Future Improvements](#future-improvements)
  - [Contributing](#contributing)
    - [Enjoy ordering your favorite pizzas from the comfort of your terminal! 🍕](#enjoy-ordering-your-favorite-pizzas-from-the-comfort-of-your-terminal-)

## Installation

1. **Prerequisites:**
   Ensure you have Rust installed on your machine. You can install Rust using the following command:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/yourusername/cli-pizza-ordering-system.git
   cd cli-pizza-ordering-system
   ```

1. **Build the Project:**

   Run the following command to build the project:

   ```bash
   cargo build
   ```

1. **Run the Application:**

   After building the project, use the following command to run the application:

   ```bash
   cargo run
   ```

## Usage

1. **Start the Application:**
   Upon running the app, you'll be greeted with a welcome message.

2. **User Input:**
   Enter your personal details (name, phone, address).

3. **Menu Selection:**

   - Select "1" to view the pizza menu.
   - Choose a pizza by name and specify the quantity.
   - Add it to your cart.

4. **Placing an Order:**

   - Once you've added items to your cart, select "2" to place your order.
   - Confirm your order when prompted.

5. **Exiting:**
   - Select "3" to exit the application.

## Project Structure

```bash
src/
├── cart.rs       # Cart and order-related logic
├── main.rs       # Entry point, handles the main loop
├── pizza.rs      # Pizza struct, menu, and display functionality
└── user.rs       # User-related logic and input handling
```

## Modules

### `main.rs`

This is the entry point of the application. It coordinates the workflow:

- Starts by asking for user details.
- Allows users to choose from viewing the menu, adding items to the cart, and placing an order.
- Imports the necessary modules like `user`, `pizza`, and `cart`.

### `user.rs`

Handles the user’s personal information:

- **`User` struct:** Stores user details like name, phone, and address.
- **`get_user_details()`**: Captures user input with error handling.

### `pizza.rs`

Manages the pizza menu:

- **`Pizza` struct:** Holds details about each pizza, such as name, price, category, and size.
- **`show_pizza()`**: Returns a list of available pizzas.
- **`display_menu()`**: Prints the pizza menu to the console.

### `cart.rs`

Handles shopping cart logic:

- **`CartItem` struct:** Stores details of pizzas added to the cart, along with quantity.
- **`add_to_cart()`**: Adds a selected pizza to the cart.
- **`make_order()`**: Displays the order summary and processes the order confirmation.

## Future Improvements

- **Add More Customization:**
  - Allow users to select different pizza toppings or customize orders.
- **Improve UI:**
  - Enhance the CLI experience with a clearer layout and better user interaction.
- **Order History:**
  - Add the ability to save and view past orders.
- **Discounts and Offers:**
  - Implement promotional discounts or coupon codes.

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -m 'Add some feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Open a pull request.

### Enjoy ordering your favorite pizzas from the comfort of your terminal! 🍕
