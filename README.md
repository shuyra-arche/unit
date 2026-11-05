# unit

Shuyra 的纯 Unit 声明、二进制记录和运行时策略库。
它从公开基线 `a6c6824ef91ac487919d87374a71b62174d465bd` 的
`crates/unit` 提取，不是具体 Unit server。

## 范围

当前接口由宿主与 System 共享：

- `document::Unit<String>` 读取严格的 `unit.toml`；`record::encode/decode`
  使用带版本的二进制记录。解码字符串借用不可变 initrd，数组由解码结果持有。
- `runtime::UnitFile` 持有原生策略数组，可通过 `runtime::Unit`（`Arc`）共享，
  不泄漏内存或生成静态登记。
- `control.creation = "startup" | "requested"` 表达实例创建时机；两种声明都必须
  随必要依赖入包。`requires` 决定入包闭包，`after` 表达已选实例的就绪约束。
- 完整 u64 数值可使用普通非负整数、带引号的十进制或 `0x` 十六进制字符串；
  TOML 本身不能表达超出 i64 的裸整数，二进制中仍保持原生 u64。
- `event` 提供带版本的类型化 System 观察帧；业务测试继续使用原生任务退出码。

旧 `UnitFile` / `Deployment` 类型暂供尚未迁移的调用者使用；新的 Deployment
属于宿主单次调用的内存对象，不存为部署文件。

- 只含 unit 声明类型与通用校验；不含 server / client / execution / programs。
- 不持有全部 Units 的值、`PROGRAMS` / `DEPLOYMENTS` 注册表，也不从
  composition 复制实际 `UnitFile` 值。
- 不是完整的 stage3 运行闭环：没有运行时装配、镜像构建或部署执行。

## 依赖

- `env` — <https://github.com/shuyra-arche/env.git>
  `d3000253cd5d13d466e91fafd4dbb27a5ec5aaae`
- `system-api` — <https://github.com/shuyra-arche/system.git>
  `9e5d63ca536a176583cf1b0054b4ecf7ddd88257`
- `wire` 与 `mold` 仅经上述公开依赖传递固定：
  Wire `0ff77dc4573fa2620d2fe995a18c01949d9c7e57`，
  Mold `f52501f4c91b686eded8b4bcd743b4e96084b7f9`。

`Cargo.lock` 是独立锁，由 source 根已验收锁经实际 `cargo metadata`
最小裁剪得到，保留已验收 registry 版本（例如 bitflags 2.13.1、
unicode-ident 1.0.24），未做无差别升级。

## 验证

```sh
cargo check --target x86_64-unknown-linux-gnu --locked
cargo check --target riscv64gc-unknown-none-elf --locked
cargo test --tests --target x86_64-unknown-linux-gnu --locked
```

## 来源、作者与许可

路径映射与逐字节清单见 [PROVENANCE.md](PROVENANCE.md)。
源基线由 `anreonyr <anreonyr@gmail.com>` 提交；源未声明许可证，
本仓不添加许可证。
