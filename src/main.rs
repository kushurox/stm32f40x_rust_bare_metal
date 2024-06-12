#![no_std]
#![no_main]


use core::panic::PanicInfo;
mod usefulmacros;

extern {
}


pub unsafe extern "C" fn _reset() -> ! {
    
    loop {}
}


#[link_section = ".vectors.reset_handler"]
#[no_mangle]
static RESET_HANDLER: unsafe extern "C" fn() -> ! = _reset;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> !{
    loop {

    }
}