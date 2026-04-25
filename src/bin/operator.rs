#![allow(unused)]

fn main() {
    // +, -, *, /
    let a: i32 = 1;
    let b: i32 = 2;
     
    let c =  a + b;
    println!("{} + {} = {}", a, b, c);

    let c = a - b;
    println!("{} - {} = {}", a, b, c);

    let c = a * b;
    println!("{} * {} = {}", a, b, c);

    let c = a / b; // @note it rounds down
    println!("{} / {} = {}", a, b, c);

    // % (remainder != mod operator)
    // mod
    // a % b = r, [0 <= r < b] 
    // -1 % 2 = 1
    // rem
    // -1 % 2 = -1


    let a = -1;
    let b = 2;

    println!("{}", a % b);


    //Literals
    let a = 1i32;
    let b = 3u64;
    let c = 1.23e3; //all these are literals similar to ether and gwei in solidity

    let d = 1_000_000_000u32;
    println!("{}", d);


    //Boolean
    let a = true && false;
    let a = true || false;
    let a  = !true;

    // [boolean operators are: and, or, not]


    //Bitwise
    // 101
    let a: u8 = 5;
    //011
    let b: u8 = 3;

    let one: u8 = 7;
    let zero: u8 = 0;


    println!("a: {:03b}", a);
    println!("b: {:03b}", b);
    println!("a&b: {:03b}", a & b);
    println!("1&b: {:03b}", one & b);
    println!("0&b: {:03b}", zero & b);

    println!("a|b: {:03b}", a | b);
    println!("1|b: {:03b}", one | b);
    println!("0|b: {:03b}", zero | b);


    println!("a ^ b: {:03b}", a ^ b);


    println!("{}", 1u32 << 3); // [1000] => 8 => x << 3 => x * 2^3
    println!("{}", 16u32 >> 3); // [10000] => 16 => x >> 3 => x / 2^3

    println!("{}", 10u32 >> 3); 

    // 000,1
}


