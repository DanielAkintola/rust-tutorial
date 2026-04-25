#![allow(unused)]
use super::foo;

pub fn print() {
    // let res = sum(1,2);
    // println!("my {res}");
    // a::print();

    foo::print();
} 

pub fn sum(x: u32, y: u32) -> u32{
    x + y
}

pub mod a;
