#[doc = "Register `SYS_SHUTDWN_RST` reader"]
pub struct R(crate::R<SYS_SHUTDWN_RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SHUTDWN_RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SHUTDWN_RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SHUTDWN_RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SHUTDWN_RST` writer"]
pub struct W(crate::W<SYS_SHUTDWN_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SHUTDWN_RST_SPEC>;
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
impl From<crate::W<SYS_SHUTDWN_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SHUTDWN_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYS_SHDN_RST` reader - When this bit is asserted 1, the SYS_SHDN# output is deasserted"]
pub type SYS_SHDN_RST_R = crate::BitReader<bool>;
#[doc = "Field `SYS_SHDN_RST` writer - When this bit is asserted 1, the SYS_SHDN# output is deasserted"]
pub type SYS_SHDN_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SHUTDWN_RST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When this bit is asserted 1, the SYS_SHDN# output is deasserted"]
    #[inline(always)]
    pub fn sys_shdn_rst(&self) -> SYS_SHDN_RST_R {
        SYS_SHDN_RST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is asserted 1, the SYS_SHDN# output is deasserted"]
    #[inline(always)]
    pub fn sys_shdn_rst(&mut self) -> SYS_SHDN_RST_W<0> {
        SYS_SHDN_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES HASH Byte Swap Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_shutdwn_rst](index.html) module"]
pub struct SYS_SHUTDWN_RST_SPEC;
impl crate::RegisterSpec for SYS_SHUTDWN_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_shutdwn_rst::R](R) reader structure"]
impl crate::Readable for SYS_SHUTDWN_RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_shutdwn_rst::W](W) writer structure"]
impl crate::Writable for SYS_SHUTDWN_RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SHUTDWN_RST to value 0"]
impl crate::Resettable for SYS_SHUTDWN_RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
