//! ch02：手动实现 `Future`（与 `async_tokio/ch02_async_rust_core/本章学习笔记.md` 对应）。
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CounterFuture {
    count: u32,
}

impl Future for CounterFuture {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        this.count += 1;
        println!("正在轮询，当前计数: {}", this.count);

        if this.count < 5 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(this.count)
        }
    }
}

#[tokio::main]
async fn main() {
    let my_future = CounterFuture { count: 0 };
    let result = my_future.await;
    println!("最终结果: {}", result);
}
