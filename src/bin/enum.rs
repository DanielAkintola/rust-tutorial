#![allow(unused)]

//Enum are custom Data 

#[derive(Debug, PartialEq)]
enum Color { //@note it uses namespacing to access the possible values 
    RED,
    BLUE, 
    GREEN,
    RGBA(u8, u8, u8, f32), //unnamed parameters using brackets 
    HEX(String),
    Hsl { //named parameters using curly brackets
        h: u8,
        s: u8, 
        l: u8
    }
}

#[derive(Debug)]
enum Height<T> {
    Short,
    Tall(T)
}


fn main() {
    // println!("hello")

    //Enum
    // let color: Color = Color::RED;
    // let color = Color::GREEN;
    // let color = Color::RGBA(0,0,0,1.0);
    // let color = Color::HEX("#FFFFFF".to_string());


    //Attributes - Debug and 
    // println!("{:?}", color);

    // PartialEq
    // println!("{}", Color::RED == Color::GREEN);
    //Option
    // @note this is an enum that can take on two values 
    // @note Some(T) | None


    // let x: Option<i32> = None;
    // let y: Option<u32> = Some(12);

    // println!("op: {:?}", y);
    

    // let my_height: Height<u32> = Height::Short;
    // println!("{:?}", my_height);

    //Result

    let z: Result<u64, String> = Ok(12);
    let err: Result<u64, String> = Err("div by 0".to_string());

    println!("Err: {:?}", err);



}