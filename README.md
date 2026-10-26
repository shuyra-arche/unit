# unit

Shuyra 的纯 Unit 声明库：`UnitFile` / `Deployment` 声明与通用校验。
它从公开基线 `a6c6824ef91ac487919d87374a71b62174d465bd` 的
`crates/unit` 提取，不是具体 Unit server。

## 范围

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
