use std::time::Duration;
use tokio::{select, task, time::sleep};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let token_clone = token.clone();
    let handle = task::spawn(async move {
        select! {
          _ = token_clone.cancelled() => println!("cloned token cancelled"),
          _ = sleep(Duration::from_secs(100)) => println!("sleep"),
        }
    });

    task::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        println!("cancel token");
        token.cancel();
    });

    handle.await.unwrap();
}
