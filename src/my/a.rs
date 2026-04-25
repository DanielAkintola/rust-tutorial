

use super::super::foo;

#[derive(Debug)]
pub struct  S {
    pub id: u32,
    pub name: String
}


pub fn print() {
    println!("my inner print")
}

pub fn call_foo() {
    foo::print();
}

