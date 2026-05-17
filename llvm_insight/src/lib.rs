//! 供 **`cargo rustc -p llvm_insight -- --emit=llvm-ir`** 导出 LLVM IR 做对照。
//! 不追求 API 完备，只保留「能生成有差异的 IR」的最小切片。
//!
//! 与仓库主线对照：`atomic/` 中的内存序、`async_tokio/` 中的异步语义，
//! 最终都可在此用 IR 观察优化与栅栏形态（需对比不同 `opt-level`）。

use std::sync::atomic::{fence, AtomicU64, Ordering};

/// `Ordering::Relaxed` 读 —— 在 IR 里与 `SeqCst` 对比。
#[inline]
pub fn load_relaxed(counter: &AtomicU64) -> u64 {
    counter.load(Ordering::Relaxed)
}

/// `Ordering::SeqCst` 读。
#[inline]
pub fn load_seqcst(counter: &AtomicU64) -> u64 {
    counter.load(Ordering::SeqCst)
}

/// `Relaxed` 写。
#[inline]
pub fn store_relaxed(counter: &AtomicU64, v: u64) {
    counter.store(v, Ordering::Relaxed);
}

/// `Release` 写（常与另一线程 `Acquire` 配对）。
#[inline]
pub fn store_release(counter: &AtomicU64, v: u64) {
    counter.store(v, Ordering::Release);
}

/// 全序栅栏（IR 里常体现为平台相关内存屏障序列）。
#[inline]
pub fn fence_seqcst() {
    fence(Ordering::SeqCst);
}

/// 普通标量加法（对照：无原子、无屏障）。
#[inline]
pub fn add_plain(a: u64, b: u64) -> u64 {
    a.wrapping_add(b)
}
