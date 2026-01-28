use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("user1", "jaya");

    let v = map.get("user");
    match v {
        Some(d) => println!("user: {}", d),
        None => println!("Error: user not found"),
    }

    match map.contains_key("user1") {
        true => println!("map contains user1"),
        false => println!("user doesn't exist in map"),
    }

    let nums = count_words(&["a", "b", "a", "c"]);
    println!("nums: {:?}", nums)
}

fn get_score(map: &HashMap<String, i32>, name: &str) -> i32 {
    map.get(name).copied().unwrap_or(0)
    // Option<&T> -> Option<T>
}

// [a, b, a] -> {"a" : 2, "b" : 1}
fn count_words(words: &[&str]) -> HashMap<String, usize> {
    let mut nums: HashMap<String, usize> = HashMap::new();
    for w in words {
        let count = nums.entry(w.to_string()).or_insert(0);
        *count += 1;
    }
    nums
}

fn add_bonus(map: &mut HashMap<String, i32>, name: &str, bonus: i32) {
    // if user exist, add bouns
    // if not, do nothing
    if let Some(balance) = map.get_mut(name) {
        *balance += bonus
    }
}

fn take_user(map: &mut HashMap<String, String>, id: &str) -> Option<String> {
    map.remove(id)
}

fn transfer_balance(
    map: &mut HashMap<String, i32>,
    from: &str,
    to: &str,
    amount: i32,
) -> Result<(), String> {
    let from_balance = map.get(from).ok_or("From balance missing")?;
    let to_balance = map.get(from).ok_or("To balance missing")?;

    if *from_balance < amount {
        return Err("Insufficient balance".to_string());
    };

    *map.get_mut(from).unwrap() -= amount;
    *map.get_mut(to).unwrap() += amount;
    Ok(())
}


// accounts: user -> balance
// transactions: (from, to, amount)
//
// Apply all transactions.
// If ANY transaction fails:
//  - missing account
//  - insufficient balance
// return Err and do NOT modify balances.
fn apply_transactions(
    accounts: &mut HashMap<String, i32>,
    transactions: Vec<(String, String, i32)>,
) -> Result<(), String> {
    // first verify the transactions
    for (from, to, amount) in &transactions {
        let from_account = accounts.get(from).ok_or("From accout missing")?;
        if *from_account < *amount {
            return Err("Insufficient balance".to_string());
        }
    }

    // then modify the accounts
    for (from, to, amount) in &transactions {
        *accounts.get_mut(from).unwrap() -= amount;
        *accounts.get_mut(to).unwrap() += amount;
    }
    Ok(())
}

// Increment counter for key
// If value exceeds `limit`, return Err
fn increment_with_limit(
    map: &mut HashMap<String, usize>,
    key: &str,
    limit: usize,
) -> Result<(), String> {
    let counter = map.get(key).unwrap();

    if *counter >= limit {
        return Err("value exceeds limin".to_string());
    };

    *map.get_mut(key).unwrap() += 1;

    Ok(())
}

// Invert map: value -> keys
// {"a": 1, "b": 1, "c": 2}
// -> {1: ["a", "b"], 2: ["c"]}
fn invert_map(map: HashMap<String, i32>) -> HashMap<i32, Vec<String>> {
    let mut inverted = HashMap::new();
    // i didn't got this problem logic so i tried something in here
    // i need to know about this problem logic so that i can try this
    // like a pseudo code what do you think
    for (key, value) in map {
        inverted.entry(value).or_insert(Vec::new()).push(key);
    }

    inverted
}

// Group words by their length
// ["hi", "hello", "hey"]
// -> {2: ["hi"], 3: ["hey"], 5: ["hello"]}
fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>> {
    let mut group = HashMap::new();

    for w in words {
        group
            .entry(w.len()) // lenght has a key
            .or_insert(Vec::new()) // we need to have a vector for the values
            .push(w.to_string()); // we need to push the values into the vector which we have created
    }
    group
}

// Merge `b` into `a`.
// If key exists in both, keep the MAX value.
// If key exists only in one, keep it.
fn merge_max(a: &mut HashMap<String, i32>, b: HashMap<String, i32>) {
    for (k, v) in b {
        // if alredy a contains b value then
        // get the max value

        a.entry(k)
            .and_modify(|val| *val = (*val).max(v))
            .or_insert(v);
    }
}
