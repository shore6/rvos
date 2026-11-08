# rvos

Rust で書く、RISC-V 64 向けの教育用カーネル（学習・趣味のプロジェクト）。

QEMU の `virt` マシンで動き、OpenSBI から S モードで起動する。
[xv6-riscv](https://github.com/mit-pdos/xv6-riscv) の設計を参考にしているが、コードは移植していない。
目標は、ユーザープロセス、システムコール、簡易シェルまでを作ることである。

## 状態

進み具合は [docs/ROADMAP.md](docs/ROADMAP.md) を参照。

## 必要なもの

- Rust（stable）と `riscv64gc-unknown-none-elf` ターゲット
- `qemu-system-riscv64`

導入手順は [docs/BUILD.md](docs/BUILD.md) を参照。

## 使い方

```bash
cargo krun           # QEMU でカーネルを起動する
cargo ktest          # QEMU 上で統合テストを実行する
cargo test -p klib   # ハードに依存しない部分をホストでテストする
```

## ドキュメント

- [SPEC.md](docs/SPEC.md)：仕様
- [ROADMAP.md](docs/ROADMAP.md)：開発ロードマップ
- [DECISIONS.md](docs/DECISIONS.md)：設計判断の記録
- [BUILD.md](docs/BUILD.md)：環境構築とビルド
