use tokio::time::Duration;
use tokio::{self, sync::oneshot};

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    let task = tokio::spawn(async move {
        tokio::select! {
            _ = rx => {
                println!("Task is cancelling...");
            }
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                println!("Task completed normally");
            }
        }
        println!("Task is cleaning up");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send a cancellation signal
    let _ = tx.send(());

    // Wait for the tasks to finish
    // NOTE: we could do this instead:
    // let _ = tokio::join!(task);
    let _ = task.await;
}
