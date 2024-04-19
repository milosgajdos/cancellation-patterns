use tokio::sync::broadcast;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(1);
    let mut rx2 = tx.subscribe();

    let task1 = tokio::spawn(async move {
        tokio::select! {
            _ = rx1.recv() => {
                println!("Task 1 is cancelling...");
            }
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                println!("Task 1 completed normally");
            }
        }
        println!("Task 1 is cleaning up");
    });

    let task2 = tokio::spawn(async move {
        tokio::select! {
            _ = rx2.recv() => {
                println!("Task 2 is cancelling...");
            }
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                println!("Task 2 completed normally");
            }
        }
        println!("Task 2 is cleaning up");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send a cancellation signal
    let _ = tx.send(());

    // Wait for the tasks to finish
    let _ = tokio::join!(task1, task2);
}
