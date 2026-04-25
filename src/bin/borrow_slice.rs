#![allow(unused)]


fn borrow(s: &mut[i32]) {
    s[0] = 12;
    println!("{:?}", s);
}

fn split(slice: &[i32], i: usize) -> (&[i32], &[i32]) {
    return(&slice[0..i], &slice[i..])
}

fn main() {
    //Borrow and slices
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    let mut s: &mut [i32] = &mut a[0..2];
    borrow(&mut s);

    println!("{:?}", s); //@note slices are always borrowed
    println!("{:?}", a);

    //splitting a slice at a specific index 
    let arr = [1,2,3,4,5,6];
    let slice_arr = &arr[1..4];

    println!("slice_arr: {:?}", slice_arr);

    let (a,b) = split(slice_arr, 1);
    println!("slices: {:?} {:?}", a, b);
}