#![allow(unused)]

trait Notify {
    fn send(&self, msg: &str) -> bool;
}

struct Sms;
struct Email;

impl Notify for Email {
    fn send(&self, msg: &str) -> bool {
        println!("Email sent: {msg}");
        true
    }
}

impl Notify for Sms {
    fn send(&self, msg: &str) -> bool {
        println!("Sms sent: {msg}");
        true
    }
}

fn send_msg<T: Notify>(channel: &T, msg: &str) -> bool{
    channel.send(msg)
}

fn main() {
    let sms = Sms;
    send_msg(&sms, "hey daniel, how are you doing!!!");
}