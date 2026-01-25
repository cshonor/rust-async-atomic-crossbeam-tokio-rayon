use std::thread;
//用 闭包
fn main() {
    let x = 1;
    let y = 2;
    let z = 3;
    let f =thread::spawn(move || {
        println!("x: {}", x);
        println!("y: {}", y);
        println!("z: {}", z);
    });
    f.join().unwrap();
}