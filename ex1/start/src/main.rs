// 运行线程示例
mod thread_example;
mod usescope;
mod thread_advanced;
mod use_rc_arc;
mod use_cell_refcell;
mod use_send_sync;

fn main() {
    // 运行 Send 和 Sync 示例（演示线程安全 trait）
    use_send_sync::main();
    
    // 如果想运行其他示例，取消下面的注释
    // use_cell_refcell::main();
    // use_rc_arc::main();
    // thread_advanced::main();
    // usescope::main();
    // thread_example::main();
}