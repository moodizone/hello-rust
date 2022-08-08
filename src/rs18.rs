struct Customer {
    age: i32,
}
fn purchase(customer: &Customer) -> Result<(), String> {
    match customer.age >= 21 {
        true => Ok(()),
        false => Err("Age restriction".to_owned()),
    }
}
fn print_purchase(purchase: Result<(), String>) {
    match purchase {
        Ok(()) => println!("Successful!"),
        Err(e) => println!("{:?}", e),
    }
}

pub fn _fn() {
    let customer: Customer = Customer { age: 33 };
    let purchase = purchase(&customer);
    print_purchase(purchase);
}
