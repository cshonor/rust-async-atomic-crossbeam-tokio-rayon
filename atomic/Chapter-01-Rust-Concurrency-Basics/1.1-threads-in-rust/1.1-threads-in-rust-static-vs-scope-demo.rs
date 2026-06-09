//! §1.1.3 · `spawn` 必须 `F: 'static` vs `thread::scope` 可借栈
use std::thread;

pub fn main() {
    demo_spawn_move_fix();
    demo_scope_borrow_local();
}

/// `thread::spawn`：不能捕获 `&local`（非 `'static`）→ 须 **`move`** 拿所有权。
pub fn demo_spawn_move_fix() {
    let local = String::from("hello");
    let h = thread::spawn(move || {
        println!("spawn (move): {local}");
    });
    h.join().unwrap();
}

/// `thread::scope`：可**借用 scope 外**的栈上数据（`F: 'scope`，不要 `'static`）。
pub fn demo_scope_borrow_local() {
    let local = String::from("hello"); // 定义在 scope **外**，子线程可借
    thread::scope(|s| {
        s.spawn(|| {
            println!("scope (borrow): {local}");
        })
        .join()
        .unwrap();
    });
}

// 取消注释以下函数可复现编译错误（`F` 不满足 `'static`）：
//
// pub fn demo_spawn_borrow_fails() {
//     let local = String::from("hello");
//     let r = &local;
//     let _h = thread::spawn(move || {
//         println!("{r}"); // ❌ 闭包含 &'a，不是 'static
//     });
// }
