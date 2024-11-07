#![allow(dead_code, unused_variables)]

use async_std::{
    net::TcpListener,
    net::TcpStream,
    task::{self, block_on},
};
use futures::{future::join_all, AsyncWriteExt};

pub mod concurrent_iter;
pub mod executor;
pub mod joinables;
pub mod pinning;
pub mod pinning_heap;
pub mod selectables;
pub mod simplefuture;
pub mod timerfuture;

use crate::executor::{new_executor_and_spawner};
use crate::joinables::try_get_book_music;
use crate::pinning::Test;
use crate::pinning_heap::TestHeap;
use crate::selectables::{race_tasks, run_loop, run_loop2};
use crate::timerfuture::TimerFuture;

use futures::{channel::mpsc, stream::StreamExt, FutureExt};
use pin_utils::pin_mut;
use std::{future::Future, pin::Pin, time::Duration};

#[derive(Debug)]
struct Song {
    title: String,
    id: i32,
}

async fn learn_song() -> Song {
    Song {
        title: "dabadabadoo".to_string(),
        id: 5,
    }
}

async fn sing_song(song: Song) {
    println!("Singing: {}, {}", song.title, song.id);
}

pub async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}
pub async fn using_futures() {
    println!("hello async");
}

pub async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = using_futures();
    futures::join!(f1, f2);
}

#[allow(clippy::manual_async_fn, clippy::needless_lifetimes)]
pub fn foo_lifetime<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}

pub fn bad() -> impl Future<Output = u8> + 'static {
    /// original example
    // let x = 5;
    // foo_lifetime(&x)

    // revised to make it compile
    const X: u8 = 5;
    foo_lifetime(&X)
}

/// By moving the argument into the async block,
/// extend its lifetime to match that of the Future
/// returned from the call to good.
#[allow(clippy::manual_async_fn)]
fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        foo_lifetime(&x).await
    }
}

/// `async` block:
///
/// Multiple different `async` blocks can access the same local variable
/// so long as they're executed within the variable's scope
pub async fn blocks() {
    let my_string = "salitos".to_string();

    let future_one = async {
        println!("future one: {}", my_string);
    };

    let future_two = async {
        println!("future one: {}", my_string);
    };
}

pub async fn move_blocks() {
    let my_string = "salitos".to_string();

    let future_one = async move {
        println!("future one: {}", my_string);
    };

    // /// this will not compile since my_string is moved above
    // let future_two = async {
    //     println!("future one: {}", my_string);
    // };
}

pub fn execute_unpin_future(x: impl Future<Output = ()> + Unpin) {
    println!("executing future unpin");
}

pub async fn send_received() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);
    tx.try_send(1).unwrap();
    tx.try_send(2).unwrap();
    drop(tx);

    let val = rx.next().await.unwrap();
    println!("{val}");
    let val = rx.next().await.unwrap();
    println!("{val}");
    let val = rx.next().await.unwrap();
    println!("{val}");
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}

pub async fn process_request(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await?;
    stream.write_all(b"Hello world").await?;
    Ok(())
}

async fn foo_async(i: u32) -> u32 {
    i
}

fn set_git_revision_hash() {
    use std::process::Command;

    let args = &["rev-parse", "--short=10", "HEAD"];
    let Ok(output) = Command::new("git").args(args).output() else {
        return;
    };
    let rev = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if rev.is_empty() {
        return;
    }
    println!("cargo:rustc-env=ASYNCABLE_GIT_HASH={}", rev);
}

pub fn main() {
    set_git_revision_hash();
    let futures_container = vec![foo_async(1), foo_async(2), foo_async(3)];

    let _future = using_futures();
    block_on(using_futures());
    block_on(async_main());
    let (executor, spawner) = executor::new_executor_and_spawner();

    spawner.spawn(async move {
        println!(
            "joined all futures from container: {:#?}",
            join_all(futures_container).await
        );
    });

    let streamer = async move {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            task::spawn(async move { process_request(&mut stream).await });
        }
    };
    // spawner.spawn(streamer);
    // block_on(streamer);

    // Spawn a task to print before and after waiting on a timer
    spawner.spawn(async {
        println!("our executor & future says hello");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done after TimerFuture");
    });

    spawner.spawn(async {
        println!("our executor runs async_main");
        TimerFuture::new(Duration::new(2, 0)).await;
        async_main().await;
        println!("done after async_main & TimerFuture");
    });

    spawner.spawn(async {
        try_get_book_music().await.unwrap();
    });

    spawner.spawn(async { race_tasks().await });
    let stream = futures::stream::iter(1..=3).map(|_| ()).fuse();
    spawner.spawn(async {
        run_loop(stream, 5u8).await;
    });
    let stream2 = futures::stream::iter(1..=3).map(|_| ()).fuse();
    spawner.spawn(async {
        run_loop2(stream2, 25u8).await;
    });

    // Drop the spawner so that our executor knows it is finished and won't
    // receive more incoming tasks to run.
    drop(spawner);

    // Run the executor until the task queue is empty.
    // This will print "howdy!", pause, and then print "done!".
    executor.run();

    let mut test1 = Test::new("test1");
    test1.init();

    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b:{}", test1.a(), test1.b());
    println!("a: {}, b:{}", test2.a(), test2.b());

    std::mem::swap(&mut test1, &mut test2);
    println!("a: {}, b:{}", test1.a(), test1.b());
    println!("a: {}, b:{}", test2.a(), test2.b());

    let mut test3 = Test::new("test3");
    let mut test3 = unsafe { Pin::new_unchecked(&mut test3) };
    Test::init_pinned(test3.as_mut());

    let mut test4 = Test::new("test4");
    let mut test4 = unsafe { Pin::new_unchecked(&mut test4) };
    Test::init_pinned(test4.as_mut());

    println!(
        "a: {}, b:{}",
        Test::a_pinned(test3.as_ref()),
        Test::b_pinned(test3.as_ref())
    );

    println!(
        "a: {}, b:{}",
        Test::a_pinned(test4.as_ref()),
        Test::b_pinned(test4.as_ref())
    );

    // this wont compile since Test is !UnPin
    // std::mem::swap(test3.get_mut(), test4.get_mut());

    let mut test1 = TestHeap::new("test1");
    let mut test2 = TestHeap::new("test2");

    println!(
        "TestHeap test1: a: {:#?}, b: {:#?}",
        test1.as_ref().a(),
        test1.as_ref().b()
    );

    println!(
        "TestHeap test2: a: {:#?}, b: {:#?}",
        test2.as_ref().a(),
        test2.as_ref().b()
    );
    std::mem::swap(&mut test1, &mut test2);
    println!(
        "TestHeap after moved test1: a: {:#?}, b: {:#?}",
        test1.as_ref().a(),
        test1.as_ref().b()
    );

    println!(
        "TestHeap after moved test2: a: {:#?}, b: {:#?}",
        test2.as_ref().a(),
        test2.as_ref().b()
    );

    let fut_unpin1 = async {
        println!("unpin future1");
    };

    // 使用 Box::pin 包装异步块
    let fut_unpin1 = Box::pin(fut_unpin1);
    execute_unpin_future(fut_unpin1);

    // let fut_unpin1 = async {
    //     println!("unpin future1");
    // };
    // 使用 pin! 宏固定异步块
    // pin_mut!(fut_unpin1);
    // execute_unpin_future(fut_unpin1);

    let result = async {
        send_received().await;
    };
    execute_unpin_future(Box::pin(result));
}
