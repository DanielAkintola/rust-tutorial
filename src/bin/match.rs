#![allow(unused)]


enum Animal {
    Mouse,
    Cat, 
    Dog
}
fn main() {
    // tradtional way for handling check..and..do
    // let x = 1; 

    // if x == 1 {
    //     println!("one")
    // } else if x == 2 {
    //     println!("two")
    // } else {
    //     println!("other")
    // }

    // let x = 7; 

    // match x { //@note this match is powerful o
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ => println!("other")
    // }

    //multiple cases
    // match x {
    //     12 | 13 => println!(""),
    //     _ => println!("other")
    // }

    // let x = 10; 

    // // range 
    // match x {
    //     12..15 => println!("within the range [12, 15)"), //@note don't forget that the 
    //     _ => println!("other!!")
    // }

    // match x {
    //     // 1 ..= 10 => println!("within the range [1, 10]"), // standard way for inclusive
    //     i @ 1 ..= 10 => println!("matched {i}"), // standard way for detecting the matched pattern
    //     _ => println!("other!!")
    // }

    // let mut nums = [1,2,3];
    // let mut i = 0;

    // println!("{:?}", nums);

    // loop {
    //     let temp = nums[i];
    //     nums[i] = nums[(nums.len() - 1) - i];
    //     nums[(nums.len() - 1) - i] = temp;

    //     i += 1;
    //     if i > nums.len() % 2 {
    //         break;
    //     }
    // }

    // println!("{:?}", nums);



    // this is an explainer to the rearrange 
    // 5-1-0 => 4
    // 5-1-1 => 3
    // 5-1-2 => 2

    // 0,1,2,3,4


    let pet = Animal::Dog;

    // this is  testing the match functionality to display 
    // match pet {
    //     Animal::Cat => println!("my pet is a cat"),
    //     Animal::Dog => println!("my pet is a dog"),
    //     Animal::Mouse => println!("my pet is a mouse"),
    // }


    // this is  testing the match functionality to assign 
    // let animal_sound = match pet {
    //     Animal::Cat => "meow",
    //     Animal::Dog => "bark",
    //     Animal::Mouse => "squeaks",
    // };

    // println!("animal sound: {animal_sound}");

    let x: Option<i32> = Some(12);
    match x {
        Some(v) => println!("{v}"),
        None => println!("")
    }


    let res: Result<i32, String> = Err("an error occured!!".to_string());

    match res {
        Ok(out) => println!("{out}"),
        Err(msg) => println!("{msg}")
    }


}