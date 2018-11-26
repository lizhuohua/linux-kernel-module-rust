# Writing Linux Device Drivers in Rust
[![Build Status](https://travis-ci.com/lizhuohua/linux-device-driver-rust.svg?token=gQ3MGp1DXsVespCpQBDg&branch=master)](https://travis-ci.com/lizhuohua/linux-device-driver-rust)
## Requirements

### Toolchain

* `x86_64`

    Rust nightly. Tested on `nightly-2018-11-02-x86_64-unknown-linux-gnu`.

* `ARMv7` (Raspberry Pi)

    Rust nightly. In addition,  you need to install the new target:
    ```bash
    $ rustup target add arm-unknown-linux-gnueabi
    ```
    And the `arm-linux-gnueabihf-` cross-compiler.

### Linux Kernel Headers

A pre-built kernel (with configuration and header files) is needed.

* `x86_64`

    Your Linux distribution should provide a package for this. For example, on Ubuntu, you can try:
    ```bash
    $ sudo apt-get install linux-headers-`uname -r`
    ```

* `ARMv7` (Raspberry Pi)

    You need to [compile your own kernel](https://www.raspberrypi.org/documentation/linux/kernel/building.md) in order for `bindgen` to work.

## Build

1. `cargo-xbuild`, `rust-src` and `rustfmt-preview`
```bash
$ cargo install cargo-xbuild
$ rustup component add --toolchain=nightly rust-src
$ rustup component add rustfmt-preview
```
2. Select an example
```bash
$ cd hello_world
```
3. Compile into a static library
    * `x86_64`
        ```bash
        $ RUST_TARGET_PATH=$(pwd)/.. cargo xbuild --target x86_64-linux-kernel-module
        ```
    * `ARMv7` (Raspberry Pi)
        ```bash
        $ RUST_TARGET_PATH=$(pwd)/.. KDIR=<path-to-your-compiled-kernel> cargo xbuild --target armv7l-linux-kernel-module
        ```
4. Link as a kernel module
    * `x86_64`
        ```bash
        $ make
        ```
    * `ARMv7` (Raspberry Pi)
        ```bash
        $ make TARGET=armv7l-linux-kernel-module KDIR=<path-to-your-compiled-kernel> CROSS=arm-linux-gnueabihf-
        ```
5. Load and test

    See below.
6. If you want to clean it up
    * `x86_64`
        ```bash
        $ make clean;cargo clean
        ```
    * `ARMv7` (Raspberry Pi)
        ```bash
        $ make clean TARGET=armv7l-linux-kernel-module KDIR=<path-to-your-compiled-kernel> CROSS=arm-linux-gnueabihf-;cargo clean
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
A simple character device which is similar to the `yes` Unix command.
```bash
$ sudo insmod yes_chardev.ko
$ cat /proc/devices # find the major number of the device 'yes', for example, 243
$ sudo mknod /dev/yes c 243 0 # make a filesystem node (replace 243 with your own major number)
$ sudo cat /dev/yes # read from the device
$ sudo rmmod yes_chardev
```

### simple_sysctl
A simple sysctl device driver.
```bash
$ sudo insmod simple_sysctl.ko
$ cat /proc/sys/rust/example/test # the default value should be 1
$ sudo sh -c "echo 2 > /proc/sys/rust/example/test" # change the value
$ cat /proc/sys/rust/example/test # now the value is 2
$ sudo rmmod simple_sysctl
```
There is another way to read/write the sysctl value:
```bash
$ sysctl rust.example.test # read
$ sudo sysctl -w rust.example.test=2 # write
```
