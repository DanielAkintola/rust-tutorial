#![allow(unused, warnings)]

struct Point <T> {
    x: T,
    y: T
}

struct Point_u32 {
    x: u32,
    y: u32,
}

//@note this is the system that will eventually be used, the compiler would monomorphize the program
struct Point_i32 {
    x: i32,
    y: i32,
}

// it will copy and paste the code that is using generics and replace it with the concrete type

fn get_X<T>(p: Point<T>) -> T {
    p.x
}

//@audit we added the <T> after point to show that we are working with a generic function, fuck we redeclared the T alright 


fn main() {
    let p0: Point<u32> = Point {x: 0, y: 0};
    let p1: Point<i32> = Point {x: 0, y: 0};

    // println!("x-axis: {}", get_X(p0));

    let p0x = get_X(p0);
    let p1x = get_X(p1);

    println!("{p0x:?} {p1x:?}");
}


// increase the size of binaries and increases slower compilation time 
// there is no runtime overhead there as to determine which code rust will execute there's none there 