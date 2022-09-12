#[doc = "Register `BROM_STS` reader"]
pub struct R(crate::R<BROM_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROM_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROM_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROM_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BROM_STS` writer"]
pub struct W(crate::W<BROM_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BROM_STS_SPEC>;
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
impl From<crate::W<BROM_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BROM_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VTR_RST_STS` reader - This bit contains the RESET_SYS Status for BOOT ROM usage.\n 1= RESET_SYS event occured.\n 0= RESET_SYS event did not occured.\n This registe is R/W1C (read / write 1 to clear)."]
pub type VTR_RST_STS_R = crate::BitReader<bool>;
#[doc = "Field `VTR_RST_STS` writer - This bit contains the RESET_SYS Status for BOOT ROM usage.\n 1= RESET_SYS event occured.\n 0= RESET_SYS event did not occured.\n This registe is R/W1C (read / write 1 to clear)."]
pub type VTR_RST_STS_W<'a, const O: u8> = crate::BitWriter<'a, u8, BROM_STS_SPEC, bool, O>;
#[doc = "Field `WDT_EVT` reader - This bit contains the WDT Event Status for BOOT ROM usage.\n 1= WDT event occured.\n 0= WDT event did not occured.\n This registe is R/W1C (read / write 1 to clear)."]
pub type WDT_EVT_R = crate::BitReader<bool>;
#[doc = "Field `WDT_EVT` writer - This bit contains the WDT Event Status for BOOT ROM usage.\n 1= WDT event occured.\n 0= WDT event did not occured.\n This registe is R/W1C (read / write 1 to clear)."]
pub type WDT_EVT_W<'a, const O: u8> = crate::BitWriter<'a, u8, BROM_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit contains the RESET_SYS Status for BOOT ROM usage.\n 1= RESET_SYS event occured.\n 0= RESET_SYS event did not occured.\n This registe is R/W1C (read / write 1 to clear)."]
    #[inline(always)]
    pub fn vtr_rst_sts(&self) -> VTR_RST_STS_R {
        VTR_RST_STS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit contains the WDT Event Status for BOOT ROM usage.\n 1= WDT event occured.\n 0= WDT event did not occured.\n This registe is R/W1C (read / write 1 to clear)."]
    #[inline(always)]
    pub fn wdt_evt(&self) -> WDT_EVT_R {
        WDT_EVT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit contains the RESET_SYS Status for BOOT ROM usage.\n 1= RESET_SYS event occured.\n 0= RESET_SYS event did not occured.\n This registe is R/W1C (read / write 1 to clear)."]
    #[inline(always)]
    pub fn vtr_rst_sts(&mut self) -> VTR_RST_STS_W<0> {
        VTR_RST_STS_W::new(self)
    }
    #[doc = "Bit 1 - This bit contains the WDT Event Status for BOOT ROM usage.\n 1= WDT event occured.\n 0= WDT event did not occured.\n This registe is R/W1C (read / write 1 to clear)."]
    #[inline(always)]
    pub fn wdt_evt(&mut self) -> WDT_EVT_W<1> {
        WDT_EVT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the VTR Reset Status for BOOT ROM.\n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brom_sts](index.html) module"]
pub struct BROM_STS_SPEC;
impl crate::RegisterSpec for BROM_STS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [brom_sts::R](R) reader structure"]
impl crate::Readable for BROM_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brom_sts::W](W) writer structure"]
impl crate::Writable for BROM_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BROM_STS to value 0x01"]
impl crate::Resettable for BROM_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
