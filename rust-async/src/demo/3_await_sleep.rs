use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};
use tokio::join;
async fn hello_world() {
    // 在async fn函数中使用.await可以等待另一个异步调用的完成。
    // 但是与block_on不同，.await并不会阻塞当前的线程，而是异
    // 步的等待Future A的完成，在等待的过程中，该线程还可以继续
    // 执行其它的Future B，最终实现了并发处理的效果
    
    // 并发执行
    join!(hello_pig(), hello_cat());

    // 串行执行
    // hello_pig().await;
    // hello_cat().await;

    println!("hello, world!");
}

async fn hello_pig() {
    println!("hello, pig!");
    sleep(Duration::from_secs(2)).await; // 异步睡眠 2 秒
    println!("pig has woken up!");
}

async fn hello_cat() {
    println!("hello, cat!");
}

fn main() {
    let rt = Runtime::new().unwrap();
    let future = hello_world();
    rt.block_on(future);
}
