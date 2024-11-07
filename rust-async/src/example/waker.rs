use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use futures::task::noop_waker;

struct MyAsyncTask;

impl Future for MyAsyncTask {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 模拟一个异步操作
        let now = std::time::Instant::now();
        println!("poll started at: {}", now.elapsed().as_secs());
        if now.elapsed() >= Duration::from_secs(1) {
            println!("Task completed after 2 seconds");
            Poll::Ready(())
        } else {
            println!("poll is still running");
            // 注册一个唤醒机制
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[async_std::main]
async fn main() {
    let my_task = MyAsyncTask;

    // 创建一个空的 Waker
    let waker = noop_waker();
    // 创建一个 Context
    let mut cx = Context::from_waker(&waker);

    // 使用 Pin 来固定任务
    let mut task = Box::pin(my_task);

    // 驱动任务的执行
    loop {
        match task.as_mut().poll(&mut cx) {
            Poll::Ready(_) => {
                println!("Task is complete");
                break;
            }
            Poll::Pending => {
                // 模拟一个事件循环
                println!("Task is still running");
                async_std::task::sleep(Duration::from_micros(100)).await;
                println!("Woke up from sleep");
            }
        }
    }
}