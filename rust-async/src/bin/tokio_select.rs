use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // 创建两个异步任务
    let task1 = async {
        sleep(Duration::from_secs(2)).await;
        "Task 1 completed"
    };

    let task2 = async {
        sleep(Duration::from_secs(1)).await;
        "Task 2 completed"
    };

    // 使用 tokio::select! 等待两个任务中的任意一个完成
    tokio::select! {
        result = task1 => {
            println!("Task 1 finished first: {}", result);
        },
        result = task2 => {
            println!("Task 2 finished first: {}", result);
        },
    }
}