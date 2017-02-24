device!{GPIO | MODE TYPE SPEED PUPD IDR ODR BSR LCK AFRL AFRH}

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
        register!(self.BSR <- what as u32);
    }

    #[inline(always)]
    pub fn breset(&mut self, what: u16) {
        register!(self.BSR <- (what as u32) << 16);
    }

    #[inline(always)]
    pub fn bsetreset(&mut self, set: u16, reset: u16) {
        register!(self.BSR <- set as u32 | ((reset as u32) << 16));
    }

    #[inline(always)]
    pub fn setmode(&mut self, port: PIN, m: MODE) {
        let mut oldm = register!(<- self.MODE);
        let mask: u32 = 0b11 << ((port as usize) * 2);
        let data = (m as u32) << ((port as usize) * 2);
        oldm &= !mask;
        oldm |= data;
        register!(self.MODE <- oldm);
    }
}
