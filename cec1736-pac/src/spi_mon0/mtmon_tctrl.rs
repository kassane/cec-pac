#[doc = "Register `MTMON_TCTRL` reader"]
pub struct R(crate::R<MTMON_TCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTMON_TCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTMON_TCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTMON_TCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTMON_TCTRL` writer"]
pub struct W(crate::W<MTMON_TCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTMON_TCTRL_SPEC>;
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
impl From<crate::W<MTMON_TCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTMON_TCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST` reader - Start Timeout Counter"]
pub type ST_R = crate::BitReader<bool>;
#[doc = "Field `ST` writer - Start Timeout Counter"]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_TCTRL_SPEC, bool, O>;
#[doc = "Field `TV` reader - Timeout Value 0 to 32"]
pub type TV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TV` writer - Timeout Value 0 to 32"]
pub type TV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTMON_TCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `TU` reader - Timeout Unit 00 = none (off), 01 = 32ms, 10 = 128ms, 11 = 1sec"]
pub type TU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TU` writer - Timeout Unit 00 = none (off), 01 = 32ms, 10 = 128ms, 11 = 1sec"]
pub type TU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTMON_TCTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Start Timeout Counter"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:20 - Timeout Value 0 to 32"]
    #[inline(always)]
    pub fn tv(&self) -> TV_R {
        TV_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Timeout Unit 00 = none (off), 01 = 32ms, 10 = 128ms, 11 = 1sec"]
    #[inline(always)]
    pub fn tu(&self) -> TU_R {
        TU_R::new(((self.bits >> 21) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start Timeout Counter"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<0> {
        ST_W::new(self)
    }
    #[doc = "Bits 16:20 - Timeout Value 0 to 32"]
    #[inline(always)]
    pub fn tv(&mut self) -> TV_W<16> {
        TV_W::new(self)
    }
    #[doc = "Bits 21:22 - Timeout Unit 00 = none (off), 01 = 32ms, 10 = 128ms, 11 = 1sec"]
    #[inline(always)]
    pub fn tu(&mut self) -> TU_W<21> {
        TU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match Fetch Timeout Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtmon_tctrl](index.html) module"]
pub struct MTMON_TCTRL_SPEC;
impl crate::RegisterSpec for MTMON_TCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtmon_tctrl::R](R) reader structure"]
impl crate::Readable for MTMON_TCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtmon_tctrl::W](W) writer structure"]
impl crate::Writable for MTMON_TCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTMON_TCTRL to value 0"]
impl crate::Resettable for MTMON_TCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
