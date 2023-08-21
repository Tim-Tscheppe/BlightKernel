// disable standard library
#![no_std]
// don't use normal entry point chain
#![no_main]

mod vga_buffer;

// panic_handler defines the function the compiler should invoke when a panic occurs
// since we disabled standard library, we need to define it ourselves
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Panic Test");
    loop {}
}

