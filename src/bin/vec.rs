#![allow(unused)]

fn main() {
    //Vec<T>
    let v: Vec<i32> = vec![-1, 0, 1];
    let v: Vec<u32> = vec![1, 2, 3];
    let v:  Vec<i32> = Vec::new();

    // v.push(10);

    let v = vec![1u8,2,3];
    let v = vec![1u8; 5];

    println!("vec: {:?}, len: {}", v, v.len());


    //getting values in vector
    let v = vec![0,1,2,3];
    let x = v[0];

    println!("vec: {:?}, len: {}", v, v.len());
    println!("first element: {}", x);


    let v = vec![1,2,3];
    let x = v.get(0);

    match x {
        Some(val) => println!("val: {val}"),
        _ => println!("element not defined!!!")
    }


    let v: Vec<i32> = vec![];
    let x = v.get(0);

    match x {
        Some(val) => println!("val: {val}"),
        _ => println!("element not defined!!!")
    }

    //updating vectors
    let mut v = vec![1,2,3];
    // v[12] = 99; //@note we can only update if the index is in the length 
    v[2] = 99; 

    println!("greenwoooddddddddddddddd");


    let mut new_v = Vec::new();
    // new_v[0] =12; //@audit same thing here we must not try to set the value like this 
    new_v.push(12);

    println!("new_v {:?}", new_v);


    //pop
    // let mut v = vec![1,2,3]; //Pop returns an OPtion<T>
    let mut v: Vec<i32> = vec![]; //Pop returns an OPtion<T>
    match v.pop() {
        Some(val) => println!("popped element: {val}"),
        _ => println!("vector is empty")
    }

    //slice of a vector 
    let v = vec![1,2,3,4];
    let s = &v[0..2];

    println!("slice: {:?}", s);

}