#![allow(unused)]


trait Animal  {
    fn speak(&self) -> String;
}

#[derive(Debug)]
struct Cat;

#[derive(Debug)]
struct Dog;

impl Animal for Cat {
    fn speak(&self) -> String {
        "meow".to_string()
    }
}

impl Animal for Dog  {
    fn speak(&self) -> String {
        "woof".to_string()
    }
}

//@audit traits 
//@note here we know the trait at compile time right here
fn greet(animal: &impl Animal) {
    println!( //@note this is a static dispatch
        "{}", animal.speak()
    );
}

fn greet_dyn(animal: &dyn Animal) { //@note this is a dynamic dispatch
    println!("{}", animal.speak());
}

fn return_concrete_type() -> impl Animal {
    Dog
} 

fn rand_animal(rand: u32) -> Box<dyn Animal>{
    if rand <= 10 {
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}

//@note here we know the trait at runtime right here

fn main() {
    let my_pet = Cat;
    let dog = Dog;
    // my_pet.speak();
    greet(&my_pet);
    greet(&dog);

    let lost_pet= return_concrete_type();
    println!("{:?}", lost_pet.speak());


    let animal_str = "dog";
    let animal: &dyn Animal = match animal_str {
        "dog" => &Dog,
        _ => &Cat
    };

    greet_dyn(animal);
    let animal = rand_animal(11);

    println!("rand animal: {}", animal.speak());
}


//@audit all what we are doing is creating a ref in real time it doesn;t matter, 
//@audit why we can't use the reference is because it outlives the value from the function so we need to use a Box 


//@audit why can't i tell the Animal to derive debug