#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LCV {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LCVR {
    bits: u16,
}
impl LCVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LCPCR {
    bits: u8,
}
impl LCPCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LCNCR {
    bits: u8,
}
impl LCNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Last converted value"]
    #[inline]
    pub fn lcv(&self) -> LCVR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LCVR { bits }
    }
    #[doc = "Bits 16:19 - Last converted positive channel"]
    #[inline]
    pub fn lcpc(&self) -> LCPCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LCPCR { bits }
    }
    #[doc = "Bits 20:22 - Last converted negative channel"]
    #[inline]
    pub fn lcnc(&self) -> LCNCR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LCNCR { bits }
    }
}
