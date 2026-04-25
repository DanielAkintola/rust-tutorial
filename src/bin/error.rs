#![allow(unused)]


#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSqrt,
}

fn div(x: u32, y: u32) -> Result<u32,MathError> {
    if y == 0 {
        return Err(MathError::DivisionByZero);
    }else {
        return Ok(x / y);
    }
}

fn main() {
    //array indexing, trying to get outside array's bounds
    let arr = [1,2,3];
    println!("arr: {arr:?}");

    let x: Option<&i32> = arr.get(1);

    // println!("x: {x:?}");
    match x {
        Some(val) => println!("x: {val}"),
        _ => println!("x is None")
    }

    //dividing by zero
    let m = div(12,0);
    println!("m: {m:?}");


    let m = div(12,4);
    println!("m: {m:?}");



}