# 第 11 章：外部函数接口 (FFI) —— 深度解析

《Rust for Rustaceans》第 11 章讨论 Rust 如何与 **C / C++** 等语言共享地址空间与调用约定。FFI 边界几乎是 **安全 Rust 与 `unsafe` 的分水岭**；本章目标是：**在不安全的外部代码之上，用类型与生命周期封装出地道、难误用的安全 API**。

---

## 1. 跨越边界：`extern` 与符号

- **`extern "C" { ... }`**：声明在其他翻译单元（常为 C）中定义的函数或 `static`；ABI 多为 **C 调用约定**（`extern` 块默认与 `"C"` 的精确关系以参考手册为准）。
- **`#[no_mangle]`**：向 C 暴露 Rust 符号时禁用 **name mangling**，便于链接器按约定名解析。
- **链接属性**：`#[link(name = "crypto")]` 等指定库名；`#[link_name = "..."]` / `#[export_name = "..."]` 用于与外部命名不一致时的符号映射。

---

## 2. 调用约定与 Panic / Unwind

- **`extern "C"`**：FFI 事实标准 ABI；参数与寄存器保存规则与 C 对齐。
- **禁止让 unwind 穿过 FFI 边界**：若 Rust 函数可能 **panic**，且 unwinding 进入 **C 一侧**，在 Rust 规则下属于 **UB**（应使用 **abort** 策略或边界上 **catch**）。

**实践**：对导出给 C 的 `extern "C"` 入口，常用 **`catch_unwind` + 错误码** 将失败映射为 C 可处理的整型 / 哨兵，并文档化线程与重入语义。

---

## 3. 跨语言类型映射

Rust 在机器码层**无保留类型信息**；两侧必须对**布局与 ABI** 有相同理解。

- **整型与别名**：用 **`std::os::raw`**（或 **`libc`**）中的 `c_int`、`c_char` 等对齐 C 头文件，避免猜测 `int` 宽度。
- **`#[repr(C)]`**：与 C 交换的 `struct` 必须固定布局；默认 `repr(Rust)` **不保证**字段顺序。
- **带负载枚举**：`#[repr(C, u8)]` 等下，可对应 **tagged union** 心智模型；具体 repr 规则以参考手册为准。
- **可空指针与 niche**：许多平台上的 **null 指针** 可映射为 **`Option<NonNull<T>>` / `Option<extern "C" fn(...)>`** 等，利用 niche 保持指针宽度。

---

## 4. 内存：谁分配谁释放

- **库内分配**：C 侧 `xxx_new` / `xxx_free`；Rust 包装在 **`Drop`** 中调用 `free`，避免泄漏与双重释放。
- **调用方缓冲区**：Rust 分配 buffer + 长度交给 C **填充**；需明确写入边界与初始化要求。

---

## 5. 安全封装模式

- **生命周期编码不变式**：若 C 文档要求「`Device` 不得长于 `Context`」，可用 **`Device<'a>`** 持有 **`&'a Context`** 或等价约束，让编译器 enforce 使用顺序。
- **`Send` / `Sync`**：不要默认 **`unsafe impl Send` / `unsafe impl Sync`**。若 C 侧**非线程安全**，应通过 **API 形态**（例如仅在 **`!Send`** 的 runtime / 句柄上暴露操作）或持有 **`!Send` 的真实字段**（如 `std::rc::Rc<()>` 等，会有相应语义与成本）来防止句柄进入 **`Send` Future**。注意：**`PhantomData<T>` 不参与 `Send`/`Sync` 的自动推导**（不“继承”`T` 的线程性质），不能指望单靠它撤销 `Send`；以 [`PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) 文档为准。
- **消灭裸 `*mut c_void`**：为不同不透明句柄定义 **ZST 标记类型**（如 `pub struct Foo(c_void);`），用 **`*mut Foo`** 区分 A/B 指针，让类型系统拦截混传。

---

## 6. 工程化工具

- **`bindgen`**：从头文件生成 Rust FFI 绑定。
- **`*-sys` crate**：裸绑定与版本钉扎放在 `foo-sys`，安全封装放在 `foo`，便于独立升级与审计。
- **`cbindgen`**：由 Rust 导出 C 头文件。
- **C++**：名称修饰、异常、构造与模板使「手写 C ABI」脆弱；生态上常用 **`cxx`** 等桥梁 crate 做类型安全绑定（以项目与平台为准）。

---

## 小结

FFI 不只是 `extern`。优秀封装需要：**正确 ABI/repr**、**明确所有权与释放**、**panic/unwind 边界**、**用生命周期与新型别擦除 `void*`**，并在必要时用 **`PhantomData`** 表达 **线程与不变式**。工具链上 **`bindgen` + `*-sys` + 安全层** 是主流分工。

---

## 与仓库目录的对应

- 原书章名：**Foreign Function Interfaces**  
- 本仓库文件夹：`RFR/Chapter-11-Foreign-Function-Interfaces/`  
- 全书总目录：`RFR/RFR-本书目录.md`
