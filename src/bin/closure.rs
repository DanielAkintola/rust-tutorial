#![allow(unused)]

fn main() {
    let add_one = |x: i32| -> i32 {
        x + 1
    }; //@audit do you noticed this is a function we set up right here 
    //@audit what the heck is this lol 


    let result = add_one(5);
    // println!("result: {}", result); 


    let name = "Rust".to_string();
    let greet =  || {
        println!("Hello, {}!", name);
    };


    // greet();

    // println!("my name is {name}");


    let numbers = vec![1,2,3,4];
    let even_numbers: Vec<_> = numbers.into_iter().filter(|&x| x % 2 == 0).collect();
    //@audit the closure the filter took in must return the bool type 

    println!("{:?}", even_numbers);
}

