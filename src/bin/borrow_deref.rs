#![allow(unused)]

fn deref_str(s: &mut String) {
    *s += "kingf";
}

fn main() {
    //deref tested right here 
    let mut s = String::from("rust");
    let s1 = &mut s;

    *s1 += "?"; 
    println!("{s}");
    // s1 += 1; 


    let mut s = String::from("rust");
    deref_str(&mut s);
    println!("after deref in func {s}");

    //deref coercion
    let x = 1;
    let y = &x;
    let z = &x;
    let w = y + z; 

    println!("w: {w}")

}