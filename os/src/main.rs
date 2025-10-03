// os/src/main.rs
#![no_std]
#![no_main]
mod console;
mod lang_items;
mod sbi;
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    unsafe {
        clear_bss();
    }
    println!("Hello, world!");
    panic!("Shutdown machine!");
}
unsafe fn clear_bss() {
    unsafe extern "C" {
        unsafe fn sbss();
        unsafe fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
