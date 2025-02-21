#![no_std]
#![no_main]

use core::panic::PanicInfo;


use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;




#[entry]
fn kmain() -> !{
    let _RCC_BASE: *mut u32 = 0x40023800 as *mut u32;
    let RCC_AHB1_ENR: *mut u32 = (0x40023800+0x30) as *mut u32;
    let GPIO_A: *mut u32 = 0x4002_0000  as *mut u32;
    let GPIO_B: *mut u32 = 0x4002_0400  as *mut u32;


    unsafe {
        let mut temp = RCC_AHB1_ENR.read_volatile();
        temp |= 0b01;
        RCC_AHB1_ENR.write_volatile(temp);
        // setting reg to output
        temp = GPIO_A.read_volatile();
        temp |= 0b01;
        GPIO_A.write_volatile(temp);
        temp = GPIO_A.offset(0x14).read_volatile();
        temp |= 0b01;
        GPIO_A.offset(0x14)
        .write_volatile(temp);  // fucked up line
    }
    loop {}
}

#[panic_handler]
fn oh_noes(_info: &PanicInfo) -> !{

    loop {}
}