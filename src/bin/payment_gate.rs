#![allow(unused)]

trait PaymentGateway {
    fn pay(&self, amount: f32) -> bool;
}

struct Invoice<T: PaymentGateway> {
    payment_method: T,
    amount: f32,
}

impl<T: PaymentGateway> Invoice<T> {
    fn process(&self) {
        println!("{}", self.payment_method.pay(self.amount));
    }
}

struct CardPayment {
    sender: String
}

struct CryptoPayment {
    sender: [u8; 20],
}

impl PaymentGateway for CryptoPayment {
    fn pay(&self, amount: f32) -> bool {
        let sender = convert_bytes_string(&self.sender);
        println!("sender: 0x{}, sent: {amount}", sender);
        return true
    }
}

fn convert_bytes_string (bytes_input: &[u8; 20]) -> String {
    let s = bytes_input
    .iter()
    .map(|n| n.to_string())
    .collect::<Vec<String>>();


    println!("collect: {:?}", s);

    let s = s.join("");

    println!("joined s: {}", s);
    

    s
}

impl PaymentGateway for CardPayment {
    fn pay(&self, amount: f32) -> bool {
        println!("sender: {:?}, sent: {amount}", self.sender);
        return true
    }
}

fn main() {

    let crypto = CryptoPayment{
        sender: [1; 20]
    };

    let card = CardPayment{
        sender: String::from("Daniel Akintola")
    };



    let crypto_invoice = Invoice {
        payment_method: crypto,
        amount: 10.00
    };

    crypto_invoice.process();
}