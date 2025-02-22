#![no_std]
#![no_main]

use core::panic::PanicInfo;

use cortex_m_rt::entry;
use gpio::{GpioPort, PinMode, PortVal, GPIO};

mod gpio;


#[entry]
fn kmain() -> !{
    let port_a = GPIO::new(GpioPort::A);
    port_a.set_pin_mode(0, PinMode::OUTPUT);
    port_a.write_atomic(0, PortVal::LOW);
    loop {}
}

#[panic_handler]
fn oh_noes(_info: &PanicInfo) -> !{

    loop {}
}