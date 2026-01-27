mod use_atomic;
mod lazy_init;
mod use_atomic_operations;
mod id_allocator;

fn main() {
    // 测试 Atomic 类型相关示例
    println!("========== 测试 Atomic 类型示例 ==========\n");
    
    use_atomic::demo_atomic_bool_stop_flag();
    use_atomic::demo_ordering_relaxed();
    use_atomic::demo_release_acquire();
    use_atomic::demo_why_need_release_acquire();
    use_atomic::demo_stdin_control();
    use_atomic::demo_ordering_summary();
    
    println!("\n========== 测试延迟初始化示例 ==========\n");
    
    // 测试延迟初始化示例
    lazy_init::main();
    
    println!("\n========== 测试原子操作方法 ==========\n");
    
    // 测试原子操作方法
    use_atomic_operations::main();
    
    println!("\n========== 测试 ID 分配器 ==========\n");
    
    // 测试 ID 分配器
    id_allocator::main();
}
