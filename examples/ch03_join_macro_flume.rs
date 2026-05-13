//! ch03：`join!` 宏（顺序 `block_on`）+ 最小 `flume` 通道（与 `async_tokio/ch03_custom_task_queue/本章学习笔记.md` 对应）。
//!
//! 注意：宏内对每个 future 依次 `block_on`，与 Tokio 的并发 `join!` 不同。

macro_rules! join {
    ($($future:expr),*) => {{
        let mut results = Vec::new();
        $(
            results.push(futures_lite::future::block_on($future));
        )*
        results
    }};
}

async fn task_one() -> u32 {
    10
}

async fn task_two() -> u32 {
    20
}

fn main() {
    let outcome: Vec<u32> = join!(task_one(), task_two());
    println!("join! (sequential block_on): {:?}", outcome);

    let (tx, rx) = flume::bounded::<u32>(4);
    tx.send(1).expect("send");
    tx.send(2).expect("send");
    println!("flume recv: {:?}", rx.recv());
    println!("flume recv: {:?}", rx.recv());
}
