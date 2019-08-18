#![no_std]
#![feature(alloc)]

extern crate alloc;

use crate::alloc::boxed::Box;
use crate::alloc::vec; // import vec! macro
use core::mem;
use linux_kernel_module::bindings;
use linux_kernel_module::c_types;
use linux_kernel_module::println;
use rlibc;

const ETH_FRAME_LEN: u32 = 1514;

struct USBDriver {
    _driver_info: Box<bindings::driver_info>,
    _table: Box<[bindings::usb_device_id]>,
    // driver must be stored in a box, why?
    driver: Box<bindings::usb_driver>,
}

struct Smsc9512Priv {
    dev: *mut bindings::usbnet,
}

unsafe extern "C" fn smsc9512_bind(
    dev: *mut bindings::usbnet,
    intf: *mut bindings::usb_interface,
) -> c_types::c_int {
    println!("SMSC9512: into smsc9512_bind");
    println!("before constructing pdata");
    let pdata = Box::new(Smsc9512Priv { dev: dev });
    println!("after constructing pdata");
    let ret = bindings::usbnet_get_endpoints(dev, intf);
    if ret < 0 {
        println!("usbnet_get_endpoints failed: {}", ret);
        return ret;
    }
    (*dev).data[0] = Box::into_raw(pdata) as u32;

    smsc9512_init_mac_address(dev);

    (*(*dev).net).flags |= 1 << 12; // IFF_MULTICAST
    (*(*dev).net).hard_header_len += 12; // SMSC95XX_TX_OVERHEAD_CSUM
    (*(*dev).net).min_mtu = 68; // ETH_MIN_MTU
    (*(*dev).net).max_mtu = 1500; // ETH_DATA_LEN
    (*dev).hard_mtu = (*(*dev).net).mtu + (*(*dev).net).hard_header_len as u32;

    0
}

fn smsc9512_init_mac_address(dev: *mut bindings::usbnet) {
    let mac_addr = unsafe { bindings::of_get_mac_address((*(*dev).udev).dev.of_node) };
    if mac_addr as usize != 0 {
        unsafe {
            rlibc::memcpy((*(*dev).net).dev_addr, mac_addr as *const u8, 6);
        }
        println!("MAC got from device tree");
        return;
    }
    println!("Error in smsc9512_init_mac_address");
}

unsafe extern "C" fn smsc9512_unbind(
    dev: *mut bindings::usbnet,
    _intf: *mut bindings::usb_interface,
) {
    println!("into unbind()");
    let _pdata = Box::from_raw((*dev).data[0] as *mut Smsc9512Priv);
    // Smsc9512Priv struct should be freed here
    println!("exit unbind()");
}

unsafe extern "C" fn smsc9512_rx_fixup(
    dev: *mut bindings::usbnet,
    skb: *mut bindings::sk_buff,
) -> c_types::c_int {
    if (*skb).len < (*(*dev).net).hard_header_len as u32 {
        return 0;
    }

    while (*skb).len > 0 {
        let mut header: u32 = 0;
        rlibc::memcpy(
            &mut header as *mut u32 as *mut u8,
            (*skb).data,
            mem::size_of::<u32>(),
        );
        bindings::skb_pull(skb, 4 + 2);
        let packet = (*skb).data;

        // get the packet length
        let size = (header & 0x3FFF0000) >> 16;
        let align_count = (4 - ((size + 2) % 4)) % 4;

        if header & 0x00008000 != 0 {
            // Error Summary
            println!("Error header = 0x{:X}", header);
            (*(*dev).net).stats.rx_errors += 1;
            (*(*dev).net).stats.rx_dropped += 1;

            if header & 0x00000002 != 0 {
                // CRC Error
                (*(*dev).net).stats.rx_crc_errors += 1;
            } else {
                if header & (0x00000080 | 0x00000800) != 0 {
                    (*(*dev).net).stats.rx_frame_errors += 1;
                }

                if (header & 0x00001000 != 0) && (header & 0x00000020) == 0 {
                    // Length Error
                    (*(*dev).net).stats.rx_length_errors += 1;
                }
            }
        } else {
            if size > (ETH_FRAME_LEN + 12) {
                println!("size err header = 0x{:X}", header);
                return 0;
            }

            // last frame in this batch
            if (*skb).len == size {
                bindings::skb_trim(skb, (*skb).len - 4);
                return 1;
            }

            let ax_skb = bindings::skb_clone(skb, 1);

            if ax_skb as u32 == 0 {
                println!("Error allocating skb");
                return 0;
            }

            (*ax_skb).len = size;
            (*ax_skb).data = packet;
            skb_set_tail_pointer_wrapper(ax_skb, size as i32);

            bindings::skb_trim(ax_skb, (*ax_skb).len - 4);
            bindings::usbnet_skb_return(dev, ax_skb);
        }
        bindings::skb_pull(skb, size);

        if (*skb).len != 0 {
            bindings::skb_pull(skb, align_count);
        }
    }
    1
}

extern "C" {
    fn skb_set_tail_pointer_wrapper(skb: *const bindings::sk_buff, offset: c_types::c_int);
}

unsafe extern "C" fn smsc9512_tx_fixup(
    _dev: *mut bindings::usbnet,
    skb: *mut bindings::sk_buff,
    _flags: bindings::gfp_t,
) -> *mut bindings::sk_buff {
    bindings::skb_push(skb, 4);
    let tx_cmd_b = (*skb).len - 4;
    rlibc::memcpy((*skb).data, &tx_cmd_b as *const u32 as *const u8, 4);

    bindings::skb_push(skb, 4);
    let tx_cmd_a = ((*skb).len - 8) | 0x00002000 | 0x00001000;
    rlibc::memcpy((*skb).data, &tx_cmd_a as *const u32 as *const u8, 4);

    skb
}

impl USBDriver {
    fn register(name: &'static str) -> linux_kernel_module::KernelResult<Self> {
        // Construct driver_info structure
        println!("before constrcuting driver_info");
        let mut smsc9512_info = Box::new(bindings::driver_info::default());
        println!("after constrcuting driver_info");
        smsc9512_info.description = "smsc9512 USB 2.0 Ethernet\0".as_bytes().as_ptr() as *mut i8;
        smsc9512_info.bind = Some(smsc9512_bind);
        smsc9512_info.unbind = Some(smsc9512_unbind);
        smsc9512_info.rx_fixup = Some(smsc9512_rx_fixup);
        smsc9512_info.tx_fixup = Some(smsc9512_tx_fixup);

        // Construct an array of usb_device_id
        let mut products = bindings::usb_device_id::default();
        products.match_flags = 3;
        products.idVendor = 0x0424;
        products.idProduct = 0xec00;
        products.driver_info = &(*smsc9512_info) as *const _ as c_types::c_ulong;
        println!("driver_info = {:X}", products.driver_info);
        println!("before constrcuting table");
        let mut table = vec![products, unsafe { mem::zeroed() }].into_boxed_slice();
        println!(
            "usb_device_id = {:X}",
            &table[0] as *const _ as c_types::c_ulong
        );
        println!("after constrcuting table");

        // Construct usb_driver structure
        println!("before constrcuting usb_driver");
        let mut driver = Box::new(bindings::usb_driver::default());
        println!("after constrcuting usb_driver");
        driver.name = name.as_bytes().as_ptr() as *const i8;
        driver.id_table = table.as_mut_ptr();
        driver.probe = Some(bindings::usbnet_probe);
        driver.disconnect = Some(bindings::usbnet_disconnect);

        unsafe {
            bindings::usb_register_driver(
                driver.as_mut(),
                &mut bindings::__this_module,
                "smsc9512\0".as_bytes().as_ptr() as *const i8,
            );
        }

        Ok(USBDriver {
            _driver_info: smsc9512_info,
            _table: table,
            driver: driver,
        })
    }
}

struct USBModule {
    usb: USBDriver,
}

impl linux_kernel_module::KernelModule for USBModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        println!("Hello from Rust!");
        let usb = USBDriver::register("smsc9512\0")?;
        Ok(USBModule { usb: usb })
    }
}

impl Drop for USBModule {
    fn drop(&mut self) {
        println!("Goodbye from Rust!");
        unsafe {
            bindings::usb_deregister(self.usb.driver.as_mut());
        }
    }
}

static mut MODULE: Option<USBModule> = None;

#[no_mangle]
pub extern "C" fn init_module() -> c_types::c_int {
    match <USBModule as linux_kernel_module::KernelModule>::init() {
        Ok(m) => {
            unsafe {
                MODULE = Some(m);
            }
            return 0;
        }
        Err(_e) => {
            return 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    unsafe {
        MODULE = None;
    }
}

#[no_mangle]
#[link_section = ".modinfo"]
pub static MODINFO: [u8; 12] = *b"license=GPL\0";
