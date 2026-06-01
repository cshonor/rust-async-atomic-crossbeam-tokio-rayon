# 第 10 章：并发与并行 (Concurrency and Parallelism) —— 深度解析

《Rust for Rustaceans》第 10 章站在**架构**与**底层**之间：不手把手写一个无锁队列，而是梳理在 Rust 里组织并发系统的**方法论**——何时用锁、何时用消息、异步与多核如何配合，以及原子与 **memory ordering** 的物理含义。若已读过 *Rust Atomics and Locks*（Mara Bos），本章可视为把「原子微观」接到「工程宏观」上的补充。

---

## 1. 为什么并发这么难？(The Trouble with Concurrency)

### 1.1 正确性

- **数据竞争**：借用检查与 `Send`/`Sync` 能在编译期挡掉大量内存级数据竞争，但**死锁**、**逻辑竞态**（基于业务顺序的 bug）仍可能发生。

### 1.2 性能与可扩展性

- **锁的串行化**：互斥会抹平并行收益；「为快而加线程」反而变慢并不少见。
- **伪共享 (False Sharing)**：两核修改**同一缓存行**上的不同变量，会触发缓存一致性流量。缓解手段包括**对齐与 padding** 把热点计数器拆到独立缓存行、数据结构分片等。

---

## 2. 三大并发模型 (Concurrency Models)

### 2.1 共享内存 (Shared Memory)

多线程通过 **`Mutex` / `RwLock`** 等直接共享可变状态。

- **适合**：多线程要共同更新复杂、非交换性的共享状态。
- **代价**：热点锁竞争；可按读写比例选 `RwLock`，或 **分片锁 (sharding)** 降低争用。

### 2.2 工作池 / 线程池 (Worker Pools)

固定工作线程从队列取任务（如 **rayon**、异步执行器的 worker）。

- **工作窃取 (work stealing)**：空闲 worker 从伙伴队列偷任务，提高利用率。
- **适合**：同质任务、数据并行（大数组分块等）。

### 2.3 Actor 模型

每个 actor **独占**状态，外部只通过 **channel 消息**驱动变更，内部往往无需 `Mutex`。

- **适合**：边界清晰、中等并发、自然「邮箱」建模的系统。
- **风险**：单 actor 过慢会成为**吞吐瓶颈**（背压与调度策略需整体设计）。

---

## 3. 异步与并行 (Asynchrony and Parallelism)

- **并发 (concurrency)**：任务**交错**推进；单线程 `async` 即可。
- **并行 (parallelism)**：任务**同时**执行；需要多核与多线程（或 SIMD 等）。

在异步 Web 等场景要**吃满多核**：由执行器 **`spawn`** 多个可独立调度的 **`Send`** 任务，使 futures 能分布到线程池。

### 异步中的锁

在 async 任务里持有 **`std::sync::Mutex`** 并跨越 **`.await`**，可能长时间阻塞执行器线程，拖垮整个调度。更常见的做法是使用**异步运行时提供的异步锁**（如 **`tokio::sync::Mutex`**），并注意其自身开销与持锁粒度设计。

---

## 4. 底层：原子与内存排序 (Lower-Level Concurrency)

手写无锁结构时需理解 **`std::sync::atomic`**。

### 4.1 物理现实

编译器可重排；CPU 可乱序；各核缓存并非瞬时一致——因此仅靠「单条指令原子」不足以表达跨线程的**可见性**契约。

### 4.2 Memory ordering（直觉）

| 顺序 | 直觉 |
|------|------|
| **`Relaxed`** | 仅本原子变量的原子性；**几乎不**建立与其他内存操作的顺序；最快、最易误用。 |
| **`Acquire` / `Release`** | 常用于锁语义：**Release** 之前的写入对之后 **Acquire** 同锁的线程可见。 |
| **`SeqCst`** | 全局一致顺序；强、贵；调试或极少数协议时考虑。 |

### 4.3 CAS 与 `compare_exchange_weak`

**CAS** 是无锁算法的核心原语。在 **ARM** 等上 **`compare_exchange_weak`** 允许**伪失败**（无竞争时也可能失败），典型写法是 **loop 重试**；在部分平台上可生成更高效的代码路径。

---

## 5. 工程实践 (Sane Concurrency)

1. **从简单开始**：先用 **`Mutex` / channel** 写对；无锁仅在 profiling 证明锁为瓶颈后再上。
2. **压力测试**：多线程高并发读写「抖动」出竞态与死锁。
3. **专业工具**：**Loom**（系统化探索交错）、**ThreadSanitizer (TSan)**（运行时数据竞争检测，适合 CI，需注意平台与编译器支持）。

---

## 小结

并发不是「编译通过即正确」。掌握 **共享内存 / 线程池 / Actor** 的取舍，分清 **async 与并行**，谨慎使用 **atomics + ordering**，并用 **Loom / TSan / 压力测试** 守住正确性，是本章给出的资深路径。

---

## 与仓库目录的对应

- 原书章名：**Concurrency and Parallelism**  
- 本仓库文件夹：`RFR/Chapter-10-Concurrency-and-Parallelism/`  
- 全书总目录：`RFR/RFR-本书目录.md`  
- 与 The Book 笔记呼应：仓库中 **`16-fearless-concurrency`** 等章节。
