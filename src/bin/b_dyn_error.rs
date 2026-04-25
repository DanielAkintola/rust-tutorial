#![allow(unused)]

use std::fmt::Debug;

#[derive(Debug)]
enum MathError {
    DivByZero
}

#[derive(Debug)]
enum ParseError {
    InvalidInt
}




impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "math error {:?}", self)
    }
}

impl std::error::Error for MathError {}



impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error {:?}", self)
    }
}

impl std::error::Error for ParseError {}


fn f1() -> Result<u32, MathError> {
    // Err(MathError::DivByZero)
    Ok(34)
}

fn f2() -> Result<u32, ParseError> {
    Err(ParseError::InvalidInt)
}

fn fbox() -> Result<(), Box<dyn std::error::Error>> {
    f1()?;
    f2()?;

    Ok(()) 
}

use std::env;
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;


fn read(src_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut src_file = File::open(src_path)?;
    let mut data = String::new();

    src_file.read_to_string(&mut data)?;
    let lines: Vec<String> = data.trim().split("\n").map(|s| s.to_string()).collect(); 

    Ok(lines)
}

fn sum(lines: Vec<String>) -> Result<i32, ParseIntError> {
    let mut sum: i32 = 0;
    for line in lines {
        let num: i32 = line.parse()?;
        sum += num;
    }

    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    // let z = fbox();
    // println!("{:?}", z)

    // match z {
    //     Ok(val) => val.fmt(), ///@audit will get back to this guy later 
    // }


    let total = sum(read("./new.txt")?)?;
    println!("total: {:?}", total); 
    Ok(())
}


// package => multiple crates, the entire system.
// crates => multiple modules,that works together. [binary crates]


// Module => this one let's you control the organization, scope and privacy of paths

// Paths => A way of naming an item, such as a struct, function or module


// we can have library crates this ones don't have a main function actually and they don't compile to an executable
// A create can come in two forms => binary crate (they have main functions) and library crate (they don't have a main functions)


// we also have the  crate root a source file that the rust compiler starts from and makes up the rooot module of your crate 