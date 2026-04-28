#![allow(unused)]

trait Language {
    fn name(&self) -> String; 
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

trait CompiledLanguage: Language +  Compiler { //@audit this is super trait 
    fn exec(&self, file_path: &str) {
        let name = self.name();
        println!("name: {name}");

        let cmd = self.compile(file_path);
        println!("cmd: {cmd}");
    }
}

struct Rust;

impl Language for Rust {
    fn name(&self) -> String {
        "rust".to_string()
    }
}

impl Compiler for Rust {
    fn compile(&self, file_path: &str) -> String {
        format!("cargo build {file_path}")
    }
}

impl CompiledLanguage for Rust{}


fn main() {
    //super trait 
    let my_love = Rust;
    my_love.exec("./info");
}