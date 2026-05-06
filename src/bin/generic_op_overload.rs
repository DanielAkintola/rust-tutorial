#![allow(unused)]


use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x: T, 
    y: T
}

impl <T> Add for Point<T> where T: Add<Output = T>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
} 

//@audit more of like an recursive process shit lol otun gbemidebe 

fn main() {
    let p0 = Point{x: 0, y: 1};  
    let p1 = Point{x: 2, y: 3};

    let p2 = p0 + p1;


    println!("derived point: {p2:?}");

 
}