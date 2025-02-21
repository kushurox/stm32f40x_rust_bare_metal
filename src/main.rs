#![no_std]
#![no_main]

use core::panic::PanicInfo;


use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;


const _RCC_BASE: *mut u32 = 0x40023800 as *mut u32;
const RCC_AHB1_ENR: *mut u32 = (0x40023800+0x30) as *mut u32;
const GPIO_A: *mut u32 = 0x4002_0000  as *mut u32;
const GPIO_B: *mut u32 = 0x4002_0400  as *mut u32;

enum GPIO_PORT {A,B,C,D,E,H}

#[repr(packed)]
pub struct GPIO {
    moder: *mut u32,
    otyper: *mut u32,
    ospeedr: *mut u32,
    pupdr: *mut u32,
    idr: *mut u32,
    odr: *mut u32,
    bsrr: *mut u32,
    lckr: *mut u32,
    afrl: *mut u32,
    afrh: *mut u32,
    port: GPIO_PORT

}

impl GPIO {
    fn new(port: GPIO_PORT) -> Self {
        GPIO {
            moder: (0x4002_0000 + (400 * (port - GPIO_PORT::A))) as *mut u32,

        }
    }
}


#[entry]
fn kmain() -> !{
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
        GPIO_A.byte_add(0x14).write_volatile(temp);
    }
    loop {}
}

#[panic_handler]
fn oh_noes(_info: &PanicInfo) -> !{

    loop {}
}