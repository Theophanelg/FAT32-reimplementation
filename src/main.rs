#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Cette fonction sera appeler à chaque panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Désactive la décoration du nom de la fonction
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop{}
}