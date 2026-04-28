// fn greater<T>


fn strlen(s: impl Asref<str>) -> usize {
    s.as_ref().len()
}

pub fn strlen2<S>(s: S) -> usize
where S: Asref<str> {
    s.as_ref().len()
}


// pub fn strlen_refstr(s: &str) -> usize {
//     s.len()
// }

// pub fn  scheck_ref(s: String) -> usize {
//     s.len()
// }

pub fn foo() {
    strlen("hello world");
    strlen2(String::from("hello world"));
}