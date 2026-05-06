#![allow(unused)]

// static and dynamic dispatch


// static dispatch
// - function to call is known at compile time
// - monomorphization

use std::fmt::Debug;

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

#[derive(Debug)]
struct C;

trait F: Debug {
    fn f(&self);
}

impl F for A {
    fn f(&self) {
        println!("{:?}", self)
    }
}

impl F for B {
    fn f(&self) {
        println!("{:?}", self)
    }
}

fn static_dispatch<T: F>(t: &T) {
    t.f();
    println!("{t:?}")
} 


fn dyn_dispatch(t: &dyn F) {
    t.f()
}

fn dyn_dispatch_box(t: Box<dyn F>) {
    t.f() // @note here we can still call t.f() right here lol!
}



fn main() {
    let obj = A;
    static_dispatch(&obj);
    println!("{obj:?}");


    let obj = B;
    static_dispatch(&obj);
    println!("{obj:?}");


    let input ="A";

    // let obj: Box<dyn F> = match input {
    //     "A" => Box::new(A),
    //     _ => Box::new(B)
    // };

    let obj: &dyn F = match input {
        "A" => &A,
        _ => &B
    };

    dyn_dispatch(obj);

    let obj: Box<dyn F> = match input {
        "A" => Box::new(A),
        _ => Box::new(B)
    };

    dyn_dispatch_box(obj);


    //[[]]] =>>> a trait object is a type that impls the trait, but the concrete type is not known at compile time 


}