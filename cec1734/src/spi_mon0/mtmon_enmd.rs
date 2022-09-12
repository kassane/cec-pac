#[doc = "Register `MTMON_ENMD` reader"]
pub struct R(crate::R<MTMON_ENMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTMON_ENMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTMON_ENMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTMON_ENMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTMON_ENMD` writer"]
pub struct W(crate::W<MTMON_ENMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTMON_ENMD_SPEC>;
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
impl From<crate::W<MTMON_ENMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTMON_ENMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MON_EN` reader - Enable Data Matching. 0 = Disabled, 1 = Enabled"]
pub type MON_EN_R = crate::BitReader<bool>;
#[doc = "Field `MON_EN` writer - Enable Data Matching. 0 = Disabled, 1 = Enabled"]
pub type MON_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_ENMD_SPEC, bool, O>;
#[doc = "Field `MON_MS` reader - Match Pattern Source Mode: 0 = SRAM, 1 = Internal Flash."]
pub type MON_MS_R = crate::BitReader<bool>;
#[doc = "Field `MON_MS` writer - Match Pattern Source Mode: 0 = SRAM, 1 = Internal Flash."]
pub type MON_MS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTMON_ENMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Data Matching. 0 = Disabled, 1 = Enabled"]
    #[inline(always)]
    pub fn mon_en(&self) -> MON_EN_R {
        MON_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Match Pattern Source Mode: 0 = SRAM, 1 = Internal Flash."]
    #[inline(always)]
    pub fn mon_ms(&self) -> MON_MS_R {
        MON_MS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Data Matching. 0 = Disabled, 1 = Enabled"]
    #[inline(always)]
    pub fn mon_en(&mut self) -> MON_EN_W<0> {
        MON_EN_W::new(self)
    }
    #[doc = "Bit 8 - Match Pattern Source Mode: 0 = SRAM, 1 = Internal Flash."]
    #[inline(always)]
    pub fn mon_ms(&mut self) -> MON_MS_W<8> {
        MON_MS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match Monitor Enable/Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtmon_enmd](index.html) module"]
pub struct MTMON_ENMD_SPEC;
impl crate::RegisterSpec for MTMON_ENMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtmon_enmd::R](R) reader structure"]
impl crate::Readable for MTMON_ENMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtmon_enmd::W](W) writer structure"]
impl crate::Writable for MTMON_ENMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTMON_ENMD to value 0"]
impl crate::Resettable for MTMON_ENMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
