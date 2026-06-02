//! 第一章 — 笔记 `1.Y-slug.md`，代码 `1.Y-slug/*.rs`

#[path = "1.3-thread-spawn/1.3-thread-spawn-move-closure-demo.rs"]
pub mod move_closure;

#[path = "1.3-thread-spawn/1.3-thread-spawn-example-demo.rs"]
pub mod thread_example;

#[path = "1.3-thread-spawn/1.3-thread-spawn-usecall-demo.rs"]
pub mod usecall;

#[path = "1.3-thread-spawn/1.3-thread-spawn-join-demo.rs"]
pub mod usejoin;

#[path = "1.3-thread-spawn/1.3-thread-spawn-return-demo.rs"]
pub mod userecall;

#[path = "1.4-scoped-threads/1.4-scoped-threads-demo.rs"]
pub mod usescope;

#[path = "1.5-shared-ownership/1.5-shared-ownership-static-demo.rs"]
pub mod usestatic;

#[path = "1.5-shared-ownership/1.5-shared-ownership-box-leak-demo.rs"]
pub mod useboxleak;

#[path = "1.5-shared-ownership/1.5-shared-ownership-arc-demo.rs"]
pub mod use_rc_arc;

#[path = "1.6-interior-mutability/1.6-interior-mutability-cell-refcell-demo.rs"]
pub mod use_cell_refcell;

#[path = "1.7-send-sync/1.7-send-sync-demo.rs"]
pub mod use_send_sync;

#[path = "1.8-mutex-rwlock/1.8-mutex-rwlock-mutex-demo.rs"]
pub mod use_mutex;

#[path = "1.8-mutex-rwlock/1.8-mutex-rwlock-guard-lifetime-demo.rs"]
pub mod use_mutex_guard_lifetime;

#[path = "1.8-mutex-rwlock/1.8-mutex-rwlock-thread-advanced-demo.rs"]
pub mod thread_advanced;

#[path = "1.9-parking-condvar/1.9-parking-condvar-demo.rs"]
pub mod use_condvar;
