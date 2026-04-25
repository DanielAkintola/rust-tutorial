#![allow(unused)]


//Struct 
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Point3d(f32, f32, f32);


struct Empty;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f32
}

fn main() {
    //Create
    let p = Point{x: 1.0, y: 2.0};
    println!("{:?}", p);

    let p = Point3d(
        2.0,
        2.0,
        2.0,
    );
    println!("{:?}", p);


    println!("{} {} {}", p.0, p.1, p.2);
    let circle = Circle {
        center: Point {
            x: 12.0,
            y: 11.0
        },
        radius: 12.0
    };


    println!("radius: {} x: {}, y: {}",  
    circle.radius, circle.center.x, circle.center.y);


    // shortcut
    let x =  1.0;
    let y = 1.0;
    let point = Point{x, y}; //we can use the variable name to set up the named fields of the 
    // > structure 

    // copy fields 
    let p0 = Point {x: 1.0, y: 1.0};
    let p1 = Point{x: 2.0, ..p0};


    println!("{:?} {:?}", p0, p1);


    // update 
    let mut p = Point {x: 0.0, y: 0.0};
    p.x += 10.0;
    p.y += 10.0;
    println!("{:?}", p);

}


