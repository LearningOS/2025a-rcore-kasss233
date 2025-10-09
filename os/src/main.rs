#![no_std]
#![no_main]
mod batch;
mod console;
mod lang_items;
mod sbi;
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

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
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}
