# RCore 内核编译与调试指南
### 1. 编译内核

```bash
cargo build --release
```

### 2. 使用 `objcopy` 将 ELF 转成 BIN
```bash
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os \
-O binary target/riscv64gc-unknown-none-elf/release/os.bin
```

### 3. 使用 QEMU 启动内核

```bash
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000
```

---

## GDB 调试

### 1. 启动 QEMU 并等待 GDB 连接

```bash
qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S
```

* `-s`：在端口 `1234` 上开启 GDB 服务器
* `-S`：启动时暂停 CPU，等待 GDB 连接

### 2. 启动 GDB 并连接 QEMU

```bash
gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
```

---

## 常用 GDB 指令

* `x/10i 0x80000000` ：显示 `0x80000000` 处的 10 条汇编指令
* `x/10i $pc` ：显示程序计数器 `$pc` 处即将执行的 10 条汇编指令
* `x/10xw 0x80000000` ：显示 `0x80000000` 处的 10 条 32-bit 数据（16 进制）
* `info registers` ：显示当前所有寄存器信息
* `info r t0` ：显示寄存器 `t0` 的值
* `break funcname` ：在目标函数第一条指令设置断点
* `break *0x80200000` ：在地址 `0x80200000` 设置断点
* `c` ：继续执行直到遇到断点
* `si` ：单步执行一条汇编指令
