use futures::executor::block_on;
async fn learn_song() { 
    println!("I'm learning to sing");
}

async fn sing_song() {
    println!("I'm sing song");
}
async fn learn_and_sing() {
    println!("I'm learning to sing begin");
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    learn_song().await;
    
    // 唱歌必须要在学歌之后
    sing_song().await;

    println!("I'm learning to sing end");
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(f1, f2);
}

async fn dance() {
    println!("I'm dancing");
}

fn main() {
    block_on(async_main());
}