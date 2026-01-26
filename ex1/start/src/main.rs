// 运行线程示例
mod thread_example;
mod usescope;
mod thread_advanced;
mod use_rc_arc;
mod use_cell_refcell;
mod use_send_sync;
mod use_mutex;
mod use_mutex_guard_lifetime;
mod use_condvar;

fn main() {
    // 运行 Condvar 示例（演示条件变量、park/unpark、超时版本）
    use_condvar::main();
    
    // 如果想运行其他示例，取消下面的注释
    // use_mutex_guard_lifetime::main();
    // use_mutex::main();
    // use_send_sync::main();
    // use_cell_refcell::main();
    // use_rc_arc::main();
    // thread_advanced::main();
    // usescope::main();
    // thread_example::main();
}