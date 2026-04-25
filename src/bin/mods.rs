#![allow(unused)]


use hello_rust::my;

fn main() {
    // my::print();
    // my::a::print();


    // let m = my::a::S{
    //     id: 14,
    //     name: "john".to_string()
    // };

    // println!("{:?}", m);

    my::a::call_foo();

    let a = my::sum(1 as u32, 34 as u32);
    println!(
        "a is {a}"
    );
}

