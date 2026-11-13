#![no_std]
#![no_main]

core::arch::global_asm!(include_str!("entry.S"));
use core::panic::PanicInfo;

#[panic_handler]
fn kpanic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain(_hartid: usize, _dtb: usize) -> ! {
    loop {}
}
