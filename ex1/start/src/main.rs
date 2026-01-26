// 运行线程示例
mod thread_example;
mod usescope;
mod thread_advanced;
mod use_rc_arc;
mod use_cell_refcell;
mod use_send_sync;
mod use_mutex;

fn main() {
    // 运行 Mutex 示例（演示互斥锁、MutexGuard、into_inner、DerefMut、线程休眠）
    use_mutex::main();
    
    // 如果想运行其他示例，取消下面的注释
    // use_send_sync::main();
    // use_cell_refcell::main();
    // use_rc_arc::main();
    // thread_advanced::main();
    // usescope::main();
    // thread_example::main();
}