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
    // //array indexing, trying to get outside array's bounds
    // let arr = [1,2,3];
    // println!("arr: {arr:?}");

    // let x: Option<&i32> = arr.get(10);

    // //using unwrap to get the value or panic if it's None
    // let x_val = x.unwrap();
    // println!("x_val: {x_val}");

    // //highlightig the possible value for the variable unwrap mechanism
    // // match x{
    // //     Some(val) => println!("x: {val}"),
    // //     _ => panic!("None value found for x")
    // // }   


    // let v: &i32 = x.expect("x is none");
    // println!("v: {v}");


    // println!("x: {x:?}");
    // match x {
    //     Some(val) => println!("x: {val}"),
    //     _ => println!("x is None")
    // }

    //dividing by zero
    // let m: Result<u32, MathError> = div(12,0);
    // println!("m: {m:?}");

    // 1. using unwrap on results to see the effect of error handling
    let m: Result<u32, MathError> = div(12,4);
    let m_val = m.unwrap();
    println!("m_val: {m_val}");


    let m: Result<u32, MathError> = div(12,0);
    let m_val = m.unwrap();
    println!("m_val: {m_val}");


    // 2. using expect on results to see the effect of error handling with custom messages
    let m: Result<u32, MathError> = div(12,4);
    let m_val = m.expect("division by zero error");
    println!("m_val: {m_val}");


    let m: Result<u32, MathError> = div(12,0);
    let m_val = m.expect("");
    println!("m_val: {m_val}");

    // let 
    // println!("m: {m:?}");



}