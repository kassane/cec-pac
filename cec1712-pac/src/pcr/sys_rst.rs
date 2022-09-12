#[doc = "Register `SYS_RST` reader"]
pub struct R(crate::R<SYS_RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_RST` writer"]
pub struct W(crate::W<SYS_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_RST_SPEC>;
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
impl From<crate::W<SYS_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFT_SYS_RST` reader - A write of a 1 forces an assertion of the RESET_SYS reset signal, resetting the device. A write of 0 has no effect."]
pub type SOFT_SYS_RST_R = crate::BitReader<bool>;
#[doc = "Field `SOFT_SYS_RST` writer - A write of a 1 forces an assertion of the RESET_SYS reset signal, resetting the device. A write of 0 has no effect."]
pub type SOFT_SYS_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_RST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - A write of a 1 forces an assertion of the RESET_SYS reset signal, resetting the device. A write of 0 has no effect."]
    #[inline(always)]
    pub fn soft_sys_rst(&self) -> SOFT_SYS_RST_R {
        SOFT_SYS_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - A write of a 1 forces an assertion of the RESET_SYS reset signal, resetting the device. A write of 0 has no effect."]
    #[inline(always)]
    pub fn soft_sys_rst(&mut self) -> SOFT_SYS_RST_W<8> {
        SOFT_SYS_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_rst](index.html) module"]
pub struct SYS_RST_SPEC;
impl crate::RegisterSpec for SYS_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_rst::R](R) reader structure"]
impl crate::Readable for SYS_RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_rst::W](W) writer structure"]
impl crate::Writable for SYS_RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_RST to value 0"]
impl crate::Resettable for SYS_RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
