A simple, proof-of-concept OS running on RISC-V, written in Rust. The goal of this project is to write as much code in Rust as possible.

Special thanks to Stephen Marz's [awesome blog](https://osblog.stephenmarz.com/) which made this project possible. If you want to learn something, you should really try to read its source code.

## Building &amp; Running

You should have `qemu-system-riscv64` installed on your system in order to run this OS under an emulator. Use the following command to configure your QEMU build for RISC-V 64:

```bash
./configure --target-list=riscv64-softmmu
```

Run the following commands under the project root to configure the Rust toolchain:

```bash
rustup target add riscv64gc-unknown-none-elf
rustup override set nightly
```

The project defaults to the `riscv64gc-unknown-none-elf` target, as configured in `.cargo/config`. You'll also need the nightly toolchain, since this project uses unstable Rust features.

If you have `qemu-system-riscv64` installed, run the following command to run the OS under QEMU:

```bash
cargo run
```

The OS should work out-of-the-box.

## Credits

Some code in project is directly taken from [sgmarz/osblog](https://github.com/sgmarz/osblog), which is licensed under the [MIT License](https://github.com/sgmarz/osblog/blob/master/LICENSE).
