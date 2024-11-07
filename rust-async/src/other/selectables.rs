use core::panic;
use std::future;

use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
    stream::{FusedStream, FuturesUnordered, Stream, StreamExt},
};

pub async fn task_one() -> future::Ready<i32> {
    println!("tasking one async");
    future::ready(2)
}

pub async fn task_two() -> future::Pending<()> {
    println!("tasking two async");
    future::pending()
}

pub async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();
    pin_mut!(t1, t2);
    loop {
        select! {
            a = t1 => println!("task one completed first! => result: {:#?}",a),
            b = t2 => println!("task two completed first!" ),
            complete => break,
            default => unreachable!(),
        };
    }
}
pub async fn add_two_streams<A, B>(mut s1: A, mut s2: B) -> u8
where
    A: Stream<Item = u8> + FusedStream + Unpin,
    B: Stream<Item = u8> + FusedStream + Unpin,
{
    let mut total = 0;
    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    total
}

async fn get_new_num() -> u8 {
    // future::ready(44).await
    45
}

async fn run_on_new_num(num: u8) {
    println!("running number: {num} ");
}

pub async fn run_loop<S>(mut interval_timer: S, starting_num: u8)
where
    S: Stream<Item = ()> + FusedStream + Unpin,
{
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(run_on_new_num_fut, get_new_num_fut);

    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // The timer has elapsed. Start a new `get_new_num_fut`
                // if one was not already running.
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            () = run_on_new_num_fut => {},

            complete => break,
            // panic!("`interval_timer` completed unexpectedly!"),
        }
    }
}

pub async fn run_loop2<S>(mut interval_timer: S, starting_num: u8)
where
    S: Stream<Item = ()> + FusedStream + Unpin,
{
    let mut run_on_new_num_fut_map = FuturesUnordered::new();
    run_on_new_num_fut_map.push(run_on_new_num(starting_num));

    let get_new_num_fut = Fuse::terminated();
    pin_mut!(get_new_num_fut);

    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // The timer has elapsed. Start a new `get_new_num_fut`
                // if one was not already running.
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                run_on_new_num_fut_map.push(run_on_new_num(new_num));
            },
            res = run_on_new_num_fut_map.select_next_some() => {
                println!("run_on_new_num_fut_map returned : {:#?}", res);
            },

            complete => break,
            // panic!("`interval_timer` completed unexpectedly!"),
        }
    }
}
