//! ch01：用 `tokio::join!` 并发发起多个 HTTP 请求（与 `async_tokio/ch01_async_intro/本章学习笔记.md` 对应）。
use std::time::Instant;

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let start_time = Instant::now();

    let (_, _, _, _) = tokio::join!(
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
    );

    println!("并发请求总耗时: {} ms", start_time.elapsed().as_millis());
    Ok(())
}
