// mod channel;
// mod threads;

#[tokio::main]
async fn main() {
    println!("My Advance rust learnings!");

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
    foo().await;
}

async fn foo() -> u32 {
    println!("I'm from foo");
    5
}
