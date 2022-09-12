#[doc = "Register `CFG_STS` reader"]
pub struct R(crate::R<CFG_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_STS` writer"]
pub struct W(crate::W<CFG_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFG_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMD` reader - Flash Intervention Mode. 0 = Power-Off (POR), 1 = RESET# Pulse"]
pub type IMD_R = crate::BitReader<bool>;
#[doc = "Field `IMD` writer - Flash Intervention Mode. 0 = Power-Off (POR), 1 = RESET# Pulse"]
pub type IMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
#[doc = "Field `RST2CSH` reader - RESET# to CS# High Delay. 2^n x 20us, 00h=20us, 1Fh= 1.3sec. IRQ triggers at this point also."]
pub type RST2CSH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RST2CSH` writer - RESET# to CS# High Delay. 2^n x 20us, 00h=20us, 1Fh= 1.3sec. IRQ triggers at this point also."]
pub type RST2CSH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_STS_SPEC, u8, u8, 5, O>;
#[doc = "Field `QBD` reader - Quad Bus Disable, 1 = Half Bus Mode, 0 = IO\\[3:0\\]
are all controlled by EQS."]
pub type QBD_R = crate::BitReader<bool>;
#[doc = "Field `QBD` writer - Quad Bus Disable, 1 = Half Bus Mode, 0 = IO\\[3:0\\]
are all controlled by EQS."]
pub type QBD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
#[doc = "Field `XQS` reader - Cross-Over Q-Switch status. (BMC->CPU). RO image of Interbus bit which as 1 overrides EQS bit at both SPI buses without changing their states."]
pub type XQS_R = crate::BitReader<bool>;
#[doc = "Field `XQS` writer - Cross-Over Q-Switch status. (BMC->CPU). RO image of Interbus bit which as 1 overrides EQS bit at both SPI buses without changing their states."]
pub type XQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
#[doc = "Field `EQS` reader - Enable Q-Switch (Isolator) to Host. 0 = Disable, 1 = Enable."]
pub type EQS_R = crate::BitReader<bool>;
#[doc = "Field `EQS` writer - Enable Q-Switch (Isolator) to Host. 0 = Disable, 1 = Enable."]
pub type EQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
#[doc = "Field `CSRT` reader - Chip Select Routing. 000 = Both CSn#_In pass directly to CSn# Out, both enabled out. 001 = CS1n_In passes to CS1# Output, but CS0# Output is disabled (floats high). 010 = CS0n_In passes to CS0# Output, but CS1# Output is disabled (floats high). 100 = Swap CS1#/CS0#_In to CS0#/CS1# Out respectively, both enabled out. 101 = CS0n_In passes to CS1# Out, and CS0# Output is disabled (floats high). 110 = CS1n_In passes to CS0# Out, and CS1# Output is disabled (floats high). x11 = CS0n/CS1n Outputs are both disabled (floating high) regardless of inputs."]
pub type CSRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSRT` writer - Chip Select Routing. 000 = Both CSn#_In pass directly to CSn# Out, both enabled out. 001 = CS1n_In passes to CS1# Output, but CS0# Output is disabled (floats high). 010 = CS0n_In passes to CS0# Output, but CS1# Output is disabled (floats high). 100 = Swap CS1#/CS0#_In to CS0#/CS1# Out respectively, both enabled out. 101 = CS0n_In passes to CS1# Out, and CS0# Output is disabled (floats high). 110 = CS1n_In passes to CS0# Out, and CS1# Output is disabled (floats high). x11 = CS0n/CS1n Outputs are both disabled (floating high) regardless of inputs."]
pub type CSRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_STS_SPEC, u8, u8, 3, O>;
#[doc = "Field `F0F` reader - Enable following of Address Mode. (SPI Snooping) for each Flash. 0=Disable, 1=Enable"]
pub type F0F_R = crate::BitReader<bool>;
#[doc = "Field `F0F` writer - Enable following of Address Mode. (SPI Snooping) for each Flash. 0=Disable, 1=Enable"]
pub type F0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
#[doc = "Field `F1F` reader - Enable following of Address Mode. (SPI Snooping) for each Flash. 0=Disable, 1=Enable"]
pub type F1F_R = crate::BitReader<bool>;
#[doc = "Field `F1F` writer - Enable following of Address Mode. (SPI Snooping) for each Flash. 0=Disable, 1=Enable"]
pub type F1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
#[doc = "Field `F0A` reader - Set 3B/4B Address Mode for Flash 0. 0=3B Address Mode, 1=4B Address Mode"]
pub type F0A_R = crate::BitReader<bool>;
#[doc = "Field `F0A` writer - Set 3B/4B Address Mode for Flash 0. 0=3B Address Mode, 1=4B Address Mode"]
pub type F0A_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
#[doc = "Field `F1A` reader - Set 3B/4B Address Mode for Flash 1. 0=3B Address Mode, 1=4B Address Mode"]
pub type F1A_R = crate::BitReader<bool>;
#[doc = "Field `F1A` writer - Set 3B/4B Address Mode for Flash 1. 0=3B Address Mode, 1=4B Address Mode"]
pub type F1A_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
#[doc = "Field `F0SIZE` reader - Flash 0 Size. Flash sizes are expressed in bytes as a power of 2, matching a Flash Get-ID convention. For the most common cases: 17h = 23d => 2^23 = 8MByte 18h = 24d => 2^24 = 16MByte 19h = 25d => 2^25 = 32MByte 1Ah = 26d => 2^26 = 64MByte"]
pub type F0SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F0SIZE` writer - Flash 0 Size. Flash sizes are expressed in bytes as a power of 2, matching a Flash Get-ID convention. For the most common cases: 17h = 23d => 2^23 = 8MByte 18h = 24d => 2^24 = 16MByte 19h = 25d => 2^25 = 32MByte 1Ah = 26d => 2^26 = 64MByte"]
pub type F0SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_STS_SPEC, u8, u8, 5, O>;
#[doc = "Field `E0W` reader - Enable Wrap Detection. 0=Disable, 1=Enable."]
pub type E0W_R = crate::BitReader<bool>;
#[doc = "Field `E0W` writer - Enable Wrap Detection. 0=Disable, 1=Enable."]
pub type E0W_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
#[doc = "Field `F0P` reader - Flash 0 Present. 0=Not Present, 1=Present"]
pub type F0P_R = crate::BitReader<bool>;
#[doc = "Field `F0P` writer - Flash 0 Present. 0=Not Present, 1=Present"]
pub type F0P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
#[doc = "Field `F1SIZE` reader - Flash 1 Size. Flash sizes are expressed in bytes as a power of 2, matching a Flash Get-ID convention. For the most common cases: 17h = 23d => 2^23 = 8MByte 18h = 24d => 2^24 = 16MByte 19h = 25d => 2^25 = 32MByte 1Ah = 26d => 2^26 = 64MByte"]
pub type F1SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1SIZE` writer - Flash 1 Size. Flash sizes are expressed in bytes as a power of 2, matching a Flash Get-ID convention. For the most common cases: 17h = 23d => 2^23 = 8MByte 18h = 24d => 2^24 = 16MByte 19h = 25d => 2^25 = 32MByte 1Ah = 26d => 2^26 = 64MByte"]
pub type F1SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_STS_SPEC, u8, u8, 5, O>;
#[doc = "Field `E1W` reader - Enable Wrap Detection. 0=Disable, 1=Enable"]
pub type E1W_R = crate::BitReader<bool>;
#[doc = "Field `E1W` writer - Enable Wrap Detection. 0=Disable, 1=Enable"]
pub type E1W_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
#[doc = "Field `F1P` reader - Flash 1 Present. 0=Not Present, 1=Present"]
pub type F1P_R = crate::BitReader<bool>;
#[doc = "Field `F1P` writer - Flash 1 Present. 0=Not Present, 1=Present"]
pub type F1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Flash Intervention Mode. 0 = Power-Off (POR), 1 = RESET# Pulse"]
    #[inline(always)]
    pub fn imd(&self) -> IMD_R {
        IMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - RESET# to CS# High Delay. 2^n x 20us, 00h=20us, 1Fh= 1.3sec. IRQ triggers at this point also."]
    #[inline(always)]
    pub fn rst2csh(&self) -> RST2CSH_R {
        RST2CSH_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Quad Bus Disable, 1 = Half Bus Mode, 0 = IO\\[3:0\\]
are all controlled by EQS."]
    #[inline(always)]
    pub fn qbd(&self) -> QBD_R {
        QBD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Cross-Over Q-Switch status. (BMC->CPU). RO image of Interbus bit which as 1 overrides EQS bit at both SPI buses without changing their states."]
    #[inline(always)]
    pub fn xqs(&self) -> XQS_R {
        XQS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Q-Switch (Isolator) to Host. 0 = Disable, 1 = Enable."]
    #[inline(always)]
    pub fn eqs(&self) -> EQS_R {
        EQS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Chip Select Routing. 000 = Both CSn#_In pass directly to CSn# Out, both enabled out. 001 = CS1n_In passes to CS1# Output, but CS0# Output is disabled (floats high). 010 = CS0n_In passes to CS0# Output, but CS1# Output is disabled (floats high). 100 = Swap CS1#/CS0#_In to CS0#/CS1# Out respectively, both enabled out. 101 = CS0n_In passes to CS1# Out, and CS0# Output is disabled (floats high). 110 = CS1n_In passes to CS0# Out, and CS1# Output is disabled (floats high). x11 = CS0n/CS1n Outputs are both disabled (floating high) regardless of inputs."]
    #[inline(always)]
    pub fn csrt(&self) -> CSRT_R {
        CSRT_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Enable following of Address Mode. (SPI Snooping) for each Flash. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable following of Address Mode. (SPI Snooping) for each Flash. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set 3B/4B Address Mode for Flash 0. 0=3B Address Mode, 1=4B Address Mode"]
    #[inline(always)]
    pub fn f0a(&self) -> F0A_R {
        F0A_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set 3B/4B Address Mode for Flash 1. 0=3B Address Mode, 1=4B Address Mode"]
    #[inline(always)]
    pub fn f1a(&self) -> F1A_R {
        F1A_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Flash 0 Size. Flash sizes are expressed in bytes as a power of 2, matching a Flash Get-ID convention. For the most common cases: 17h = 23d => 2^23 = 8MByte 18h = 24d => 2^24 = 16MByte 19h = 25d => 2^25 = 32MByte 1Ah = 26d => 2^26 = 64MByte"]
    #[inline(always)]
    pub fn f0size(&self) -> F0SIZE_R {
        F0SIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Enable Wrap Detection. 0=Disable, 1=Enable."]
    #[inline(always)]
    pub fn e0w(&self) -> E0W_R {
        E0W_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Flash 0 Present. 0=Not Present, 1=Present"]
    #[inline(always)]
    pub fn f0p(&self) -> F0P_R {
        F0P_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Flash 1 Size. Flash sizes are expressed in bytes as a power of 2, matching a Flash Get-ID convention. For the most common cases: 17h = 23d => 2^23 = 8MByte 18h = 24d => 2^24 = 16MByte 19h = 25d => 2^25 = 32MByte 1Ah = 26d => 2^26 = 64MByte"]
    #[inline(always)]
    pub fn f1size(&self) -> F1SIZE_R {
        F1SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Enable Wrap Detection. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn e1w(&self) -> E1W_R {
        E1W_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Flash 1 Present. 0=Not Present, 1=Present"]
    #[inline(always)]
    pub fn f1p(&self) -> F1P_R {
        F1P_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Intervention Mode. 0 = Power-Off (POR), 1 = RESET# Pulse"]
    #[inline(always)]
    pub fn imd(&mut self) -> IMD_W<0> {
        IMD_W::new(self)
    }
    #[doc = "Bits 1:5 - RESET# to CS# High Delay. 2^n x 20us, 00h=20us, 1Fh= 1.3sec. IRQ triggers at this point also."]
    #[inline(always)]
    pub fn rst2csh(&mut self) -> RST2CSH_W<1> {
        RST2CSH_W::new(self)
    }
    #[doc = "Bit 6 - Quad Bus Disable, 1 = Half Bus Mode, 0 = IO\\[3:0\\]
are all controlled by EQS."]
    #[inline(always)]
    pub fn qbd(&mut self) -> QBD_W<6> {
        QBD_W::new(self)
    }
    #[doc = "Bit 7 - Cross-Over Q-Switch status. (BMC->CPU). RO image of Interbus bit which as 1 overrides EQS bit at both SPI buses without changing their states."]
    #[inline(always)]
    pub fn xqs(&mut self) -> XQS_W<7> {
        XQS_W::new(self)
    }
    #[doc = "Bit 8 - Enable Q-Switch (Isolator) to Host. 0 = Disable, 1 = Enable."]
    #[inline(always)]
    pub fn eqs(&mut self) -> EQS_W<8> {
        EQS_W::new(self)
    }
    #[doc = "Bits 9:11 - Chip Select Routing. 000 = Both CSn#_In pass directly to CSn# Out, both enabled out. 001 = CS1n_In passes to CS1# Output, but CS0# Output is disabled (floats high). 010 = CS0n_In passes to CS0# Output, but CS1# Output is disabled (floats high). 100 = Swap CS1#/CS0#_In to CS0#/CS1# Out respectively, both enabled out. 101 = CS0n_In passes to CS1# Out, and CS0# Output is disabled (floats high). 110 = CS1n_In passes to CS0# Out, and CS1# Output is disabled (floats high). x11 = CS0n/CS1n Outputs are both disabled (floating high) regardless of inputs."]
    #[inline(always)]
    pub fn csrt(&mut self) -> CSRT_W<9> {
        CSRT_W::new(self)
    }
    #[doc = "Bit 12 - Enable following of Address Mode. (SPI Snooping) for each Flash. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn f0f(&mut self) -> F0F_W<12> {
        F0F_W::new(self)
    }
    #[doc = "Bit 13 - Enable following of Address Mode. (SPI Snooping) for each Flash. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn f1f(&mut self) -> F1F_W<13> {
        F1F_W::new(self)
    }
    #[doc = "Bit 14 - Set 3B/4B Address Mode for Flash 0. 0=3B Address Mode, 1=4B Address Mode"]
    #[inline(always)]
    pub fn f0a(&mut self) -> F0A_W<14> {
        F0A_W::new(self)
    }
    #[doc = "Bit 15 - Set 3B/4B Address Mode for Flash 1. 0=3B Address Mode, 1=4B Address Mode"]
    #[inline(always)]
    pub fn f1a(&mut self) -> F1A_W<15> {
        F1A_W::new(self)
    }
    #[doc = "Bits 16:20 - Flash 0 Size. Flash sizes are expressed in bytes as a power of 2, matching a Flash Get-ID convention. For the most common cases: 17h = 23d => 2^23 = 8MByte 18h = 24d => 2^24 = 16MByte 19h = 25d => 2^25 = 32MByte 1Ah = 26d => 2^26 = 64MByte"]
    #[inline(always)]
    pub fn f0size(&mut self) -> F0SIZE_W<16> {
        F0SIZE_W::new(self)
    }
    #[doc = "Bit 21 - Enable Wrap Detection. 0=Disable, 1=Enable."]
    #[inline(always)]
    pub fn e0w(&mut self) -> E0W_W<21> {
        E0W_W::new(self)
    }
    #[doc = "Bit 23 - Flash 0 Present. 0=Not Present, 1=Present"]
    #[inline(always)]
    pub fn f0p(&mut self) -> F0P_W<23> {
        F0P_W::new(self)
    }
    #[doc = "Bits 24:28 - Flash 1 Size. Flash sizes are expressed in bytes as a power of 2, matching a Flash Get-ID convention. For the most common cases: 17h = 23d => 2^23 = 8MByte 18h = 24d => 2^24 = 16MByte 19h = 25d => 2^25 = 32MByte 1Ah = 26d => 2^26 = 64MByte"]
    #[inline(always)]
    pub fn f1size(&mut self) -> F1SIZE_W<24> {
        F1SIZE_W::new(self)
    }
    #[doc = "Bit 29 - Enable Wrap Detection. 0=Disable, 1=Enable"]
    #[inline(always)]
    pub fn e1w(&mut self) -> E1W_W<29> {
        E1W_W::new(self)
    }
    #[doc = "Bit 31 - Flash 1 Present. 0=Not Present, 1=Present"]
    #[inline(always)]
    pub fn f1p(&mut self) -> F1P_W<31> {
        F1P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Configuration Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_sts](index.html) module"]
pub struct CFG_STS_SPEC;
impl crate::RegisterSpec for CFG_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_sts::R](R) reader structure"]
impl crate::Readable for CFG_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_sts::W](W) writer structure"]
impl crate::Writable for CFG_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_STS to value 0"]
impl crate::Resettable for CFG_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
