
// forbidding stdlib which depends os
#![no_std]
// remove the predefined entry point main
#![no_main]

// forbidding name mangling, so the linker can recongnize the "_start"
// 'extern "C"' tells the complier that the function should obey the C language call convention
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

use core::panic::PanicInfo;
// without stdlib, we must implement a panic function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
