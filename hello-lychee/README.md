# Hello, Lychee!

A dependency-free Rust CLI for the Sipeed LicheeRV Nano running RISC-V Linux.

## Cross-compile on the host

```sh
cd /disk1/github/hardwarewrighter/LycheeRV-Nano/hello-lychee
rustup target add riscv64gc-unknown-linux-musl
cargo build --release --offline
```

`.cargo/config.toml` selects `riscv64gc-unknown-linux-musl`, uses Rust's bundled
`rust-lld` linker, and requests static C-runtime linking. No vendor C compiler
is needed for this pure Rust example. Crates containing native C/C++ dependencies
may need a compatible cross-toolchain. Run Cargo from this directory so it finds
the project configuration.

The target is [distributed through rustup](https://doc.rust-lang.org/rustc/platform-support/riscv64gc-unknown-linux-musl.html).
The generic GC instruction set avoids requiring vector extensions. The Nano's
observed CPU reports `rv64imafdvcsu`; its running OS is `riscv64` with musl.

Output: `target/riscv64gc-unknown-linux-musl/release/hello-lychee`.

## Copy and run

From this directory on the host:

```sh
scp target/riscv64gc-unknown-linux-musl/release/hello-lychee root@10.105.250.1:/root/hello-lychee
ssh root@10.105.250.1 'chmod +x /root/hello-lychee && /root/hello-lychee'
```

Factory password: `root`, unless changed. Modern SCP's default SFTP transport
was tested successfully with this image. `/root` is on the SD card and persists
across reboots. Updating the executable requires no OS reflash.

Expected output:

```text
Hello, Lychee!
Rust CLI running on riscv64 Linux
```

For an interactive session:

```sh
ssh root@10.105.250.1
/root/hello-lychee --help
/root/hello-lychee --version
```

## Validation

On 2026-09-28, Rust 1.88.0 built the executable successfully. `file` identified
it as a stripped, statically linked 64-bit RISC-V executable with the double-float
ABI. `cargo fmt --check` passed.

The binary was copied with SCP to `/tmp/hello-lychee` on the actual Nano and run
through SSH. Greeting, help, and version succeeded; an invalid argument returned
exit code 2. The temporary copy may be removed on reboot. The commands above
install your own persistent copy.
