#![allow(unused)]

trait List <T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T; //@audit look here we are returning a borrowed value lol!
}

impl List<u32> for (u32, u32) {
    fn count(&self) -> usize {
        2
    }

    fn first(&self) -> &u32 {
        &self.0
    }
}

impl <T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }

    fn first(&self) -> &T {
        &self[0]
    }
}

impl <X,Y> List<(X,Y)> for [(X,Y); 2] {
    fn count(&self) -> usize {
        self.len()
    }

    fn first(&self) -> &(X,Y) {
        &self[0]
    }
}

fn main() {
    let name = Vec::from(["Daniel".to_string(), "Akintola".to_string(), "Victor".to_string()]);
    println!("first: {:?}, count: {:?}", name.first(), name.count());

    let xy: (u32, u32) = (1,2);
    println!("xy count: {:?}", xy.count());
}