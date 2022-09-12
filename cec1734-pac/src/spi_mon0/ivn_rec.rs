#[doc = "Register `IVN_REC` writer"]
pub struct W(crate::W<IVN_REC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVN_REC_SPEC>;
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
impl From<crate::W<IVN_REC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVN_REC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCC` writer - Write 1 to clear FCS Flash Chip Selects forced high and bus forced low."]
pub type FCC_W<'a, const O: u8> = crate::BitWriter<'a, u8, IVN_REC_SPEC, bool, O>;
#[doc = "Field `FPC` writer - Write 1 to clear FPO Flash Power or RESET# Activated."]
pub type FPC_W<'a, const O: u8> = crate::BitWriter<'a, u8, IVN_REC_SPEC, bool, O>;
#[doc = "Field `HRC` writer - Write 1 to clear HRS Host held in Reset"]
pub type HRC_W<'a, const O: u8> = crate::BitWriter<'a, u8, IVN_REC_SPEC, bool, O>;
#[doc = "Field `HIC` writer - Write 1 to clear HIS Host held Isolated"]
pub type HIC_W<'a, const O: u8> = crate::BitWriter<'a, u8, IVN_REC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Write 1 to clear FCS Flash Chip Selects forced high and bus forced low."]
    #[inline(always)]
    pub fn fcc(&mut self) -> FCC_W<0> {
        FCC_W::new(self)
    }
    #[doc = "Bit 1 - Write 1 to clear FPO Flash Power or RESET# Activated."]
    #[inline(always)]
    pub fn fpc(&mut self) -> FPC_W<1> {
        FPC_W::new(self)
    }
    #[doc = "Bit 2 - Write 1 to clear HRS Host held in Reset"]
    #[inline(always)]
    pub fn hrc(&mut self) -> HRC_W<2> {
        HRC_W::new(self)
    }
    #[doc = "Bit 3 - Write 1 to clear HIS Host held Isolated"]
    #[inline(always)]
    pub fn hic(&mut self) -> HIC_W<3> {
        HIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Intervention Recovery Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivn_rec](index.html) module"]
pub struct IVN_REC_SPEC;
impl crate::RegisterSpec for IVN_REC_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [ivn_rec::W](W) writer structure"]
impl crate::Writable for IVN_REC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IVN_REC to value 0"]
impl crate::Resettable for IVN_REC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
