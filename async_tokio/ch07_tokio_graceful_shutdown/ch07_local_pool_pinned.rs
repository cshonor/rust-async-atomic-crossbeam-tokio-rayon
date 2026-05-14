//! ch07：`LocalPoolHandle` + `spawn_pinned_by_idx`（与 `async_tokio/ch07_tokio_graceful_shutdown/本章学习笔记.md` §3 对应）。
use tokio_util::task::LocalPoolHandle;

#[tokio::main]
async fn main() {
    let pool = LocalPoolHandle::new(4);

    let handle = pool.spawn_pinned_by_idx(
        || async {
            println!("任务正在线程 0 上运行");
            42
        },
        0,
    );

    let result = handle.await.expect("join");
    println!("最终结果: {result}");
}
