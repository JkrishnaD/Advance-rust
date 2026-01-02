// use std::time::Duration;

// use tokio::time::Instant;

/*
#[tokio::main]
async fn main() {
    // synchronous execution - it is the process where a task is a sequence of operations
    // ex: a function task is executed from start to finish it is the default operation
    // mode of every program execution

    // asynchronous execution happens when a task is executed with the breaks in between
    // where it waits for other tasks to be executed in this mean time
    // this allows us to jump between the execution of two totally unrelated functions
    // it means when task 1 is in break 2 executes when 2 in break 1 executes

    // parallelism - this means where two tasks are running at exact same time
    // these tasks are either can be independent or they can communicate with each other
    // in these the execution of the code is synchronous.

    // concurrency - it is the way where the execution of second happens before even
    // the execution of first task ended
    // it can be achieved in both the ways either by parallelism or async code execution.

    // the async methods we we define are futures so to be able to run them we need to
    // await them so for that we need to have main funcion as async function
    // this is the part wnere threads are being used.

    // switching between the threads is always an expensive task because each thread has it's own stack
    // so when even we pause a thread it need to store it's own stack in the memory
    // and when re run that again i need to copy the stack information from the memory
    // so with all these processes it'll become expensive.

    // async execution it the process where we run multiple tasks on a single thread
    // but this method needs to be managed because tasks are created and managed by async runtime
    // when a thread is active async runtime decides which task needs to be excueted
    // this will be done in asynchronous way

    // so one of the popular async runtime which we use is tokio
    // now we can use the await in this main function
    // foo().await;

    let start_time = Instant::now();
    // this line will execute first and wait for the future execution to complete
    // let url_status1 = get_status("https://jkrishna.xyz").await.unwrap();
    // println!("Status Code 1: {}", url_status1);

    // // after status_2 this second call will run this is concurency
    // // to make these run concurrently we need to use a tokio method
    // // called join()
    // let url_status2 = get_status("https://marabos.nl/atomics/basics.html")
    //     .await
    //     .unwrap();
    // println!("Status code 2: {}", url_status2);

    // this async spawn will run conurrently with our status requests
    tokio::spawn(heartbeat(0));
    // here both the execution run concurrently
    // let (status_1, status_2) = tokio::join!(
    //     get_status("https://jkrishna.xyz"),
    //     get_status("https://github.com/google/comprehensive-rust")
    // );
    // println!("Status Code 1: {}", status_1.unwrap());
    // println!("Status Code 2: {}", status_2.unwrap());

    // in this method we are able to run only one execution
    // which means that if the first execution is succesful then it stops execution
    // and it won't even run the 2nd execution
    tokio::select! {
        stat =  get_status("https://jkrishna.xyz") => println!("Status Code 1: {}", stat.unwrap()),
        stat =  get_status("https://github.com/google/comprehensive-rust") => println!("Status Code 2: {}", stat.unwrap())
    }
    println!(
        "Time taken for execution {}ms",
        start_time.elapsed().as_millis()
    )
}

// async functions are norma functions which returns the implement trait Future<>
// async fn foo() -> u32 {
//     println!("I'm from foo");
//     5
// }

async fn get_status(url: &str) -> Result<reqwest::StatusCode, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    // reqwest is an async method so we need to use await in here
    let status_code = reqwest::get(url).await?.status();
    let duration = start_time.elapsed().as_millis();
    println!("It took {}ms to fetch url {}", duration, url);
    Ok(status_code)
}
// all the async funtions which we write are all the futures which means that they won't run immediately
// they need tokio async runtime to be executed using await as future are lazy which drive to zero execution
// so future do nothing unless you await them
// we can have multiple async function within each but at the top level main function we can't have them directly
// we need a runtime or an executor which helps to run these futures so for this we need tokio which is a async runtime

async fn heartbeat(mut num: u32) {

    loop {
        println!("beating... {}", num);
        tokio::time::sleep(Duration::from_millis(50)).await;
        num += 1
    }
}

// the future trait looks like
impl Future{
    type Output;
    fn poll(&mut self, Pin:fn) -> Poll<>;
}

enum Poll{
    Ready,
    Pending
}
*/

use std::time::Duration;

use tokio::sync::mpsc::{Receiver, Sender, channel};

// sending message for every second
async fn send_message(tx: Sender<String>) {
    let mut message_nr = 1;
    loop {
        tx.send(format!("Message from async: {}", message_nr).to_string())
            .await // we need to await as we are in async channels
            .unwrap();
        message_nr += 1;
        tokio::time::sleep(Duration::from_secs(1)).await
    }
}

// printing the maessage whenever rx recieves it
async fn recv_message(mut rx: Receiver<String>) {
    while let Some(message) = rx.recv().await {
        println!("Recieved Message: {}", message)
    }
}

#[tokio::main]
async fn main() {
    // all the tokio channles bounded so we need to provide the limit
    let (tx, rx) = channel::<String>(5);

    tokio::spawn(send_message(tx));
    recv_message(rx).await
}
