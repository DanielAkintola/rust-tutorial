#![allow(unused)]
use std::fmt::Debug;

fn print_value<T> (value: T) {
    //...
}

trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!(
            "woof"
        )
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}


fn make_it_speak<T: Speak>(animal: T) {
    animal.speak();
}

fn main() {
    //rust doesn't assume the properties of Types 
    //so we use trait bounds to say, This generic type is allowed if only if it implements this trait

    let dog = Dog;
    let cat = Cat;

    make_it_speak(dog); 
    make_it_speak(cat); 
}