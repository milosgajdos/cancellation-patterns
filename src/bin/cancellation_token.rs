use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    // Create a CancellationToken
    let token = CancellationToken::new();

    let token1 = token.clone();
    let token2 = token.clone();

    let task1 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token1.cancelled() => {
                        println!("Task 1 is cancelling...");
                        break;
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    println!("Task 1 completed normally");
                    break;
                }
            }
        }
        println!("Task 1 is cleaning up");
    });

    let task2 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = token2.cancelled() => {
                        println!("Task 2 is cancelling...");
                        break;
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    println!("Task 2 completed normally");
                    break;
                }
            }
        }
        println!("Task 2 is cleaning up");
    });

    sleep(Duration::from_millis(100)).await;

    // Send a cancellation signal
    token.cancel();

    // Wait for the tasks to finish
    let _ = tokio::join!(task1, task2);
}
