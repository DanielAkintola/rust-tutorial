#![allow(unused)]

// use std::ops::Add;

fn main() {
    // immutable references/borrow 
    // let s = String::from("rust");
    // let s1 = &s;
    // let s2 = &s;
    // let s3 = &s;

    //mutable borrow
    // let mut s: String = String::from("daniel");
    
    
    // let mut s1 = &mut s;
    // println!("{s1}");
    
    // s.push_str(" vinccent");

    // println!("{s}")
    // let s1: &mut String = &mut s;
    // // let new = s1.clone().add("daniel"); //@note this guy doesn't do that it tries to move the value lol
    // // println!("{s1}"); //

    
    // s1.push_str(" daniel"); //@note this guys borrows the mutable value of the object 

    // println!("{s1}");
    // println!("{s}"); 
    
    // //@note why this guy is having an issue is because there already exist a mutable reference earlier, you noticed it wasn't used so we think we are good
    // //@note but guess not we are good, immediately we activated it down there 
    
    
    // //@audit s itself is a string right // there is a immutable borrow here 
    // // println!("{s1}"); //@audit why is it that here  // there is an mutable borrow here 




    let mut name = String::from("daniel");
    let ss1 = &name;
    let ss2 = &name;
    let s3 = &mut name;

    let s1 = &mut name;
    let s2 = &mut name;

    // println!("{ss1} {ss2}");

    println!("{s2}");


    let my_name = String::from("victor");
    let green = &my_name;

    // drop(my_name);

    println!("{green}");
}