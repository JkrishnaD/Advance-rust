// use std::{
//     sync::mpsc,
//     thread::{self, sleep},
//     time::Duration,
// };

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
