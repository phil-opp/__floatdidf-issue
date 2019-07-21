#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    foo(1);

    loop {}
}

fn foo(x: isize) -> f64 {
    5 as f64 / x as f64
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
