use std::{
    sync::{Arc, Mutex, RwLock},
    thread::{self, JoinHandle},
    time::Duration,
};

/*  Thread is an OS-managed execution unit with it's own stack
// threads helps to run multiple execution contexts within the same process concurrently
// threads communicate by sharing the data with each other with in the rust ownership rules
//
//
// Rust does not allow mutable access to the same value from multiple threads because that would permit data races.
// Data races cause undefined behavior, not just inconsistent values, and would violate Rust’s guarantee of memory safety.
// The ownership and borrowing rules prevent this at compile time.
*/

/* single thread
fn main() {
    // in here the Arc meand Atomically reference conunted - which means it keeps the track of how many owners exist
    // Arc counter = 1
    let data = Arc::new(vec![1, 2, 3, 4]);
    // counter = 2
    let data_for_thread = Arc::clone(&data);
    // creating thread by spawing it in here
    let handle = thread::spawn(move || {
        // process whih a thread handles
        println!("printing the data from thread {:?}", data_for_thread);
    });
    // we are waiting in here until the process ends
    handle.join().unwrap();

    println!("printing the data from main {:?}", data)
}
*/

/*  multiple threads
fn main() {
    // Arc doesn't allow any mutable reference it only give the ownership of the reference
    // to have a mutable reference we need to use `Mutex` which helps to have a mutable reference
    let data: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    // creating multiple threads using the loop
    for i in 0..=3 {
        let data_for_thread = Arc::clone(&data);
        // creating the thread using spawn
        let handle = thread::spawn(move || {
            // this make sures that whenever a thread is accessing the data other thread can't access it
            // until the thread done it work with the data so, this makes sure that multiple cannot access
            // the same data variable at the same time which leads to the data races and inconsistencies
            let mut data_cloned = data_for_thread.lock().unwrap();
            println!("printing data {:?} from thread {}", data_cloned, i);
            data_cloned.push(i);
        });
        // adding each thread to a variable
        handles.push(handle);
    }

    // handling the each thread in the variable and waiting until it's complete
    // the threads in the array wil not complete in an order each thread will take their own time and complete it
    // so we may see that last thread may complete first and the first thread may come at last
    for handle in handles {
        handle.join().unwrap();
    }

    println!("printing data {:?} from main thread", data.lock().unwrap());
}
*/

// mutex which we use here can only prevents the data races it can't prevent the deadlocks.

// deadlock - it is the situation when 2 threads are blocked each other by waiting for other thread to relase the locked variable

/*
fn main() {
    // RwLock is a read write lock which helps us to giev the write access to one thread and
    // also provide the read access to all other threads
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    // creating multiple threads using the loop
    for i in 0..=3 {
        let data_for_thread = Arc::clone(&data);
        // creating the thread using spawn
        let handle = thread::spawn(move || {
            // here in all the threads reading the data happens at the same time
            // but the write]ing the data will takes seperately
            if i == 2 {
                // here .write() methods creates the mutable reference so that the value can be changed
                let mut data_cloned = data_for_thread.write().unwrap();
                println!("writing data {:?} from thread {}", data_cloned, i);
                thread::sleep(Duration::from_secs(1));
                data_cloned.push(i);
                println!("Done")
            } else {
                // .read() method create a reference which gives us access to read
                let data_cloned = data_for_thread.read().unwrap();
                println!("reading data {:?} from thread {}", data_cloned, i);
                println!("Done")
            }
        });
        // adding each thread to a variable
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("printing data {:?} from main thread", data.read().unwrap());
}
*/

// sharing the data between the threads required two things
// 1 - shared ownership -> Arc<T>,
// 2 - safe access -> Mutex<T> or RwLock<T>
