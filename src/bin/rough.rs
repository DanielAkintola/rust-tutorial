#![allow(unused)]

use std::mem;

fn main() {
    let mut u: Option<i32> = Some(4);
    let d = take(&mut u);

    println!("d: {d:?}");
    println!("u: {u:?}");


    let mut exam_string = Some("greenwood");
    let d = take(&mut exam_string);
    println!("d: {d:?}");
    println!("exam_str: {exam_string:?}");

}


fn take<T>(y: &mut Option<T>) -> Option<T> {
    // FIXME(const-hack) replace `mem::replace` by `mem::take` when the latter is const ready
    mem::replace(y, None)
}