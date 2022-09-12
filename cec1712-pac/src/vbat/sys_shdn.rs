#[doc = "Register `SYS_SHDN` reader"]
pub struct R(crate::R<SYS_SHDN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SHDN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SHDN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SHDN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SHDN` writer"]
pub struct W(crate::W<SYS_SHDN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SHDN_SPEC>;
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
impl From<crate::W<SYS_SHDN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SHDN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS` reader - This bit controls the System Shutdown.\n 0 = Enable System Shutdown (SYS_SHDN#).\n 1 = Disable System Shutdown (SYS_SHDN#).\n"]
pub type DIS_R = crate::BitReader<bool>;
#[doc = "Field `DIS` writer - This bit controls the System Shutdown.\n 0 = Enable System Shutdown (SYS_SHDN#).\n 1 = Disable System Shutdown (SYS_SHDN#).\n"]
pub type DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SHDN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit controls the System Shutdown.\n 0 = Enable System Shutdown (SYS_SHDN#).\n 1 = Disable System Shutdown (SYS_SHDN#).\n"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls the System Shutdown.\n 0 = Enable System Shutdown (SYS_SHDN#).\n 1 = Disable System Shutdown (SYS_SHDN#).\n"]
    #[inline(always)]
    pub fn dis(&mut self) -> DIS_W<0> {
        DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Shutdown Enable register.\n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_shdn](index.html) module"]
pub struct SYS_SHDN_SPEC;
impl crate::RegisterSpec for SYS_SHDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_shdn::R](R) reader structure"]
impl crate::Readable for SYS_SHDN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_shdn::W](W) writer structure"]
impl crate::Writable for SYS_SHDN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SHDN to value 0"]
impl crate::Resettable for SYS_SHDN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
