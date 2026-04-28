#![allow(unused)]
use std::cmp::PartialOrd;

fn swap<A, B>(t: (A, B)) -> (B, A) {
    (t.1, t.0)
}


fn max<T: PartialOrd>(s: &[T]) -> Option<&T> {
    if s.len() == 0 {
        return None;
    } 

    let mut largest = &s[0];

    for item in s {
        if item > largest {
            largest = item
        }
    }

    return Some(largest)
}



fn main() {
    let t = (1,2);

    let s = swap(t);
    println!("{:?} {:?}", t, s);


    let t: (i32, u32) = (-1 ,2);

    let s: (u32, i32) = swap(t);
    println!("{:?} {:?}", t, s);



    println!("////////////////// Maximum value check /////////");

    let nums = vec![1,2,3,4,5];
    let largest = max(&nums);

    println!("largest num: {:?}", largest);


    let chars = vec!['a', 'b', 'c', 'd', 'P'];
    let largest = max(&chars);
    println!("largest num: {:?}", largest);
}