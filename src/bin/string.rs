#![allow(unused)]

fn main() {
    // when working with string we have 2 options 
    // 1. String type 2. the &str type [this is the string slice]


    // String ====>>>> Vec<u8> vectors of utf-8
    // &str ====>>>> slice of u8, &[u8]

    // we use the String when we want to allow mutation or data needs to be owned 
    //&str => when we want to support read only    


    // let msg: String = String::from("hello world!!!!😂");
    // println!("{}", msg);
    // println!("{:?}", msg);


    // let len: usize = msg.len();
    // println!("{}", len);


    // let str_slice = "green";
    // println!("{}", str_slice);

    // // get raw pointer to the first byte
    // let ptr = str_slice.as_ptr();
    // println!("Memory address: {:p}", ptr);


    // //ways to declare string slice  using pointer to mem
    // // get raw pointer to the first byte
    // // let ptr: *const u8 = str_slice.as_ptr();
    // let text = &msg[0..4];
    // println!("text slice: {}", text);


    // //deref coercion

    // let msg = String::from("hello guy");
    // // msg = String::from("hrege");

    // let msg_slice = &msg;
    // println!("msg_slice: {msg_slice}");


    // let green: &str = msg_slice;
    // println!("green: {green}");



    // //mutation
    // let mut msg = String::from("hello guy");
    // msg += "hrege";


    // println!(
    //     "{msg}"
    // );


    // //string interpolation
    // let lang = "Rust";
    // let emoji = "❤️";

    // let arr: [u32; 7] = [12; 7];

    // let msg = format!(
    //     "Hello {lang} {emoji} {}", arr[0]
    // );

    // println!("{msg}");



    // asisigment printing a string out in rust

    let text = "Daniel Victor".to_string();
    println!("{text}");


    let text_slice  = &text[0..7]; //@audit it says the value of the text cannot be known at compile time and that we should do something to that
    println!("{text_slice}");

}