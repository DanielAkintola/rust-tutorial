
#![allow(unused)]

fn main() {
    // integer types is written by iN, where N is the size of bits we use to represent the values
    // Example is the i32, -2^(N - 1) ==== 2^(N - 1) - 1
    let base: i32 = -2;
    // let num: i32 = -base.pow(30) - 1;
    // println!("{num}");

    //unsigned integers is written by uN, where N is the size of bits we use to represent the values
    //Example is the u32 ===>>> 2^N - 1
    // let base_unsigned: u32 = 2;
    // let num: u32 = base_unsigned.pow(12);
    // println!("{num}");


    let num: i32 = base.pow(11);
    println!("{num}");

    //floats we can have the f32 and the f64 
    let f0: f32 = 0.01;
    let f0: f64 = 0.01;

    // characters
    let c: char = '😂';
    println!("{c}");

    let i: i32 = 123;
    let u: u32 = i as u32;

    println!("{i} is {u}");


    //abomination  but it works 💯
    let i: i32 = -123;
    let u: u32 = i as u32;


    println!("{i} is {u}");


    let x: i32 = 123;
    let u: u32 = 67;
    // let m: u32 = u + x; can't work because of different types
    let m: u32 = u + (x as u32);

    println!("sum is {m}");


    // min/max
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("i32: Min is {min_i} and Max is {max_i}");


    let min_u: u32 = u32::MIN;
    let max_u: u32 = u32::MAX;

    println!("u32: Min is {min_u} and Max is {max_u}");


    let min_u6: u64 = u64::MIN;
    let max_u6: u64 = u64::MAX;

    println!("u64: Min is {min_u6} and Max is {max_u6}");


    let c_max: char = char::MAX;
    let c_min: char = char::MIN;
    println!("char: Min is {c_min} and Max is {c_max}");


    // overflow
    let mut u: u32 = u32::MAX;
    println!("u: {u}");

    u += 13;
    println!("u: {u}");

    //overflow handling
    let u = u32::wrapping_add(u32::MAX, 112);
    println!("wrapping_add is {:?}", u);

    let u = u32::checked_add(u32::MAX, 0);
    println!("checked_add is {:?}", u); 
    
}