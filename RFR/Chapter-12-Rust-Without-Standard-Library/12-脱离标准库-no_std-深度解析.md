# 第 12 章：脱离标准库的 Rust (`no_std`) —— 深度解析

《Rust for Rustaceans》第 12 章面向 **裸机 / 嵌入式 / 自研 OS** 等场景：没有完整操作系统提供文件、网络与默认堆分配器时，如何用 **`core`（与可选的 `alloc`）** 仍写出可维护、尽量安全的 Rust。

---

## 1. 退出标准库：`#![no_std]`

- **`std`**：在典型目标上依赖 OS 能力；其下可视为建立在 **`core`** 与（可选）**`alloc`** 之上。
- **`core`**：子集与平台无关：代数类型、迭代器、切片、`ptr` 等；**不假设堆**。
- **`#![no_std]`**：crate 根属性；不再链接完整 `std`（目标需支持；细节以参考手册为准）。预导入变为 **`core`** 侧 prelude 语义（与版本/edition 相关）。

**库的常见做法**：默认 **`no_std`**，通过 **`feature = "std"`** 在宿主环境打开 `std` 扩展，兼顾嵌入式与常规用户。

---

## 2. 动态内存 (`alloc`)

纯 `no_std` + 无全局分配器时，**不能**使用依赖全局堆的类型（如 **`Box` / `Vec` / `String` / `Arc`** 等，除非自行提供 `alloc` 环境）。

- **栈与静态**：固定上限容器（如 **`heapless::Vec`** 或自写 **`[MaybeUninit<T>; N]`** 上的安全封装、const 泛型 **`ArrayVec` 思路**）避免堆。
- **引入堆**：实现 **`GlobalAlloc`** 并通过 **`#[global_allocator]`** 注册后，可链接 **`extern crate alloc`** 使用 `Box`/`Vec` 等（仍需处理 **OOM** 与碎片策略）。

---

## 3. “隐藏”运行时职责

常说 Rust **无重量级 GC 运行时**，但二进制仍有 **panic / 启动 / OOM** 等契约点。

- **`#[panic_handler]`**：`no_std` 下必须提供；定义 panic 时行为（死循环、日志、复位等）。**不得**再假设 `std` 的默认 unwind/打印路径。
- **入口与启动**：裸机目标常使用 **`#![no_main]`** 并配合 **链接脚本 / 启动汇编 / `#[export_name]`** 等定义复位向量与初始化（具体因 **target** 与 **HAL** 而异；与 `std` 的 `lang_start → main` 路径不同）。

---

## 4. 底层内存与外设寄存器

外设 MMIO 寄存器可能在两次读之间被硬件改变；编译器若按普通内存做 **CSE / 省略**，会得到错误观测。

- **`core::ptr::read_volatile` / `write_volatile`**：对给定地址做 **volatile** 访问，约束编译器对该读写的优化语义。
- **注意**：volatile **不等于**完整的多核/设备 **内存排序** 模型；强顺序或屏障需求需结合 **ISA / 外设手册** 与 **`atomic::*`** 等（超出「仅 volatile」）。

---

## 5. 防误用的硬件抽象（类型状态）

用 **ZST + `PhantomData`**（或消费 `self` 的状态转移 API）把 **非法硬件组合** 变成 **不可类型构造**：

- 例：继电器/时钟域等 **互斥配置** 用 `Pair<StateA, StateB>` 编码；切换仅通过 **消耗旧状态、返回新状态** 的方法链。
- **零成本**：理想下无额外字段；错误在 **编译期** 被拒绝。

---

## 6. 交叉编译 (Cross-Compilation)

- **Target triple**（如 `thumbv7m-none-eabi`）选择指令集、ABI、是否带 OS。
- **自定义 `target-spec` JSON**：冷门平台可向 `rustc` 提供 JSON 描述（字段以官方文档为准）。
- **工具链**：`rustup target add ...`、链接器/运行库由平台生态（如 `cortex-m-rt`）补齐。

---

## 小结

抽掉 `std` 后，**所有权与类型状态**仍可把大量错误前移到编译期；代价是亲自处理 **panic、启动、堆（可选）、volatile/排序** 与 **交叉构建**。本章给出的是 **系统级 Rust** 的路线图，而非单一板卡的 API 手册。

---

## 与仓库目录的对应

- 原书章名：**Rust Without the Standard Library**  
- 本仓库文件夹：`RFR/Chapter-12-Rust-Without-Standard-Library/`  
- 全书总目录：`RFR/RFR-本书目录.md`
