//! ch06：用 `AtomicI16` + 两个 Tokio 任务粗略模拟「加热器 / 显示器」（与书中加热器案例对照）。
use std::sync::atomic::{AtomicI16, Ordering};
use std::sync::Arc;

use tokio::time::{sleep, Duration};

/// 目标温度（主体的一部分）
static DESIRED_TEMP: AtomicI16 = AtomicI16::new(22);

#[tokio::main]
async fn main() {
    let temp = Arc::new(AtomicI16::new(18));

    let heater = {
        let temp = Arc::clone(&temp);
        tokio::spawn(async move {
            for _ in 0..30 {
                let cur = temp.load(Ordering::Relaxed);
                let goal = DESIRED_TEMP.load(Ordering::Relaxed);
                if cur < goal {
                    temp.fetch_add(1, Ordering::Relaxed);
                }
                sleep(Duration::from_millis(80)).await;
            }
        })
    };

    let display = {
        let temp = Arc::clone(&temp);
        tokio::spawn(async move {
            let mut last = i16::MIN;
            for _ in 0..60 {
                let cur = temp.load(Ordering::Relaxed);
                if cur != last {
                    println!(
                        "[Display] 当前 {}°C，目标 {}°C",
                        cur,
                        DESIRED_TEMP.load(Ordering::Relaxed)
                    );
                    last = cur;
                }
                sleep(Duration::from_millis(40)).await;
            }
        })
    };

    let _ = tokio::join!(heater, display);
}
