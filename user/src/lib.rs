#![no_std]
#![feature(linkage)]
#[macro_use]
pub mod console;
mod lang_items;
mod syscall;
use syscall::*;
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}
#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Cannot find main!");
}
fn clear_bss() {
    unsafe extern "C" {
        unsafe static sbss: u8;
        unsafe static ebss: u8;
    }
    unsafe {
        let sbss_ptr = sbss as *mut u8;
        let ebss_ptr = ebss as *const u8;
        let bss_size = ebss_ptr as usize - sbss_ptr as usize;
        core::ptr::write_bytes(sbss_ptr, 0, bss_size);
    }
}
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
