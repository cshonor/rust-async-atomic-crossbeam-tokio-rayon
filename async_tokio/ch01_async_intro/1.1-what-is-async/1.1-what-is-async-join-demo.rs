//! 1.1 What Is Async：`tokio::join!` 在单线程运行时内并发等待（不增加 OS 线程）。

use std::time::Instant;
use tokio::time::{sleep, Duration};

async fn tick(ms: u64, label: &str) {
    sleep(Duration::from_millis(ms)).await;
    println!("{label} done");
}

#[tokio::main]
async fn main() {
    let t0 = Instant::now();
    tokio::join!(tick(80, "a"), tick(50, "b"), tick(20, "c"));
    println!(
        "=== join! wall time ~max(80,50,20) ms, actual {} ms ===",
        t0.elapsed().as_millis()
    );
}
