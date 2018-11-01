# Writing Linux Device Drivers in Rust
[![Build Status](https://travis-ci.com/lizhuohua/linux-device-driver-rust.svg?token=gQ3MGp1DXsVespCpQBDg&branch=master)](https://travis-ci.com/lizhuohua/linux-device-driver-rust)
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

### hello_world
The simplest kernel module. It just prints "hello" and "goodbye".
```bash
$ sudo insmod helloworld.ko # load the module
$ sudo rmmod helloworld # remove the module
$ dmesg # dump kernel messages
```

### yes_chardev
A simple character device which is similar with the `yes` Unix command.
```bash
$ sudo insmod yes_chardev.ko
$ cat /proc/devices # find the major number of the device 'yes', for example, 243
$ sudo mknod /dev/yes c 243 0 # make a filesystem node (replace 243 with your own major number)
$ sudo cat /dev/yes # read from the device
$ sudo rmmod yes_chardev
```
