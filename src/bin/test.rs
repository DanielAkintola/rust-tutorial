
#![allow(unused)]
// fn main() {
//     let test_str = "Hello World!!!";
//     println!("{test_str}");

//     {                      // s is not valid here, since it's not yet declared
//         let s = "hello";   // s is valid from this point forward
//         println!("{s}")
//         // do stuff with s
//     }    


    
    // println!("{s}")                  // this scope is now over, and s is no longer valid
 
    // // working with mutable strings
    // let mut s = String::from("hello");
    // String::push_str(&mut s,", world!"); // push_str() appends a literal to a String

    // println!("{s}"); // this will print `hello, world!`



    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{s1}, world!");


fn main() {
    let s1 = String::from("hello");

    let (s1, len) = calculate_length(s1);

    println!("The length of '{s1}' is {len}.");


    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String

        (s, length)
    }


    let mut s = String::from("hello");

    let r3 = &mut s;    
    let r4 = & s;    

    println!("{}  {}", s,  r4)



    // println!("{}", r3); // last use of r1



}