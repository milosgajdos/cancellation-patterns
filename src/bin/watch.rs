use tokio::sync::watch;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = watch::channel(false);
    let mut rx2 = tx.subscribe();

    let task1 = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = rx1.changed() => {
                    if *rx1.borrow() {
                        println!("Task 1 is cancelling...");
                        break;
                    }
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
                _ = rx2.changed() => {
                    if *rx2.borrow() {
                        println!("Task 2 is cancelling...");
                        break;
                    }
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {
                    println!("Task 2 completed normally");
                    break;
                }
            }
        }
        println!("Task 2 is cleaning up");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    // Send a cancellation signal
    let _ = tx.send(true);

    // Wait for the tasks to finish
    let _ = tokio::join!(task1, task2);
}
