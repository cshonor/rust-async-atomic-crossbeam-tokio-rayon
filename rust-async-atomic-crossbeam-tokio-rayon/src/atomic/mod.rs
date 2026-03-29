//! 原子与并发基础示例（由原 `ex1` / `ex2` 练习合并）。

pub mod id_allocator;
pub mod lazy_init;
pub mod move_closure;
pub mod quick_demo;
pub mod thread_advanced;
pub mod thread_example;
pub mod use_atomic;
pub mod use_atomic_operations;
pub mod use_cell_refcell;
pub mod use_condvar;
pub mod use_fence;
pub mod use_mutex;
pub mod use_mutex_guard_lifetime;
pub mod use_rc_arc;
pub mod use_send_sync;
pub mod use_seqcst;
pub mod useboxleak;
pub mod usecall;
pub mod usejoin;
pub mod userecall;
pub mod usescope;
pub mod usestatic;

/// 简短概览：与根目录原先 `atomic::demo` 行为一致。
pub fn demo() {
    quick_demo::demo();
}

/// 原 `ex1/start` 默认入口：条件变量等示例。
pub fn run_ex1_default() {
    use_condvar::main();
}

/// 原 `ex2/start` 中的完整 Atomic 相关演示顺序。
pub fn run_extended() {
    println!("========== 测试 Atomic 类型示例 ==========\n");

    use_atomic::demo_atomic_bool_stop_flag();
    use_atomic::demo_ordering_relaxed();
    use_atomic::demo_release_acquire();
    use_atomic::demo_why_need_release_acquire();
    use_atomic::demo_stdin_control();
    use_atomic::demo_ordering_summary();

    println!("\n========== 测试延迟初始化示例 ==========\n");
    lazy_init::main();

    println!("\n========== 测试原子操作方法 ==========\n");
    use_atomic_operations::main();

    println!("\n========== 测试 ID 分配器 ==========\n");
    id_allocator::main();

    println!("\n========== 测试 SeqCst（顺序一致性）==========\n");
    use_seqcst::main();

    println!("\n========== 测试栅栏（Fence）==========\n");
    use_fence::main();
}
