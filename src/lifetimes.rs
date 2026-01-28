use std::collections::HashMap;

fn main() {
    let r;
    {
        let x = 10;
        r = &x;
    }
    // reference cannot live longer than the value it refers to
    // println!("{}", r)
    // most of the time lifetimes are inferred by the compiler, so we only annotate 'a them
    // when the references are ambiguous

}

// lifetimes are mostly required when we are returning the references
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

// struct never owns the data it only store the references
// so it need to lifetimes to make that stuct cannot outlive the value.
struct User<'a> {
    name: &'a str,
}

// passing a referenc value to a function doesn't need any lifetime because compiler infer it in the function body
// but the lifetimes are need to be provided for the refernce which are returned because the compiler need to
// know that to which value the reference is tied to which prevents that the reference doesn't outlive the live
// without lifetime the value will be amibiguous and leads to dangling references
// fn get_user<'a>(map: &'a HashMap<String, String>, id: &str) -> Option<&'a String> {}

// here we only need lifetime fo a because we are only returning a so we need to know that is return reference
// will outlive the value so we use lifetime only for returned values
fn first<'a>(a: &'a str, b: &str) -> &'a str {
    a
}

// the string we are returning is a local variable so we cannot return a reference to a local variable
// so to make this function work we need to return the String rather than the str
fn bad() -> String {
    let s = String::from("hi");
    s
}

// anytime struct doesn't owns the data it only refers to the data to
// we need lifetimes in here to make that the reference won't outlive the values
struct Config<'a> {
    name: &'a str,
}
// for what i think this function looks to me if there is any problem
// that we need life time for name and at the function and the returning struct
fn make_config<'a>(name: &'a str) -> Config<'a> {
    Config { name }
}

// lifetime 'a in here is only reference to the hashmap because
// we are only returning the hashmap here and we are not returning the id
// so i doesn't make any sense to use the lifetime for that
fn find_user<'a>(map: &'a HashMap<String, String>, id: &str) -> Option<&'a String> {
    map.get(id)
}

fn parse_positive(s: &str) -> Result<i32, String> {
    // parse int
    // error if <= 0
    let num = s.parse::<i32>().map_err(|_| "parse failed".to_string())?;
    if num <= 0 {
        return Err("num is less than 0".to_string());
    };
    Ok(num)
}

fn record_hit(map: &mut HashMap<String, usize>, key: &str) {
    // increment count
    *map.entry(key.to_string()).or_insert(0) += 1;
    let ent = map.entry(key.to_string());
}
