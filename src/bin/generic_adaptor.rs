#![allow(unused)]

fn main() {
    // Iterator Adaptor
    // map, filter, collect, zip, fold

    let vals = vec![1,2,3];
    let mut data = Vec::new();

    for v in vals {
        data.push(v * 2)
    }

    println!("{data:?}");

    // let data = vals.iter().map(f)

    let vals = vec![1,2,3];
    let mut semi_data = vals.iter();
    let mut data: Vec<u32> = vals.iter().map(|x| 2 * x).collect();

    println!("semi_data: {semi_data:?}");
    println!("data: {data:?}");

}