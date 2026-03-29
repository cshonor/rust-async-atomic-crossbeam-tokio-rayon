use rust_async_atomic_crossbeam_tokio_rayon::{atomic, async_tokio, crossbeam, rayon};
use std::env;

fn print_usage() {
    eprintln!(
        "\
用法: cargo run -- <命令> [子命令]

  all              依次运行 async_tokio → atomic → crossbeam → rayon（与旧版默认行为一致）
  async_tokio      Tokio 异步示例
  atomic           原子 / 内存序 简短概览（原 quick demo）
  atomic ex1       原 ex1 默认：条件变量等（较长）
  atomic ex2       原 ex2 全套：Atomic、lazy_init、fence 等（较长）
  crossbeam        crossbeam-channel + scope
  rayon            Rayon 并行迭代器

示例:
  cargo run -- all
  cargo run -- atomic ex2
"
    );
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "all" => {
            println!("学习 Rust 异步与并发：async + tokio + atomic + crossbeam + rayon\n");
            async_tokio::demo().await;
            println!();
            atomic::demo();
            println!();
            crossbeam::demo();
            println!();
            rayon::demo();
        }
        "async_tokio" => async_tokio::demo().await,
        "atomic" => match args.get(2).map(String::as_str) {
            Some("ex1") => atomic::run_ex1_default(),
            Some("ex2") => atomic::run_extended(),
            Some(other) => eprintln!("未知子命令: {other}（可用 ex1 / ex2）"),
            None => atomic::demo(),
        },
        "crossbeam" => crossbeam::demo(),
        "rayon" => rayon::demo(),
        _ => {
            eprintln!("未知命令: {}", args[1]);
            print_usage();
        }
    }
}
