# cancellation-patterns

Rust task cancellation patterns

[![Build Status](https://github.com/milosgajdos/cancellation-patterns/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/milosgajdos/cancellation-patterns/actions?query=workflow%3ACI)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

See the examples below or run them yourself. All the code is in [src](./src/bin)

# Drop task JoinHandle

```rust
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // do some work
        tokio::time::sleep(Duration::from_secs(10)).await;
        println!("Task completed");
    });

    // Cancel the task after 100 milliseconds
    time::sleep(Duration::from_millis(100)).await;
    drop(handle);

    println!("Task was cancelled");
}
```

# Oneshot channel

```rust
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
```

# Broadcast channel

```rust
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
```

# Watch channel

```rust
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
```

# Cancellation token

```rust
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
```
