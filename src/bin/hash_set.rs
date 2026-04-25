#![allow(unused)]


// use std::collections::HashMap;
use std::collections::HashSet;


fn main() {
    let v = vec![1,2,3,1];
    // let map: HashMap<u32, bool> = HashMap::new();

    let mut set: HashSet<u32> = HashSet::new();

    let new = set.insert(12);
    println!("new element: {}", new);
    println!("{:?}", set);


    let new = set.insert(12);
    println!("new element hunh??, {new}");
    println!("{:?}", set);


    // check if the hash set contains an element or not
    let is_in = set.contains(&1);
    println!("1 is in: {}", is_in);


    let is_in = set.contains(&12);
    println!("12 is in: {}", is_in);


    
}