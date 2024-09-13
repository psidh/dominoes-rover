#[derive(Debug, Clone)]
pub enum Size {
    Regular,
    Medium,
    Large,
}

#[derive(Clone)]
pub struct Pizza {
    pub name: String,
    pub category: String,
    pub price: f64,
    pub is_veg: bool,
    pub size: Size,
}

pub fn show_pizza() -> Vec<Pizza> {
    vec![
        Pizza {
            name: String::from("Margherita"),
            category: String::from("Veg"),
            price: 200.0,
            is_veg: true,
            size: Size::Regular,
        },
        Pizza {
            name: String::from("Farmhouse"),
            category: String::from("Veg"),
            price: 300.0,
            is_veg: true,
            size: Size::Medium,
        },
        Pizza {
            name: String::from("Peppy Paneer"),
            category: String::from("Veg"),
            price: 350.0,
            is_veg: true,
            size: Size::Large,
        },
        Pizza {
            name: String::from("Chicken Sausage"),
            category: String::from("Non-Veg"),
            price: 400.0,
            is_veg: false,
            size: Size::Regular,
        },
        Pizza {
            name: String::from("Chicken Golden Delight"),
            category: String::from("Non-Veg"),
            price: 450.0,
            is_veg: false,
            size: Size::Medium,
        },
    ]
}

pub fn display_menu(pizzas: &Vec<Pizza>) {
    println!("--- Pizza Menu ---");
    for pizza in pizzas {
        println!(
            "Name: {}\nCategory: {}\nPrice: {}\nIs Veg: {}\nSize: {:?}\n",
            pizza.name, pizza.category, pizza.price, pizza.is_veg, pizza.size
        );
    }
}
