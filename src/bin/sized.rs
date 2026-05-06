#![allow(unused)]


// Sized
// - Size is known at compile time
// - Automatically implemented for primitives types

//?Sized
// - size may not be known at compile time
// - Examples = dynamically sized types, slices, trait objects 


fn f<T: Sized>(x: T) {}
fn g<T: ?Sized>(x: &T) {}

trait A {}

impl A for u32 {}

fn d(x: Box<dyn A>)  {}


struct S {
    i: i32,
    j: i32
}

fn main() {
    let i = 1;
    let x: f64 = 1.0;
    let b = true;

    f(i);
    f(x);
    f(b);

    let new_s = S{
        i: 12,
        j: -2
    };

    f(new_s);

    let arr = [0; 4];
    f(arr);

    //?sized
    //@audit the note i added here is very wrong
    // let slice: &[i32; 3] = &[2,4,6];
    // f(arr); //@audit it works so well because we defined the size right there 

    let slice: &[i32]; //@audit the value of every element or variable has to be known at compile time 
    let arr: [i32; 10] = [1; 10];
    slice = &arr[0..];
    g(slice);


    let s = "Hello world";
    g(s);


    let v: Box<dyn A> = Box::new(1u32);
    g(&v);

    let dd = 123;
    g(&dd);

}