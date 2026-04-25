#![allow(unused)]

#[derive(Debug)]

struct Point {
    x: f32,
    y: f32
}


//Struct Methods
impl Point  {
    //Associated functions - static method
    fn zero() -> Self {
        return Self {
            x: 0.0,
            y: 0.0
        };
    }


    // Methods 
    fn move_to (&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn dist(& self) -> f32 {
        return ((self.x * self.x) + (self.y * self.y)).sqrt();
    }
}
//Associated functions - static methods
//Methods

fn main() {
    let mut p = Point::zero();
    println!("{:?}", p);

    p.move_to(1.99, 2.0 );
    println!("{:?}", p);


    let d = p.dist();
    println!("dist: {}", d);

}