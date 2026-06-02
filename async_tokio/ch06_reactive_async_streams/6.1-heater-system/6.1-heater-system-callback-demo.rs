//! ch06：同步回调最小例（与 `async_tokio/ch06_reactive_async_streams/本章学习笔记.md` §5 对应）。

fn perform_operation_with_callback<F>(callback: F)
where
    F: Fn(i32),
{
    let result = 42;
    callback(result);
}

fn main() {
    let my_callback = |result: i32| {
        println!("结果是: {result}");
    };
    perform_operation_with_callback(my_callback);
}
