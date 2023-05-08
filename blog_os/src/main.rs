// we disable standard libraries to design our OS as close to the metal as possible

#![no_std]
#![no_main] // tell compiler not to use normal entry point chain, since we aren't using std lib C
            // that helps us link to crt0 (c runtime event zero) 

use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// removed main() {} since we don't use underlying runtime that calls it (crt0)
// instead we overwrite operating system entry point with our own _start function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
