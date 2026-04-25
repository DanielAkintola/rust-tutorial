
#![allow(unused)]
fn main() {
    //array is a list of items with a fixed length known at compile time 
    //slice is a list of items with a unknown length at compile time 

    let arr: [u64; 5];
    arr = [1,2,3,4,5];

    println!("{:?}", arr);
    println!("first element {}", arr[0]);


    // //changing to mutable
    let mut arr: [u64; 5];
    arr = [1,2,3,4,5];

    arr[0] = 5;

    // // println!("{:?}", arr);
    // println!("new first element {}", arr[0]);

    // //placeholdering declaring arrays
    let mut arr: [char; 10] = ['B'; 10];
    // println!("{:?}", arr);


    //slice reference to an array 
    let arr = [1,2,3,4,5,6,7];
    let slice = &arr[0..3];

    // println!("slice: {:?}", slice);

    let mid_slice = &arr[2..5];
    // println!("slice: {:?}", mid_slice);


    let all_ele = &arr[..];
    println!("all element's slice: {:?}", all_ele);



}