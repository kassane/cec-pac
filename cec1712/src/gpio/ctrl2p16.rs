#[doc = "Register `CTRL2P16[%s]` reader"]
pub struct R(crate::R<CTRL2P16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2P16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2P16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2P16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2P16[%s]` writer"]
pub struct W(crate::W<CTRL2P16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2P16_SPEC>;
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
impl From<crate::W<CTRL2P16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2P16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEW_CTRL` reader - Selects slew rate on the pin. 1=fast 0=slow"]
pub type SLEW_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `SLEW_CTRL` writer - Selects slew rate on the pin. 1=fast 0=slow"]
pub type SLEW_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2P16_SPEC, bool, O>;
#[doc = "Field `DRIV_STREN` reader - Selects the drive strength on the pin. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
pub type DRIV_STREN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIV_STREN` writer - Selects the drive strength on the pin. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
pub type DRIV_STREN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2P16_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Selects slew rate on the pin. 1=fast 0=slow"]
    #[inline(always)]
    pub fn slew_ctrl(&self) -> SLEW_CTRL_R {
        SLEW_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Selects the drive strength on the pin. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
    #[inline(always)]
    pub fn driv_stren(&self) -> DRIV_STREN_R {
        DRIV_STREN_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selects slew rate on the pin. 1=fast 0=slow"]
    #[inline(always)]
    pub fn slew_ctrl(&mut self) -> SLEW_CTRL_W<0> {
        SLEW_CTRL_W::new(self)
    }
    #[doc = "Bits 4:5 - Selects the drive strength on the pin. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
    #[inline(always)]
    pub fn driv_stren(&mut self) -> DRIV_STREN_W<4> {
        DRIV_STREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The GPIO PIN_CTRL2 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2p16](index.html) module"]
pub struct CTRL2P16_SPEC;
impl crate::RegisterSpec for CTRL2P16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2p16::R](R) reader structure"]
impl crate::Readable for CTRL2P16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2p16::W](W) writer structure"]
impl crate::Writable for CTRL2P16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2P16[%s]
to value 0"]
impl crate::Resettable for CTRL2P16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
