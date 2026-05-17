//! 统一 IR 实验入口：从仓库根执行
//! `cargo rustc --manifest-path llvm_insight/Cargo.toml -p llvm_insight_lab -- --emit=llvm-ir`
//! （若在 workspace 中已加入本目录，则可用 `-p llvm_insight_lab`。）
//!
//! 精读第 4、7 章时在此增删函数；归档时将 `.ll` 复制到 `ir_samples/` 对应子目录。

use std::sync::atomic::{fence, AtomicU64, Ordering};

#[inline]
pub fn load_relaxed(counter: &AtomicU64) -> u64 {
    counter.load(Ordering::Relaxed)
}

#[inline]
pub fn load_seqcst(counter: &AtomicU64) -> u64 {
    counter.load(Ordering::SeqCst)
}

#[inline]
pub fn store_release(counter: &AtomicU64, v: u64) {
    counter.store(v, Ordering::Release);
}

#[inline]
pub fn fence_seqcst() {
    fence(Ordering::SeqCst);
}

#[inline]
pub fn add_plain(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}
