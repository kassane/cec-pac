#[doc = "Register `CTRL2P[%s]` reader"]
pub struct R(crate::R<CTRL2P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2P[%s]` writer"]
pub struct W(crate::W<CTRL2P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2P_SPEC>;
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
impl From<crate::W<CTRL2P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEW_RATE` reader - This bit is used to select the slew rate on the pin. 0 = slow (half frequency), 1 = fast"]
pub type SLEW_RATE_R = crate::BitReader<bool>;
#[doc = "Field `SLEW_RATE` writer - This bit is used to select the slew rate on the pin. 0 = slow (half frequency), 1 = fast"]
pub type SLEW_RATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2P_SPEC, bool, O>;
#[doc = "Field `DRIV_STREN` reader - These bits are used to select the drive strength on the pin. The drive strength is the same whether the pin is\n powered by 3.3V or 1.8V. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
pub type DRIV_STREN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DRIV_STREN` writer - These bits are used to select the drive strength on the pin. The drive strength is the same whether the pin is\n powered by 3.3V or 1.8V. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
pub type DRIV_STREN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2P_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to select the slew rate on the pin. 0 = slow (half frequency), 1 = fast"]
    #[inline(always)]
    pub fn slew_rate(&self) -> SLEW_RATE_R {
        SLEW_RATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - These bits are used to select the drive strength on the pin. The drive strength is the same whether the pin is\n powered by 3.3V or 1.8V. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
    #[inline(always)]
    pub fn driv_stren(&self) -> DRIV_STREN_R {
        DRIV_STREN_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to select the slew rate on the pin. 0 = slow (half frequency), 1 = fast"]
    #[inline(always)]
    pub fn slew_rate(&mut self) -> SLEW_RATE_W<0> {
        SLEW_RATE_W::new(self)
    }
    #[doc = "Bits 4:5 - These bits are used to select the drive strength on the pin. The drive strength is the same whether the pin is\n powered by 3.3V or 1.8V. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
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
#[doc = "The GPIO PIN_CTRL2 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2p](index.html) module"]
pub struct CTRL2P_SPEC;
impl crate::RegisterSpec for CTRL2P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2p::R](R) reader structure"]
impl crate::Readable for CTRL2P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2p::W](W) writer structure"]
impl crate::Writable for CTRL2P_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2P[%s]
to value 0"]
impl crate::Resettable for CTRL2P_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
