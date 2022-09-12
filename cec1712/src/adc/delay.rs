#[doc = "Register `DELAY` reader"]
pub struct R(crate::R<DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DELAY` writer"]
pub struct W(crate::W<DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELAY_SPEC>;
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
impl From<crate::W<DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRT_DLY` reader - This field determines the starting delay before a conversion cycle is begun when Start_Repeat is written with a 1."]
pub type STRT_DLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STRT_DLY` writer - This field determines the starting delay before a conversion cycle is begun when Start_Repeat is written with a 1."]
pub type STRT_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAY_SPEC, u16, u16, 16, O>;
#[doc = "Field `RPT_DLY` reader - This field determines the interval between conversion cycles when Start_Repeat is 1."]
pub type RPT_DLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RPT_DLY` writer - This field determines the interval between conversion cycles when Start_Repeat is 1."]
pub type RPT_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAY_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This field determines the starting delay before a conversion cycle is begun when Start_Repeat is written with a 1."]
    #[inline(always)]
    pub fn strt_dly(&self) -> STRT_DLY_R {
        STRT_DLY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This field determines the interval between conversion cycles when Start_Repeat is 1."]
    #[inline(always)]
    pub fn rpt_dly(&self) -> RPT_DLY_R {
        RPT_DLY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field determines the starting delay before a conversion cycle is begun when Start_Repeat is written with a 1."]
    #[inline(always)]
    pub fn strt_dly(&mut self) -> STRT_DLY_W<0> {
        STRT_DLY_W::new(self)
    }
    #[doc = "Bits 16:31 - This field determines the interval between conversion cycles when Start_Repeat is 1."]
    #[inline(always)]
    pub fn rpt_dly(&mut self) -> RPT_DLY_W<16> {
        RPT_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The ADC Delay register determines the delay from setting Start_Repeat in the ADC Control Register and the start of a conversion cycle. This register also controls the interval between conversion cycles in repeat mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delay](index.html) module"]
pub struct DELAY_SPEC;
impl crate::RegisterSpec for DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [delay::R](R) reader structure"]
impl crate::Readable for DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delay::W](W) writer structure"]
impl crate::Writable for DELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DELAY to value 0"]
impl crate::Resettable for DELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
