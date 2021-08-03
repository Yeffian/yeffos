// Since this is a operating system, we can't use anything from the rust std as most of 
// the stuff there uses or interacts with your host os in some way.
#![no_std] 
// Tell Rust we don't want to use the standard entry point.
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic.
// Since this function can never return, it returns the "never" type: !.
#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

// The _start function is called when the C runtimes invokes the Rust runtime. 
// In other words, _start is the entry point or the Rust runtime.
// We are overwriting the OS entry point with our own.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
