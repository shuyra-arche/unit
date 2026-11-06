# unit

Shuyra 的纯 Unit 声明、二进制记录和运行策略库，由宿主构建工具与 System 共享。
本仓不提供 Unit server 或具体产品的全局注册表。

- `src/document/`：严格的 `unit.toml` 文档模型。
- `src/record/`：带版本的二进制编码与借用解码。
- `src/runtime/`：持有策略数组的 `UnitFile`，通过 `Arc` 共享。
- `src/event/`：类型化 System 观察帧。
- `src/construction/`、`src/deployment/`：构造与选择策略；旧接口仍供既有消费者使用。
- `tests/`：记录、策略、帧与声明测试。

`requires` 决定入包闭包；`after` 约束已选实例的就绪顺序。
`control.creation` 可为 `startup` 或 `requested`，两者都必须随必要依赖入包。
完整 u64 可写为非负整数或带引号的十进制/十六进制字符串；TOML 裸整数受 i64 限制。
解码字符串借用不可变 initrd，数组由结果持有。Deployment 是宿主单次调用的内存对象。

## 工具链与维护

使用 `rust-toolchain.toml` 固定的 `nightly-2026-09-04`，首次运行需要 Rustup
下载工具链及 RV64 目标。依赖版本以 Cargo 清单和锁文件为准。
下列命令从仓库根目录执行；宿主测试不代表真实内核运行验证。

```sh
cargo check --target x86_64-unknown-linux-gnu --locked
cargo check --target riscv64gc-unknown-none-elf --locked
cargo test --tests --target x86_64-unknown-linux-gnu --locked
```

业务测试使用原生退出码；System 观察帧描述退出与回收，不能替代实际任务执行。

## 贡献与来源

维护与 PR 规则见 [Shuyra 贡献规范](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md)。
来源、作者与许可情况见 [PROVENANCE.md](PROVENANCE.md)。
