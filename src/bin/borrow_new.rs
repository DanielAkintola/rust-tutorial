#![allow(unused)]

fn main() {
 
 
    // a()

    // let mut s: String = String::from("hello");
    // let r1 = & s;
    // let r2 = & s;


    // println!("{}, {}", r1, r2);

    // let r3 = &mut s;
    // r3.push('h');


    // println!("r3: {}", r3); //@audit we can add this one right 
    //@audit since the scope is a whole lot different from the others right here 

    dangle();

}

fn dangle() -> &String {
    let s: String = String::from("hello");
    &s
}

fn a() {
    let x = "green world";
    let y: i32 = 12;
    b();
}

fn b() {
    let my_str = String::from("green hello");
    println!("my str inside b: {my_str}");
}