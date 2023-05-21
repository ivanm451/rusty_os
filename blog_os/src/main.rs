// we disable standard libraries to design our OS as close to the metal as possible

#![no_std] // don't link the Rust standard library
#![no_main] // tell compiler not to use normal entry point chain, since we aren't using std lib C
            // that helps us link to crt0 (c runtime event zero) 

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


static HELLO: &[u8] = b"Hello World!";

// removed main() {} since we don't use underlying runtime that calls it (crt0)
// instead we overwrite operating system entry point with our own _start function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8; // pass string byte in memory 

    // iterate over HELLO string byte and write into vga_buffer, as well as color light cyan (0xb)
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
