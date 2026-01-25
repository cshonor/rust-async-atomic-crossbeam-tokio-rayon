// 运行线程示例
mod thread_example;
mod usescope;
mod thread_advanced;

fn main() {
    // 运行高级线程示例（包含多线程 push、只读、Box::leak 等）
    thread_advanced::main();
    
    // 如果想运行其他示例，取消下面的注释
    // usescope::main();
    // thread_example::main();
}