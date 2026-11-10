#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn kpanic(_info: &PanicInfo) -> ! {
    loop {}
}
