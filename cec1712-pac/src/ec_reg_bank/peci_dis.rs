#[doc = "Register `PECI_DIS` reader"]
pub struct R(crate::R<PECI_DIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PECI_DIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PECI_DIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PECI_DIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PECI_DIS` writer"]
pub struct W(crate::W<PECI_DIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PECI_DIS_SPEC>;
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
impl From<crate::W<PECI_DIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PECI_DIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P_DIS` reader - When this bit is asserted ('1'), it disables the PECI pads to reduce leakage."]
pub type P_DIS_R = crate::BitReader<bool>;
#[doc = "Field `P_DIS` writer - When this bit is asserted ('1'), it disables the PECI pads to reduce leakage."]
pub type P_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PECI_DIS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When this bit is asserted ('1'), it disables the PECI pads to reduce leakage."]
    #[inline(always)]
    pub fn p_dis(&self) -> P_DIS_R {
        P_DIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is asserted ('1'), it disables the PECI pads to reduce leakage."]
    #[inline(always)]
    pub fn p_dis(&mut self) -> P_DIS_W<0> {
        P_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PECI Disable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peci_dis](index.html) module"]
pub struct PECI_DIS_SPEC;
impl crate::RegisterSpec for PECI_DIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peci_dis::R](R) reader structure"]
impl crate::Readable for PECI_DIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peci_dis::W](W) writer structure"]
impl crate::Writable for PECI_DIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PECI_DIS to value 0"]
impl crate::Resettable for PECI_DIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
