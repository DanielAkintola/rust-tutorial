#![allow(unused)]

trait Compare <T>{
    fn is_equal(&self, other: T) -> bool;
}

struct Number {
    value: i32
}

impl Compare<i32> for Number {
    fn is_equal(&self, other: i32) -> bool {
        self.value == other
    } 
}


//////////////////
/// storing generic data

trait Container<T> {
    fn add (&mut self, item: T);
}

#[derive(Debug)]
struct Boxed {
    items: Vec<String> 
}

impl Container<String> for Boxed {
    fn add(&mut self, item: String) {
        self.items.push(item)
    }
}

fn main() {
    let n = Number{value: 10};
    println!("{}", n.is_equal(10));

    let mut msg = Boxed{
        items: Vec::from(["1".to_string(), "2".to_string(), "3".to_string()])
    };

    Boxed::add(&mut msg, "green".to_string());

    println!(
        "mutated msg:  {:?}", msg
    )
}