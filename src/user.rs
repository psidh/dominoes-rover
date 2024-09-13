use std::io::{self, Write};

pub struct User {
    pub name: String,
    pub phone: i64,
    pub address: String,
}

pub fn get_user_details() -> User {
    let mut _name = String::new();
    let _phone = String::new();
    let mut _address = String::new();

    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut _name)
        .expect("Failed to read input");

    let phone = get_phone_number();

    print!("Enter your address: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut _address)
        .expect("Failed to read input");

    User {
        name: _name.trim().to_string(),
        phone,
        address: _address.trim().to_string(),
    }
}

fn get_phone_number() -> i64 {
    loop {
        let mut phone = String::new();
        print!("Enter your phone number: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut phone)
            .expect("Failed to read input");

        match phone.trim().parse::<i64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid phone number. Please enter a valid number."),
        }
    }
}
