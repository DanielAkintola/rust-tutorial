#![allow(unused)]

enum Option<T> {
    Some(T),
    None
}


enum Result<T,E> {
    Ok(T),
    Err(E)
}

struct Point<T = u32> {
    x: T,
    y: T
}

fn main() {
    let x: Option<u32> = Option::Some(12);
    let x: Option<i32> = Option::Some(-1);

    let res: Result<bool, String> = Result::Ok(true);

    let v: Vec<i32> = vec![1,2,3]; //@note we could also use Vec<_> where rust itself would infer the type


    let origin = Point{
        x: 1,y: 2
    };

    let origin: Point<u32> = Point{
        x: 1,y: 2
    };
}