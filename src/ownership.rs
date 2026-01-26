fn main() {
    println!("My Advance Rust Learnings");
    let x;
    x = 42;
    println!("{}", x);
    // let y = &x;
    let x = 43;
    println!("{}", x);
    // assert_eq!(*y, 42)

    // let s = String::from("Hello"); // it get stored in heap memory
    // variable is a ptr to that heap
    // Stack:                Heap:
    // ┌─────────┐          ┌───┬───┬───┬───┬───┐
    // │ ptr     │─────────>│ H │ e │ l │ l │ o │
    // │ len: 5  │          └───┴───┴───┴───┴───┘
    // │ cap: 5  │
    // └─────────┘

    // String literal
    // let mut s1: &'static str = "hello";
    // this stored in the program binary read-only memory
    // Stack:                Binary (read-only):
    // ┌─────────┐          ┌───┬───┬───┬───┬───┐
    // │ ptr     │─────────>│ H │ e │ l │ l │ o │
    // │ len: 5  │          └───┴───┴───┴───┴───┘
    // └─────────┘
    let own = String::from("Hello");
    let t = own;
    // println!("{}",own) - this is invalid because String has heap allocation which doesn't implement any trait
    // the ownership of hello is moved from own to t so we can't access it
    takes(&t);
    println!("{}", t);

    let mut s = String::from("hello");

    // Multiple immutable references - Allowed
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // here we can't have a mutable reference because we still have a immutable reference variable scope
    // rust forbids this because it allows simultaneous read and write access which can break the memory safety
    println!("{} and {}", r1, r2);

    // Works: both r1 and r2 immutable refs scope is done
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);

    // level - 1
    let x = 5;
    let y = x;
    println!("{}", x);
    // here we won't get any error because i32 bool these types have Copy trait by default
    // so we can use this variable multiple times which is opposite to the string trait
    // string and Vec doesn't impl the Copy trait so they move the ownership which throws an error

    // level - 3
    let mut s = String::from("Hello");
    let r1 = &s; // ekada first immutable borrow aindhi kabati mari mutable borrow avadhu
    // let r2 = &mut s; // to counter this first we can first have the mutable and we can have just borrow
    println!("{}", r1);

    {
        let r = &mut s;
        r.push_str("!");
    } // this type wroks because the r has the limited scope and it doesn't effect the main scope any way
    // so the changes we have in this scope will go away when scope ends
    println!("{}", s);

    // Level - 4
    let len = process(&s);
    // here there is no ownership problem because the function is borrowing the string values
    // which doesn't move the ownership of the varible to the function it just uses the value
    // in here there will be no lifetime problem
    println!("len {}", len);

    let x = give();
    // there is no errors in here because we are just printing the varialbe from the function
    // the ownership flow in this function is that the function own's the string and we are using that
    println!("{x}");
}

// Level - 2
// passing a variable to a function moves it ownership to functions and we can't use that variable later
// borrowing allows temporary access without moving the ownership
fn takes(s: &String) {
    println!("Took {}", s)
}

fn process(s: &String) -> usize {
    s.len()
}

fn give() -> String {
    String::from("hello")
}
