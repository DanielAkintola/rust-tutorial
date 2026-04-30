#![allow(unused)]

trait Notifier {
    fn send(&self, msg: &str);
}

struct Email;
struct Sms;

impl Notifier for Email {
    fn send(&self, message: &str) {
        println!("Sending email: {}", message);
    }
}

impl Notifier for Sms {
    fn send(&self, message: &str) {
        println!("Sending SMS: {}", message);
    }
}

fn notify_user<T: Notifier>(notifier: T, msg: &str) {
    notifier.send(msg);
}


fn main() {
    let email = Email;
    notify_user(email, "hey, how are you!!!");
}