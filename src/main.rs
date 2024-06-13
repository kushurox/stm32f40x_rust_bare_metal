#![no_std]
#![no_main]


use core::{panic::PanicInfo, ptr::{addr_of, addr_of_mut, copy_nonoverlapping, read_volatile, write_bytes, write_volatile}};


mod usefulmacros;

extern "C" {
    static mut _sbss: u32;  // _sbss = *0x2000 rather than 0x2000 hence we need to use &_sbss to get the address 0x2000
    static mut _ebss: u32;
    static mut _sdata: u32;  // VMA of .data (in the ram)
    static mut _edata: u32;
    static mut _sidata: u32; // LMA of .data (we will use this to copy data from flash memory to the ram upon initialising)
}

static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;

#[no_mangle]
pub unsafe extern "C" fn _reset() -> ! {
    let temp = addr_of!(_ebss) as usize - addr_of!(_sbss) as usize; // really hope i didnt miss out the last byte
    write_bytes(addr_of_mut!(_sbss), 0, temp); // initializing bss section to 0
    let temp = addr_of!(_edata) as usize - addr_of!(_sdata) as usize;
    copy_nonoverlapping(addr_of!(_sidata), addr_of_mut!(_sdata), temp); // copying data from LMA to VMA (flash to RAM)
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