#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Quote: By default, Rust uses unwinding to run the destructors of
// all live stack variables in case of a panic, unless, we specify the
// behavior in the profile of Cargo.toml.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // The linker looks for a function named "_start" by default

    // Cast to a raw pointer.
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    
    loop {}
}
