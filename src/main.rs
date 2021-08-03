// Since this is a operating system, we can't use anything from the rust std as most of 
// the stuff there uses or interacts with your host os in some way.
#![no_std] 
// Tell Rust we don't want to use the standard entry point.
#![no_main]

use core::panic::PanicInfo;

// The message to display on the screen.
static MESSAGE: &[u8] = b"Hello World!";

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
    // Reference to VGA Buffer.
    // Casting 0xb000 to a raw pointer.
    let vga_buffer = 0xb8000 as *mut u8;

    // Loop through the bytes of the static MESSAGE byte string.
    for (i, &b) in MESSAGE.iter().enumerate() {
        // Use the unsafe block to bypass all of Rust's memory saftey features.
        unsafe {
            *vga_buffer.offset(i as isize * 2) = b; // Write the string byte.
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // Write the colour byte.
        }
    }

    loop {}
}
