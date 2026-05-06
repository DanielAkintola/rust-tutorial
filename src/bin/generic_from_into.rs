#![allow(unused)]

use std::convert::{From,Into};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32 
}

impl From<(u32, u32)> for Point { //@audit the idea is to allow the system itself for us to design this grandeur
    fn from(value: (u32, u32)) -> Self {
        Point {
            x: value.0,
            y: value.1
        }
    }
}



fn main() {
    let t: (u32, u32) = (1,2);
    let gen_point = Point::from(t);

    println!("point: {:?}", gen_point);


    let gen_point: Point = t.into();
    println!("point: {:?}", gen_point);
}