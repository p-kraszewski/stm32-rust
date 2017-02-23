#![allow(dead_code)]
#![allow(non_snake_case)]

#[repr(C)]
pub struct GPIO {
    pub MODE: u32,
    pub TYPE: u32,
    pub SPEED: u32,
    pub PUPD: u32,
    pub IDR: u32,
    pub ODR: u32,
    pub BSR: u32,
    pub LCK: u32,
    pub AFRL: u32,
    pub AFRH: u32,
}


#[derive(Clone, Copy)]
pub enum MODE {
    Input = 0b00,
    Output = 0b01,
    Alternate = 0b10,
    Analog = 0b11,
}

#[derive(Clone,Copy)]
pub enum PIN {
    PIN00,
    PIN01,
    PIN02,
    PIN03,
    PIN04,
    PIN05,
    PIN06,
    PIN07,
    PIN08,
    PIN09,
    PIN10,
    PIN11,
    PIN12,
    PIN13,
    PIN14,
    PIN15,
}

impl GPIO {
    #[inline(always)]
    pub fn bset(&mut self, what: u16) {
        unsafe {
            ::core::intrinsics::volatile_store(&mut self.BSR, what as u32);
        }
    }

    #[inline(always)]
    pub fn breset(&mut self, what: u16) {
        unsafe {
            ::core::intrinsics::volatile_store(&mut self.BSR,
                                               (what as u32) << 16);
        }
    }

    #[inline(always)]
    pub fn bsetreset(&mut self, set: u16, reset: u16) {
        unsafe {
            ::core::intrinsics::volatile_store(&mut self.BSR,
                                               set as u32 |
                                               ((reset as u32) << 16));
        }
    }

    #[inline(always)]
    pub fn setmode(&mut self, port: PIN, mode: MODE) {
        unsafe {
            let mut m = ::core::intrinsics::volatile_load::<u32>(&self.MODE);
            let mask: u32 = 0b11 << ((port as usize) * 2);
            let data = (mode as u32) << ((port as usize) * 2);
            m &= !mask;
            m |= data;
            ::core::intrinsics::volatile_store(&mut self.MODE, m);
        }
    }
}
