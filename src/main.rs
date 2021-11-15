#![no_std]
#![no_main]

extern "C" {
    pub fn kill();
    pub fn empty();
    // pub fn exit();
}
#[no_mangle]
extern "C" fn _start() {
    unsafe {
        kill();
    }
    loop {}
    main();
}

fn main() {
    unsafe {
        kill();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
use core::panic::PanicInfo;
