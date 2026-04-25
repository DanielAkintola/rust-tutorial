#![allow(unused)]

fn main() {
    // let x: Option<u32> = Option::None;
    let x: Option<u32> = Some(12);

    // match x {
    //     Some(v) => println!("some: {v}"), 
    //     _ => println!("None")
    // }

   if let Some(v) = x { 
        println!("{v}")
    }  else {
        println!("x is None")
   }

    let Some(v) = x else {
        panic!("x is None");
    };

    println!("{v}")
     
}