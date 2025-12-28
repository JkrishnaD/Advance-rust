use std::{
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

// fn main() {
//     println!("This contains my learnings");

//     // mpsc - multiple producer and single consumer
//     let (tx, rx) = mpsc::channel::<String>();

//     let handle = thread::spawn(move || {
//         sleep(Duration::from_secs(3));
//         tx.send("Messgae 1 from thread 1".to_string()).unwrap();
//         tx.send("Messgae 2 from thread 1".to_string()).unwrap();
//     });
//     //  let message = rx.try_recvv()
//     // let possible_message = rx.recv_timeout(Duration::from_secs(1));
//     // match possible_message {
//     //     Ok(msg) => println!("Recieved Message: {}", msg),
//     //     Err(_) => println!("No message"),
//     // }

//     for msg in rx {
//         println!("Recieved message {}",msg)
//     }
//     handle.join().unwrap();
// }

// multiple senders and single reciever
// fn main() {
//     let (tx, rx) = mpsc::channel::<String>();
//     let tx1 = tx.clone();
//     let thread1 = thread::spawn(move || tx.send("Message from thread 1".to_string()).unwrap());

//     let thread2 = thread::spawn(move || tx1.send("Message from thread 2".to_string()).unwrap());

//     for msg in rx {
//         println!("Recieved Message:{}", msg)
//     }

//     println!("Recieve Iteration is Terminated");
//     thread1.join().unwrap();
//     thread2.join().unwrap()
// }

// channels we get by the mpsc are the asynchronus and unbounded channels
// which means that sender doen't need to know that reciever is present or not
// it sends endlessly until the memory is full and when ever the reciever is ready
// then they read the messages sent by the sender
// fn main() {
//     let (tx, rx) = mpsc::channel::<String>();

//     let handle = thread::spawn(move || {
//         for i in 1..5 {
//             tx.send(format!("Message from thread {i}")).unwrap();
//         }

//         println!("Finished sending! Terminating thread");
//     });

//     sleep(Duration::from_secs(3));

//     for msg in rx.iter() {
//         println!("Recieved msg: {}", msg)
//     }
//     println!("Rx loop finished");
//     handle.join().unwrap()
// }

// simillar to the channel which is asynchronous there are synchronous channels
// those are called sync_channels

// fn main() {
//     // having the zero as the bound creats a randevu channel which means
//     // before even sending the message sender and reciever needs to connect
//     // after the connection the messages will be sent
//     let (tx, rx) = mpsc::sync_channel::<String>(0);

//     let handle = thread::spawn(move || {
//         tx.send("Message 1 from thread".to_string()).unwrap();
//         println!("message 1 sent");
//         // the message 2 will only sent after the message 1 is recieved on the reciever side
//         // because the channel buffer is full it contains only one message see bound
//         // how many we can send depends on the bound we enter if it is 2
//         // we can send 2 message before reciever can recieve
//         tx.send("Message 2 from thread".to_string()).unwrap();
//         println!("message 2 sent");
//     });

//     sleep(Duration::from_secs(3));

//     let msgs = rx.recv().unwrap();
//     println!("Recieved first message {}", msgs);

//     handle.join().unwrap()
// }

// different methods both sender and reciever has
// sender -> .send() -> non-blocking method
// reciever -> .recv() -> blocking
//             .recv_timeout() -> blocking
//             .iter() -> blocking
//             .try_recv() -> non-blocking

fn main() {
    // let (tx, rx) = mpsc::channel();
    // let data = String::from("hello");

    // tx.send(data).unwrap();
    // tx.send(data).unwrap();
    // here it is failing because it is failing because the data is already moved
    // it's ownership to the reciever by first tx.send()
    // so when are tryig to send it again it throws the error as use of a moved value
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     tx.send(1).unwrap();
    // });

    // for msg in rx {
    //     println!("{}", msg);
    // }
    //The loop terminates because the sender is dropped when the spawned thread exits, which closes the channel.

    // let (tx1, rx1) = mpsc::sync_channel(10);

    // // A bounded channel enforces backpressure by blocking senders when the buffer is full.
    // for i in 0..1_000_000 {
    //     tx1.send(i).unwrap();
    // }

    // for msg in rx1 {
    //     println!("Message from bound: {}", msg);
    // }

    // drop(tx1);

    // let (tx2, rx2) = mpsc::channel::<i32>();

    // for i in 0..5 {
    //     let tx_clone = tx2.clone();
    //     thread::spawn(move || {
    //         tx_clone.send(i).unwrap();
    //     });
    // }

    // drop(tx2);
    // let mut data: Vec<i32> = vec![];

    // for msg in rx2 {
    //     data.push(msg);
    // }

    //println!("message data: {:?}", data);
    // why mpsc is more better fundamentally than Mutex and Arc
    // mpsc is better suited for async because it models ownership transfer and non-blocking coordination,
    // whereas Mutex relies on blocking and shared state, which breaks async execution

    // 1 - write a program a threads sends a string the main thread prints it and sender cannot use the string after sending
    // let (tx, rx) = mpsc::channel::<String>();
    // let msg = String::from("Hello from the sender");
    // let handle = thread::spawn(move || tx.send(msg));

    // println!("Recieved message: {}", rx.recv().unwrap());

    // let _ = handle.join().unwrap();

    // 2 - One thread sends 3 messages, Main thread iterates over rx, the loop must terminate naturally
    // let (tx, rx) = mpsc::channel::<i32>();
    // let (tx, rx) = mpsc::sync_channel::<i32>(5);
    // let mut handles: Vec<JoinHandle<()>> = vec![];
    // when it comes to the bound buffers it can buffer atmost messages mentioned in bound
    // if buffer is full then send() blocks until the reciever removes all the existing messages
    // this is called backpressure
    //
    // unlike channels send() in sync_channle() can block the sender thread in mid execution

    // Each producer must own it's own sender so each of them to be cloned before the threads even spawn
    // and the channels which are opened must be explicitly closed to terminate the loop
    // for i in 0..5 {
    //     for _ in 0..3 {
    //         let tx_clone = tx.clone();
    //         let handle = thread::spawn(move || {
    //             tx_clone.send(i).unwrap();
    //         });
    //         handles.push(handle);
    //     }
    // }
    // closing the chanels
    // drop(tx);

    // println!("All threads are eliminated");

    // in here we need to handle the messages first before handling the threads
    // so by the time the 5 messages bound reached to we need to clean the rx so that new messags can come in
    // if the thread handling is done first than rx then the execution will stop
    // because the channel is never cleared which leads to "DEADLOCK"
    // for msg in rx {
    //     println!("messages:{}", msg)
    // }

    // for handle in handles {
    //     handle.join().unwrap()
    // }

    // println!("All message are printed");

    //Write a program where: Receiver calls recv(), No sender exists for 3 seconds, Then a sender sends a message
    // let (tx, rx) = mpsc::channel::<String>();
    // let handle = thread::spawn(move || {
    //     sleep(Duration::from_secs(3));
    //     tx.send("Hello!".to_string()).unwrap();
    // });
    // println!("Message sent");

    // let msg = rx.recv().unwrap();
    // handle.join().unwrap();
    // println!("Recieved Message: {:?}", msg)

    // write code where producer generates data very quickly and consumer processes data very slowly
    // the major idea behind this is that we need to use the bound channels so that we can
    // restrict the messages limit from the sender.

    let (tx, rx) = mpsc::sync_channel::<i32>(2);

    // what really happening here is that
    // - we have created a situation where we have a bound channel with buffer capacity 2
    // - in the loop we are sending messages with the limit 10
    // - while recieving them we are printing each message and waiting for 2 secs
    // - the receiver continues to receive messages until all senders are dropped and the channel is closed.
    // - when the buffer reaches capacity, further send() calls block until the receiver consumes a message.
    // - when the receiver consumes a message, it frees one slot in the buffer
    // - allowing a blocked sender to proceed and this continues until the loop ends.
    let handle = thread::spawn(move || {
        for i in 0..10 {
            println!("Producing {}", i);
            tx.send(i).unwrap();
            println!("sent {}", i);
        }
    });

    for msg in rx {
        println!("Message Recieved: {:?}", msg);
        thread::sleep(Duration::from_secs(2));
    }

    handle.join().unwrap();
}
