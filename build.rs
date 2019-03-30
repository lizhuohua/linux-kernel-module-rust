extern crate bindgen;
extern crate cc;
extern crate shlex;

use std::env;
use std::path::PathBuf;
use std::process::Command;

const INCLUDED_TYPES: &[&str] = &["file_operations", "ctl_table", "spinlock_t", "mutex", "usb_driver", "usb_device_id", "driver_info"];
const INCLUDED_FUNCTIONS: &[&str] = &[
    "__register_chrdev",
    "__unregister_chrdev",
    "_copy_to_user",
    "register_sysctl",
    "unregister_sysctl_table",
    "proc_dointvec_minmax",
    "spin_lock",
    "usbnet_probe",
    "usbnet_disconnect",
    "usb_register_driver",
    "usb_deregister",
    "usbnet_get_endpoints",
    "of_get_mac_address",
    "skb_pull",
    "skb_push",
    "skb_trim",
    "skb_clone",
    "usbnet_skb_return",
];
const INCLUDED_VARS: &[&str] = &["__this_module", "THIS_MODULE"];

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("Target={}", target);
    let mut builder = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("c_types")
        .no_copy(".*")
        .derive_default(true)
        .rustfmt_bindings(true)
        .clang_arg(format!("--target={}", target));

    let output = String::from_utf8(
        Command::new("make")
            .arg("-C")
            .arg("kernel-cflags-finder")
            .arg("-s")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    Command::new("make")
        .arg("-C")
        .arg("kernel-cflags-finder")
        .arg("clean");

    println!("get output:{}", output);
    // These three arguments are not supported by clang
    // output = output.replace("-mapcs", "");
    // output = output.replace("-mno-sched-prolog", "");
    // output = output.replace("-mno-thumb-interwork", "");

    for arg in shlex::split(&output).unwrap() {
        builder = builder.clang_arg(arg.to_string());
    }

    println!("cargo:rerun-if-changed=src/bindgen_helper.h");
    builder = builder.header("src/bindgen_helper.h");

    for t in INCLUDED_TYPES {
        builder = builder.whitelist_type(t);
    }
    for f in INCLUDED_FUNCTIONS {
        builder = builder.whitelist_function(f);
    }
    for v in INCLUDED_VARS {
        builder = builder.whitelist_var(v);
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
