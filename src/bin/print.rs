#![allow(unused)]

#[derive(Debug)]

struct Lang {
    language: String,
    version: String
}


fn main(){
    let lang = "rust";
    println!("hello {lang}");
    println!("hello {}", lang);
    println!("hello {} {}", lang, lang);

    // placeholder printing
    let x = 2;
    println!("{0} x {0} = {1}", x, x * x);

    // printing struct 
    let lang_struct = Lang{
        language: "python".to_string(),
        version: "1.8.22".to_string()
    };


    println!("Language is {} and version is {}", lang_struct.language, lang_struct.version); // was checking if ownership affects 
    // println!("Language is {} and version is {}", lang_struct.language, lang_struct.version);

    println!("{:?}", lang_struct);
    println!("{:#?}", lang_struct);
}