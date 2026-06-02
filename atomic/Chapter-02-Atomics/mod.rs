//! 第二章 — 笔记 `2.Y-slug.md`，代码 `2.Y-slug/*.rs`

#[path = "2.2-load-store/2.2-load-store-demo.rs"]
pub mod use_atomic;

#[path = "2.2-load-store/2.2-load-store-lazy-init-demo.rs"]
pub mod lazy_init;

#[path = "2.3-fetch-modify/2.3-fetch-modify-demo.rs"]
pub mod use_atomic_operations;

#[path = "2.4-cas/2.4-cas-id-allocator-demo.rs"]
pub mod id_allocator;

#[path = "2.5-quick-demo/2.5-quick-demo.rs"]
pub mod quick_demo;

#[path = "2.6-fence/2.6-fence-demo.rs"]
pub mod use_fence;

#[path = "2.7-seqcst/2.7-seqcst-demo.rs"]
pub mod use_seqcst;
