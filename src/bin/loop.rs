#![allow(unused)]

fn main() {
    //LOOP
    // let mut x  = 0; 
    // loop {
    //     println!("loop infinitely!!!");

    //     x += if x < 4 {
    //         1  
    //     } else {
    //         break
    //     }
    // }

    // WHILE  
    // let mut i = 0;
    // while i <= 3 {
    //     println!("while {i}");
    //     i += 1;
    // }

    //for loop
    // for i in 0..5 {
    //     println!("for loop {i}");
    // }

    //for loop array

    let arr = ["john".to_string(),"sam".to_string(), "victor".to_string()]; 

    for a in &arr {
        println!("{a}");
    }

    for a in arr.iter() { //@note array here is giving me an issue because it now contains complex concept initially it didn't because it stored only cimple data types 
        println!("{a}");
    }

    //usize and range
    // let arr = [1,2,3];
    // let n = arr.len();
    // for i in 0..n   {
    //     println!("{}th element is {}",  {i+1}, arr[i]);
    // }

    // //for loop vector, vectors are array that can grow and shrink in size 
    // let v: Vec<&str> = vec!["john","sam","victor"]; //@note why the vector gave me a strong issue is because it is a complex system itself 
    // //so the way it is implemented makes it to be properly handled when referenced 

    // for a in v.iter() { //@audit then we can call all these guys a semi-function no wonder they can return values 
    //     println!("{a}");
    // };

    // // to allow for safe looping through vectors multiple times we use 

    // for a in v.iter() {
    //     println!("{a}")
    // }

    // // let d = {
    // //     let x = 1 + 1;
    // //     x
    // // };

    // // println!("{d}");

    // // for a in v {
    // //     println!("{a}")
    // // }

    // let mut i = 0;
    // i = loop {
    //     if i == 3 {
    //         break 99;
    //     } 
        
    //     i += 1;
    // }; 

    // println!("i: {i}");

    // // labels 
    // 'outer: for i in 0..5 {
    //     'inner: for j in 0..5 {
    //         println!("{i}, {j}");

    //         if i == 1 && j == 2 {
    //             break 'outer;
    //         }
    //     }
    // }

}