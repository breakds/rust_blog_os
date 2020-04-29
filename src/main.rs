#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

// Quote: By default, Rust uses unwinding to run the destructors of
// all live stack variables in case of a panic, unless, we specify the
// behavior in the profile of Cargo.toml.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // The linker looks for a function named "_start" by default
    use core::fmt::Write;
    println!("Hi!");
    println!("{} * {} * {} = {}", 47, 59, 71, 47 * 59 * 71);
    panic!("Try to be wrong.");
    
    loop {}
}
