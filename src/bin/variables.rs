#![allow(unused)]


fn main() {
    let num: i32 = -234;
    // num += 234; mutating immutable variable

    let mut x: i32 = 0;
    x += 1;
    println!("{x}");


    //placeholder for the types
    let nums: Vec<_> = vec![1,2,3];
    let nums: _ = true;
}