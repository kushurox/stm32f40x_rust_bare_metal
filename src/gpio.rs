const RCC_AHB1_ENR: *mut u32 = 0x40023830 as *mut u32;



#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum GpioPort {A,B,C,D,E,_F,_G,H}


#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum PinMode {INPUT, OUTPUT, ALT, ANALOG}


#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum PortVal {LOW, HIGH}

#[allow(dead_code)]
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
    port: GpioPort,
}


#[allow(dead_code)]
// might make atomic version of this incase needed
impl GPIO {
    pub fn new(port: GpioPort) -> Self {
        let moder_off: u32 = (port as u32) - (GpioPort::A as u32);
        let moder: *mut u32 = (0x4002_0000 + (0x400*moder_off)) as *mut u32;
        
        let mut temp = unsafe{RCC_AHB1_ENR.read_volatile()};
        temp |= 1 << moder_off;
        unsafe{RCC_AHB1_ENR.write_volatile(temp);}      // enabling clock for that port, should this be a critical op?


        Self {
            moder,
            otyper: moder.wrapping_byte_add(4),
            ospeedr: moder.wrapping_byte_add(8),
            pupdr: moder.wrapping_byte_add(12),
            idr: moder.wrapping_byte_add(16),
            odr: moder.wrapping_byte_add(20),
            bsrr: moder.wrapping_byte_add(24),
            lckr: moder.wrapping_byte_add(28),
            afrl: moder.wrapping_byte_add(32),
            afrh: moder.wrapping_byte_add(36),
            port,
        }

    }

    pub fn set_pin_mode(&self, pin: u8, mode: PinMode) {
        let mut temp = unsafe{self.moder.read_volatile()};
        temp |= (mode as u32) << (pin << 1);
        unsafe{self.moder.write_volatile(temp)};
    }

    pub fn write_atomic(&self, pin: u8, val: PortVal){
        match val {
            PortVal::HIGH => {
                unsafe{self.bsrr.write_volatile(1 << pin);}
            },
            PortVal::LOW => {
                unsafe{self.bsrr.write_volatile(1 << (pin+16));}
            }
        }
    }

}