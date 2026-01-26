// 运行线程示例
mod thread_example;
mod usescope;
mod thread_advanced;
mod use_rc_arc;
mod use_cell_refcell;
mod use_send_sync;
mod use_mutex;
mod use_mutex_guard_lifetime;

fn main() {
    // 运行 MutexGuard 生命周期示例（演示锁持有时间、常见误区、最佳实践）
    use_mutex_guard_lifetime::main();
    
    // 如果想运行其他示例，取消下面的注释
    // use_mutex::main();
    // use_send_sync::main();
    // use_cell_refcell::main();
    // use_rc_arc::main();
    // thread_advanced::main();
    // usescope::main();
    // thread_example::main();
}