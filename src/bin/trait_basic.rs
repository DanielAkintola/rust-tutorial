#![allow(unused)]


struct Solidity {
    version: String
}


struct Vyper {
    version: String
}

//@audit we have a trait this is the behaviour the type must implement 

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

trait Test {
    fn test(&self, file_path: &str) -> String {
        format!("{file_path}")
    }
}


impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {file_path}")
    }
    
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vy {file_path}")
    }
}


impl Test for Solidity {
    fn test(&self, file_path: &str) -> String {
        format!("forge test {file_path}")
    }
}


impl Test for Vyper {}



fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

fn test(lang: &impl Test, file_path: &str) -> String {
    lang.test(file_path)
}

fn main() {
    let sol = Solidity {
        version: "0.8".to_string()
    }; 

    let vy = Vyper{
        version: "0.4".to_string()
    };

    println!("{}", compile(&vy, "token.vy"));
    println!("{}", compile(&sol, "vault.sol"));

    println!("");

    println!("{}", test(&vy, "token.vy"));
    println!("{}", test(&sol, "vault.sol"));
}