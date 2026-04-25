#![allow(unused)]

fn take(msg: String) {
    println!("take {msg}")
}

fn borrow(msg: &str) {
    println!("borrow {msg}")
}

fn borrow_mut(msg: &mut String) {
    // msg = String::from("greenwood");
    msg.push_str("greenwhich");
}

fn print_len_return_own(msg: String) -> String {
    println!("{}", msg.len());
    msg
}

fn print_len(msg: &str) {
    println!("{}", msg.len())
}

fn main() {
    // //taking ownership right here
    // let s = String::from("hello world");
    // take(s);

    // // println!("this won't work: {s}")

    // //borrowing the value right here
    // let s = String::from("hello, daniel");
    // borrow(&s);
    // println!("this will work: {s}");

    // //borrowing mutable form right here 
    // let mut s = String::from("my name is victor!!!");
    // borrow_mut(&mut s);
    // println!("{s}")


    // let  s = String::from("Jehovah!!!");
    // let s = print_len_return_own(s);  


    let s = String::from("Daniel Akintola");
    print_len(&s);
    println!("incoming leaderboard should be afraid {s}")  
}