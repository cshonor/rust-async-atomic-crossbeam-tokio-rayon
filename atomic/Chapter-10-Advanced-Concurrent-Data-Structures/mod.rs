//! 第 10 章 — 项目与灵感

#[path = "10.1-semaphores/10.1-semaphores-demo.rs"]
pub mod semaphores;

#[path = "10.2-rcu/10.2-rcu-pointer-swap-demo.rs"]
pub mod rcu;

#[path = "10.3-lock-free-linked-list/10.3-lock-free-stack-push-demo.rs"]
pub mod lock_free_stack;

pub fn demo() {
    semaphores::demo();
}
