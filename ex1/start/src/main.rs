// 运行线程示例
mod thread_example;
mod usescope;
mod thread_advanced;
mod use_rc_arc;

fn main() {
    // 运行 Rc 和 Arc 示例（演示引用计数、线程安全等）
    use_rc_arc::main();
    
    // 如果想运行其他示例，取消下面的注释
    // thread_advanced::main();
    // usescope::main();
    // thread_example::main();
}