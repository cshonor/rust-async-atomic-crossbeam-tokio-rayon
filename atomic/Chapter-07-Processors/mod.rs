//! 第 7 章 — 处理器与缓存

#[path = "7.2-caching/7.2-caching-false-sharing-demo.rs"]
pub mod false_sharing;

#[path = "7.3-reordering/7.3-reordering-relaxed-demo.rs"]
pub mod reordering;

pub fn demo() {
    false_sharing::demo();
}
