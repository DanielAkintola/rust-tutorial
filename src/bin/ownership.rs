#![allow(unused)]

use core::time;
use std::ops::Add;

fn f(s: String) {}

fn print(s: String) {
    println!("{s}")
}

fn main() {
    let s = String::from("rust");
    f(s);

    // println!("{}",s);

    //Ownership rules
    // 1. Each value has an Owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped 

    // 1. Each value has an owner

    // the owner of "rust!!!" is S
    let s = String::from("rust!!!");
    // Owner of -1 is i
    let i: i32 = -1;






    //2. There can only be one owner at a time 

    //s -> s1 -> s2
    let s1 = s; 

    // in this case s will be dropped because now the owner of the value "rust!!!"
    // is now s1
    println!("{}", s1);
    // println!("{}", s); this will panic

    let s2 = s1;

    println!("{s2}");
    // println!("{s1}");


    // this case works because it is not a heapified data 
    let i = -1;
    let i1 = i;

    println!("{i} {i1}");





    // 3. When the owner goes out of scope, the value will be dropped 

    let mut m = String::from("rust");

    if true {
        // m.push_str("green");
        m;
    }

    // println!("{m}"); this panics lol

    // let's test it out with function for you to see
    let name = String::from("daniel");
    print(name);

    // println!("my name is {name}");



}