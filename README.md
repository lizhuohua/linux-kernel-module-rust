# Writing Linux Device Drivers in Rust

## Requirements

* Rust nightly

    Tested on `nightly-2018-10-01-x86_64-unknown-linux-gnu`

* Linux Kernel Headers

    A pre-built kernel (with configuration and header files) is needed. Your Linux distribution should provide a package for this.

## Build

1. `cargo-xbuild` and `rust-src`
```bash
$ cargo install cargo-xbuild
$ rustup component add --toolchain=nightly rust-src
```
2. Select an example
```bash
$ cd hello_world
```
3. Compile into a static library
```
$ RUST_TARGET_PATH=$(pwd)/.. cargo xbuild --target x86_64-linux-kernel-module
```
4. Link as a kernel module
```bash
$ make
```

## Load and Test
```bash
$ sudo insmod helloworld.ko
$ sudo rmmod helloworld
$ dmesg
```
