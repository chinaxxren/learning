use futures::executor::block_on;
use tokio::time::{sleep, Duration};

async fn hello_world() {
    // 在async fn函数中使用.await可以等待另一个异步调用的完成。
    // 但是与block_on不同，.await并不会阻塞当前的线程，而是异
    // 步的等待Future A的完成，在等待的过程中，该线程还可以继续
    // 执行其它的Future B，最终实现了并发处理的效果
    hello_cat().await;
    println!("hello, world!");
}

async fn hello_cat() {
    println!("hello, cat!");
}

fn main() {
    let future = hello_world();
    block_on(future); 
}