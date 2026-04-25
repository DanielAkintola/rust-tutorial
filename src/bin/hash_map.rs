#![allow(unused)]

use std::collections::HashMap;

fn main() {
    //Initialize
    let mut score: HashMap<String, u32> = HashMap::new();
    //Insert

    // score.insert("red". to_string(), 200);
    score.insert("green". to_string(), 150);
    score.insert("blue". to_string(), 150);
    //Get
    let val:Option<&u32>    = score.get("red");

    println!("red's score: {:?}", val);

    //Upsert
    let value = score.entry("red". to_string()).or_insert(14);
    *value += 13;


    let val = score.get("red");
    println!("red's score after upsert: {:?}", val);
}