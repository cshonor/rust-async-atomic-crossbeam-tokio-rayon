// 运行线程示例
mod thread_example;
mod usescope;
mod thread_advanced;
mod use_rc_arc;
mod use_cell_refcell;

fn main() {
    // 运行 Cell 和 RefCell 示例（演示内部可变性）
    use_cell_refcell::main();
    
    // 如果想运行其他示例，取消下面的注释
    // use_rc_arc::main();
    // thread_advanced::main();
    // usescope::main();
    // thread_example::main();
}