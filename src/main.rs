// disable standard library
#![no_std]
// don't use normal entry point chain
#![no_main]

// panic_handler defines the function the compiler should invoke when a panic occurs
// since we disabled standard library, we need to define it ourselves
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}