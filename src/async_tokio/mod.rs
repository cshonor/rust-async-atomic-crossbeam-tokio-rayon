//! async/await + Tokio：运行时、`spawn`、`sync` 原语。

use std::time::Duration;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio::time::sleep;

/// 可运行的 Tokio 示例：spawn、oneshot、mpsc、共享 Mutex。
pub async fn demo() {
    println!("=== async_tokio（Tokio）===");

    // 1) tokio::spawn + await
    let h = tokio::spawn(async {
        sleep(Duration::from_millis(10)).await;
        40 + 2
    });
    println!("spawn 结果: {}", h.await.expect("join"));

    // 2) oneshot：单发单收
    let (tx, rx) = oneshot::channel::<String>();
    tokio::spawn(async move {
        let _ = tx.send("hello from oneshot".to_string());
    });
    println!("oneshot: {}", rx.await.expect("recv"));

    // 3) mpsc：多生产者单消费者
    let (tx, mut rx) = mpsc::channel::<u32>(8);
    for i in 0..3 {
        let mut t = tx.clone();
        tokio::spawn(async move {
            let _ = t.send(i).await;
        });
    }
    drop(tx);
    let mut sum = 0u32;
    while let Some(v) = rx.recv().await {
        sum += v;
    }
    println!("mpsc 收到的和: {} (期望 0+1+2)", sum);

    // 4) 共享状态：Arc<Mutex<T>>
    let counter = std::sync::Arc::new(Mutex::new(0i32));
    let mut handles = vec![];
    for _ in 0..4 {
        let c = counter.clone();
        handles.push(tokio::spawn(async move {
            let mut g = c.lock().await;
            *g += 1;
        }));
    }
    for h in handles {
        let _ = h.await;
    }
    println!("Arc<Mutex<i32>> 最终: {}", *counter.lock().await);

    println!("（还可扩展：Semaphore、RwLock、TcpListener 等）");
}
