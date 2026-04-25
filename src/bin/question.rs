#![allow(unused)]

fn f1() -> Result<u32, String> {
    Ok(7)
}

fn f2() -> Result<u32, String> {
  Ok(4)
}

fn f3() -> Result<u32, bool> {
    println!("hello this is f3, the boolen guy!!!");
    Ok(66)
}

fn f_match() -> Result<u32, String> {
    let m_1 = f1();
    let x_1 = match m_1 {
        Ok(val) => val,
        Err(e) => return Err(e),
    };

    let  m_2 = f2();
    let x_2 = match m_2 {
        Ok(val) => val,
        Err(e) => return Err(e)
    };

    return Ok(x_1 + x_2);
}

fn f_question() -> Result<u32, String> {
    let x_1 = f1()?;
    let x_2 = f2()?;

    //@note we need to match on f3 because it doesn't return a string type on error
    let x_3 = match f3() {
        Ok(val) => val,
        Err(e) => return Err("an error occured".to_string())
    };

    return Ok(x_1 + x_2 +x_3);
}


fn main() -> Result<(), String> {
    // let res = f1();
    // match res {
    //     Ok(x) => println!("res is {x}"),
    //     Err(e) => println!("Error: {e}"),
    // }

    //@note this is the basic one testing it directly on the f1
    // let res = f1()?;
    // println!("res: {res}");
    // Ok(())


    //@note this is the extra one which involves combination of multiple result returned functions
    let final_sum = f_question()?;
    println!("final_sum: {final_sum}");
    Ok(())
}