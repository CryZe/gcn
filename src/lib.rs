#![no_std]

extern crate arrayvec;

pub mod gx;
pub mod os;

use arrayvec::ArrayString;
use core::fmt::Write;
use core::panic::PanicInfo;

pub fn report_panic(info: &PanicInfo) {
    let mut message = ArrayString::<[u8; 1024]>::new();
    write!(message, "{}\0", info).ok();
    unsafe {
        os::report(message.as_ptr());
    }
}
