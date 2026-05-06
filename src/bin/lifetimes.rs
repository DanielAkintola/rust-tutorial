#![allow(unused)]

fn longest_str<'a>(x: & 'a str, y: & 'a str) -> & 'a str {
    if x.len() > y.len() {
        x 
    } else {
        y
    }
}

fn main() {
    let x = "href".to_string();
     
    {
        let y = "green".to_string();
        let m = longest_str(&x, &y);

        println!(
            "{m}"
        )

    };   

}