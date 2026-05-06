#![allow(unused)]

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn todo_twice(action: fn(u32, u32) -> u32, a: u32, b: u32) -> u32{
    action(a,b) + action(a,b)
}

fn push(v: &mut Vec<u32>, x: u32) {
    v.push(x)
}

fn executor(action: fn(&mut Vec<u32>, u32), list: &mut Vec<u32>, x: u32) {
    action(list, x);
}

fn main() {
    let f: fn(u32 , u32) -> u32 = add; //@note this is a fn pointer
    println!("{}", f(1,2));

    println!("{:?}", todo_twice(f, 1, 2));


    let mut my_array = Vec::new();
    executor(push, &mut my_array, 12);

    println!("my array: {my_array:?}");
}