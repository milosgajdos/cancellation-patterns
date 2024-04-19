use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // do some work
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Task completed");
    });

    // Cancel the task after 100 milliseconds
    time::sleep(Duration::from_millis(100)).await;
    handle.abort();
    time::sleep(Duration::from_secs(2)).await;

    println!("Task was cancelled");
}
