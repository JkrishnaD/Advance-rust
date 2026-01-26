// mod ownership;

use std::{collections::HashMap, num::ParseIntError};

fn main() {
    // Error handling consists of two main concepts
    // - Option -> value may or may not exist
    // - Result -> operation may sucess or fail
    // Rust make error handling part of the type system

    enum Option<T> {
        Some(T),
        None,
    }

    let x = Some(5);
    // let x = None;
    match x {
        Some(x) => print!("{}", x),
        None => println!("Got nothing"),
    };

    if let Some(x) = x {
        println!("{}", x)
    }
    let v = x.unwrap_or(6);
    println!("{v}");

    // let y = v.map() - this is not possible
    // .map() only exists on Option<T>, not on T
    // as x is a Option<i32> so it has this method
    // let y = x.map(|v| v * 2); // map is a transform function
    // println!("{:?}", y);

    // let n = parse_num("%");
    // println!("{:?}", n);

    // let p = Some(4);

    // match p {
    //     Some(x) => println!("{}", x),
    //     None => println!("Number not found"),
    // }

    // let e: Result<i32, &str> = Err("Invalid input");
    // // match e {
    // //     Ok(v) => println!("{}", v),
    // //     Err(e) => println!("Error: {}", e),
    // // }
    // let r = e.inspect_err(|e| println!("Error:{}", e));

    // println!("{:?}", r);
    // exercise - 1
    let c = first_char("Jaya");
    match c {
        Some(c) => println!("{}", c),
        None => println!("Not found"),
    }

    //exercise - 2
    let div = divide(10, 0);
    match div {
        Ok(d) => println!("divisin: {}", d),
        Err(e) => println!("Error: {}", e),
    }

    // exercise - 3
    let double = parse_and_double("4");
    match double {
        Ok(d) => println!("double: {}", d),
        Err(e) => println!("Error: {}", e),
    }

    // exercise - 4
    let mut data = HashMap::new();
    data.insert("user1".to_string(), "jaya".to_string());
    data.insert("user2".to_string(), "krishna".to_string());

    let user1 = get_username(&data, "user3");
    match user1 {
        Ok(name) => println!("username: {}", name),
        Err(e) => println!("Error: {}", e),
    }
}

fn first_char(s: &str) -> Option<char> {
    if s.is_empty() {
        return None;
    };

    let mut c = s.chars();
    c.next()
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("b equals to 0".to_string());
    };

    Ok(a / b)
}

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let num: i32 = s.parse()?;
    let d = num * num;
    Ok(d)
}

fn get_username(
    map: &std::collections::HashMap<String, String>,
    id: &str,
) -> Result<String, String> {
    match map.get(id) {
        Some(key) => {
            let name = key.to_string();
            return Ok(name);
        }
        None => {
            return Err("Id not found".to_string());
        }
    }
}

// fn parse_num(s: &str) -> Result<i32, ParseIntError> {
//     let n = s.parse()?;
//     Ok(n)
// }
