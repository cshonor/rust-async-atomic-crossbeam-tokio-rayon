//! ch05：用状态机模拟协程的 **Yield / Resume**（stable；与 `async_tokio/ch05_coroutines_generators/本章学习笔记.md` 对照）。
//!
//! 与 nightly 的 `Coroutine` / `CoroutineState` 名称相近，但本文件不依赖不稳定特性。

#[derive(Debug)]
enum Step {
    Yielded(&'static str),
    Complete(&'static str),
}

struct Lines {
    n: u8,
}

impl Lines {
    fn new() -> Self {
        Self { n: 0 }
    }

    /// 每次调用相当于从上次停下来的地方再往前走一步。
    fn resume(&mut self) -> Step {
        self.n += 1;
        match self.n {
            1 => Step::Yielded("第一屏"),
            2 => Step::Yielded("第二屏"),
            _ => Step::Complete("结束"),
        }
    }
}

fn main() {
    let mut c = Lines::new();
    loop {
        match c.resume() {
            Step::Yielded(v) => println!("Yielded: {v}"),
            Step::Complete(msg) => {
                println!("Complete: {msg}");
                break;
            }
        }
    }
}
