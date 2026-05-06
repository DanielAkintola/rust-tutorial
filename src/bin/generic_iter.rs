#![allow(unused)]

fn main() {

    // for v in vals.iter() { //@audit here we want to call loop twice on the same vector 
    //     println!("{v}")
    // }


    // for v in vals.iter() {
    //     println!("{v}")
    // }

    let mut vals: Vec<i32> = vec![1,2,3];


    for mut v in vals.iter_mut() {
        *v += 1; //@audit thanks so much for this 
        println!("{}", v);
    }

    println!("{vals:?}")

    // for mut v in vals {
    //     v += 1;
    //     println!({})
    // }

}


