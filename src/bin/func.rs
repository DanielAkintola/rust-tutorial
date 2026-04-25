#![allow(unused)]

fn add (x: u32, y: u32) -> u32 {
    x + y 
}

fn forever () -> ! {
    loop {}
}

fn print_and_add(x: u32, y: u32) {
    println!(
        "{}", x + y
    )
}

fn crash() -> ! {
    panic!("crashed!!!")
}


fn main() {
    let m: u32 = add(4294967295, 0);
    println!("m is {m}");

    print_and_add(12, 12);

    //diverge this guy whatever
    // forever();

    crash();
}
