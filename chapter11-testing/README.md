# Chapter 11: Test Organization

这个项目展示 Rust 中三类测试的组织方式：

```text
chapter11-testing/
├── src/
│   ├── lib.rs                 # 公开 API、私有实现、单元测试、文档测试
│   └── main.rs                # 很薄的二进制入口
└── tests/
    ├── common/mod.rs          # 集成测试共享辅助代码
    └── checkout_workflow.rs   # 集成测试
```

## 运行与观察

运行全部测试：

```console
cargo test
```

输出会依次显示：

1. `src/lib.rs` 中的单元测试
2. `src/main.rs` 的测试目标（其中没有测试）
3. `tests/checkout_workflow.rs` 中的集成测试
4. `src/lib.rs` 文档注释中的文档测试

只运行一个集成测试文件：

```console
cargo test --test checkout_workflow
```

按测试名称过滤：

```console
cargo test discount
```

## 核心规则

- 单元测试通常放在被测代码所在文件的 `#[cfg(test)] mod tests` 中。
- `#[cfg(test)]` 让测试模块只在测试配置下参与编译。
- 测试模块是被测模块的子模块，所以 `use super::*` 后可以访问私有项。
- `tests/` 顶层的每个 `.rs` 文件都是独立 crate，只能使用库的公开 API。
- `tests/common.rs` 会被 Cargo 当成独立的集成测试 crate；使用
  `tests/common/mod.rs` 可以避免产生一个多余的测试目标。
- 集成测试不能导入二进制 crate 的 `main.rs`。把核心逻辑放在 `lib.rs`，
  再让 `main.rs` 调用它。

## 练习

1. 在 `src/lib.rs` 添加私有函数 `shipping_fee`，并为它写单元测试。
2. 让 `checkout_total` 使用 `shipping_fee`，再更新集成测试中的预期结果。
3. 尝试从 `tests/checkout_workflow.rs` 调用 `calculate_subtotal`，阅读编译错误。
4. 临时把 `tests/common/mod.rs` 移成 `tests/common.rs`，观察 `cargo test` 输出的变化。
