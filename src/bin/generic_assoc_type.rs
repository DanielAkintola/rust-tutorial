#![allow(unused)]

// Associated type
// - placeholder type inside triat definition
// - placeholder is replaced by the implementation

//Difference with generic trait 

// - generic = multiple implementation per type
// - assoc type = 1 implementation per type 

use std::path::Iter;

use hello_rust::my;

trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

trait GenericIterator<T> {
    fn get_next(&mut self) -> Option<T>;
}

struct ArrayIter<T> {
    array: [T; 5],
    i: usize
}

impl GenericIterator<u32> for ArrayIter<u32> {
    fn get_next(&mut self) -> Option<u32> {
        match self.array.get(self.i) {
            Some(v) => {
                self.i += 1; 
                Some(*v)
            },

            _ => None
        }    
    }
}

impl GenericIterator<bool> for ArrayIter<u32> {
    fn get_next(&mut self) -> Option<bool> {
        Some(true)
    }
}


impl Iterator for ArrayIter<u32> {
    type Item = u32;
     fn next(&mut self) -> Option<Self::Item> {
        match self.array.get(self.i) {
            Some(v) => {
                self.i += 1; 
                Some(*v)
            },

            _ => None
        }    
    }
}


// impl Iterator for ArrayIter<u32> { this is not possible again like the generic trait lol
//     type Item = bool;
//      fn next(&mut self) -> Option<Self::Item> {
//         match self.array.get(self.i) {
//             Some(v) => {
//                 self.i += 1; 
//                 Some(*v)
//             },

//             _ => None
//         }    
//     }
// }

fn main() {
    let array: [u32; 5] = [1,2,3,4,5];

    let mut my_struct = ArrayIter {
        array,
        i: 0
    };

    // println!("{:?}", my_struct.get_next()); //@note i've manipulated this guy here 
    // println!("{:?}", my_struct.get_next());
    // println!("{:?}", my_struct.get_next());

    while let Some(v) = my_struct.next() {
        println!("{v:?}")
    } 




}