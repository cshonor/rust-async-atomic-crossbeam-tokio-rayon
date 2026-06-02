//! 第 5 章 — 通道

#[path = "5.1-mutex-based-channel/5.1-mutex-based-channel-demo.rs"]
pub mod mutex_channel;

#[path = "5.2-unsafe-one-shot-channel/5.2-unsafe-one-shot-channel-demo.rs"]
pub mod one_shot_channel;

pub fn demo() {
    mutex_channel::demo();
    one_shot_channel::demo();
}
