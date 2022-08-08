enum Ticket {
    BackStage(f64, String),
    VIP(f64, String),
    Standard(f64),
}
pub fn _fn() {
    let tickets: Vec<Ticket> = vec![
        Ticket::BackStage(1.1, "babak".to_owned()),
        Ticket::VIP(1.2, "vahid".to_owned()),
        Ticket::Standard(1.3),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Standard(amount) => println!("vector price is {:?}", amount),
            Ticket::BackStage(amount, holder) => println!("{:?} price is {:?}", holder, amount),
            Ticket::VIP(amount, holder) => println!("{:?} price is {:?}", holder, amount),
        }
    }
}
