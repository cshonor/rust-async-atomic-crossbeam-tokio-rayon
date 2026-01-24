// 运行线程示例
mod thread_example;
mod usescope;

fn main() {
    // 运行 thread::scope 示例
    usescope::main();
    
    // 如果想运行普通线程示例，取消下面的注释
    // thread_example::main();
}