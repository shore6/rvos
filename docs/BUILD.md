# 環境構築とビルド手順

ワークフロー：[WORKFLOW.md](WORKFLOW.md) ／ 設計判断：[DECISIONS.md](DECISIONS.md)

ツールは前提知識ゼロとして書く。コマンドはそのまま打てばよい。
「（M0 で確定）」と書いた部分は、M0 で実際に動かしてから書き換える。

---

## 1. WSL2 の準備（D-007）

リポジトリは Linux 側のホームディレクトリ配下に置く。今回は、`~/project/rvos/` に置いた。

`/mnt/c/...`（Windows の C ドライブ）の下には置かない。ビルドが大幅に遅くなる。

## 2. パッケージの導入

```bash
sudo apt update
sudo apt install -y build-essential git qemu-system-misc gdb-multiarch device-tree-compiler
```

| パッケージ | 用途 |
| --- | --- |
| `build-essential` | リンカなど、Rust のビルドが内部で使う道具 |
| `qemu-system-misc` | `qemu-system-riscv64`（RISC-V のエミュレータ） |
| `gdb-multiarch` | RISC-V のプログラムも扱えるデバッガ |
| `device-tree-compiler` | `dtc`。QEMU の DTB を読める形に変換する |

確認：

```bash
qemu-system-riscv64 --version
```

OpenSBI が動くかを確かめる。カーネルはまだないので、OpenSBI のバナーが出たあと止まれば成功である。

```bash
qemu-system-riscv64 -machine virt -nographic -bios default
```

QEMU を抜けるには **Ctrl-A を押して離し、続けて X** を押す。
バナーが出ず、ファームウェアが見つからないというエラーが出た場合は、`opensbi` パッケージが入っているかを確認する。

## 3. Rust の導入

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup target add riscv64gc-unknown-none-elf
rustup component add llvm-tools clippy
cargo install cargo-binutils
```

| 導入するもの | 用途 |
| --- | --- |
| `riscv64gc-unknown-none-elf` | OS なしの RISC-V 64 向けにビルドするためのターゲット |
| `llvm-tools` + `cargo-binutils` | `rust-objdump`、`rust-nm`、`rust-objcopy`。生成されたバイナリを調べる |
| `clippy` | 警告より一段踏み込んだ静的検査（WORKFLOW 5.2） |

ツールチェーンのバージョンは、M0 でリポジトリの `rust-toolchain.toml` に固定する（D-009）。
固定したあとは、リポジトリの中で `cargo` を打つと、その版が自動で使われる。

## 4. ビルドと実行（M0 で確定）

予定しているコマンド。名前と中身は M0 で `.cargo/config.toml` のエイリアスとして定義する。

| コマンド | 内容 |
| --- | --- |
| `cargo krun` | カーネルを RISC-V 向けにビルドし、QEMU で起動する |
| `cargo ktest` | カーネルの統合テストを QEMU 上で実行し、終了コードで結果を返す |
| `cargo test -p klib` | `klib` をホスト（x86_64 の Linux）向けにビルドしてテストする |
| `cargo clippy` | 静的検査 |

`kernel` と `klib` はビルド先のターゲットが違う。
`kernel` は常に `riscv64gc-unknown-none-elf`、`klib` のテストはホスト向けである。
これをどう設定に落とすか（エイリアスでターゲットを指定する、など）は M0 で決める。

QEMU の起動オプションの基本形（SPEC 6 で確定させる）：

```bash
qemu-system-riscv64 -machine virt -smp 1 -m 128M -nographic -bios default -kernel <カーネルの ELF>
```

## 5. デバッグ

### 5.1 QEMU のログ

トラップや不正なアクセスを QEMU に記録させる。

```bash
qemu-system-riscv64 ... -d int,guest_errors -D qemu.log
```

`-d int` はすべての例外と割り込みを記録する。タイマー割り込みを有効にしたあとはログが膨大になるので、必要なときだけ付ける。

### 5.2 gdb

QEMU を、gdb の接続を待つ状態で止めて起動する。

```bash
qemu-system-riscv64 ... -s -S
```

`-s` は TCP の 1234 番で gdb の接続を受け付ける。`-S` は最初の命令を実行する前に止まる。

別のターミナルで：

```bash
gdb-multiarch <カーネルの ELF> -ex 'target remote :1234'
```

よく使う操作：

| gdb のコマンド | 意味 |
| --- | --- |
| `b *0x80200000` | そのアドレスで止める |
| `b kmain` | その関数で止める |
| `c` | 続行 |
| `si` | 1 命令だけ進める |
| `x/8i $pc` | 現在地から 8 命令を逆アセンブルする |
| `info registers` | 汎用レジスタを表示する |
| `p/x $scause` | CSR を 16 進で表示する |

### 5.3 バイナリを調べる

```bash
rust-objdump -d <カーネルの ELF> | less     # 逆アセンブル
rust-nm -n <カーネルの ELF>                 # シンボルをアドレス順に並べる
rust-objdump -h <カーネルの ELF>            # セクションの配置
```

### 5.4 QEMU のハード構成を確かめる

```bash
qemu-system-riscv64 -machine virt,dumpdtb=virt.dtb -smp 1 -m 128M
dtc -I dtb -O dts virt.dtb -o virt.dts
```

`virt.dts` に、メモリの範囲や UART・PLIC のアドレスが書かれている。SPEC 2 の表はこれと照らし合わせる。
