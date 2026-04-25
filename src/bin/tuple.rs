//complex data types
// arrays and tuples 

#![allow(unused)]
fn main() {
    //tuples
    let returned_struct: (bool, u32, char) = (false, 12, 'H');

    //destructuring 
    let (a,b,c) = returned_struct;

    //we can ignore the parts of the returned variables
    let (_,b,_) = returned_struct;
    println!("this is b's value: {}", b);

    

    println!("{:?}", returned_struct);
    println!("{}", returned_struct.0);
    println!("{}", returned_struct.1);
    println!("{}", returned_struct.2);
    // println!("{}", returned_struct); ====>>> this is not allowed you see 


    //wwe have an idea called the unit type
    let b: () = ();
    println!("{:?}", b);


    //nested tuple
    let c = ((16u32, true), (true, 12u32));
    println!("{:?}", c.0);
    println!("{:?}", c.1);

    //note: we can also use the nested dot notation to access elements
    println!("{}", c.0.0)
 

}