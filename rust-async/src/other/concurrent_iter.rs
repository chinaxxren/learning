use futures::{io, Stream};
use std::pin::Pin;

/// for loops are not usable with Streams
pub async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt;

    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}

pub async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt;

    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }

    Ok(sum)
}
pub async fn jump_n_times(num: u8) -> Result<u8, io::Error> {
    println!("jump {num}");
    Ok(num + 1)
}
pub async fn jump_around(
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    use futures::TryStreamExt;
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    stream
        .try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
            jump_n_times(num).await?;
            // report_n_jumps.await?;
            Ok(())
        })
        .await?;

    Ok(())
}
