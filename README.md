# curtOS

Based on the tutorial 'Writing an OS in Rust' by Philipp Oppermann [(link)](https://os.phil-opp.com/)

![Screenshot of a QEMU VGA screen showing the curtOS splash screen](https://github.com/curtis-wils0n/curt_os/assets/60797928/721211e7-a531-403d-8525-ff25b9296c1f)

curtOS is a simple x86-based operating system written in Rust and named after the nerd who wrote it.

## Setup

1. Make sure your rustup is set to the nightly release by running `rustup default nightly`.
2. To recompile the libraries, cargo needs access to the rust source code. Run `rustup component add rust-src`
3. Install bootimage with `cargo install bootimage`
4. Add llvm-tools-preview with `rustup component add llvm-tools-preview`
5. Run `cargo bootimage`
6. You can now run the bootimage in QEMU by running `PATH/TO/qemu-system-x86_64 -drive format=raw,file=target/x86_64-curt_os/debug/bootimage-curt_os.bin`. If you configure Cargo.toml's `run-command` to point to your QEMU installation, you can just run `cargo run` instead.
