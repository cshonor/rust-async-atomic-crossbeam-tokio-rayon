//! ch04：最小 mio —— `Poll` + `Token` + `TcpListener`（与 `async_tokio/ch04_network_io_runtime/本章学习笔记.md` 对应）。
//!
//! 运行后约半秒内无连接则事件列表可能为空；可另开终端 `nc 127.0.0.1 <端口>` 触发可读事件。

use std::time::Duration;

use mio::net::TcpListener;
use mio::{Events, Interest, Poll, Token};

const LISTENER: Token = Token(0);

fn main() -> std::io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(8);

    let addr: std::net::SocketAddr = "127.0.0.1:0".parse().expect("valid addr");
    let mut server = TcpListener::bind(addr)?;
    poll.registry()
        .register(&mut server, LISTENER, Interest::READABLE)?;

    let local = server.local_addr()?;
    println!("listening on {local} (try: nc 127.0.0.1 {})", local.port());

    poll.poll(&mut events, Some(Duration::from_millis(500)))?;

    let n = events.iter().count();
    println!("poll returned, {n} event(s)");
    for event in events.iter() {
        println!("  token={:?} readable={} writable={}",
            event.token(),
            event.is_readable(),
            event.is_writable(),
        );
    }

    Ok(())
}
