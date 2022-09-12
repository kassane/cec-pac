#[doc = "Register `SYS_SLP_CTRL` reader"]
pub struct R(crate::R<SYS_SLP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SLP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SLP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SLP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SLP_CTRL` writer"]
pub struct W(crate::W<SYS_SLP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SLP_CTRL_SPEC>;
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
impl From<crate::W<SYS_SLP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SLP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_MOD` reader - Selects the System Sleep mode"]
pub type SLP_MOD_R = crate::BitReader<bool>;
#[doc = "Field `SLP_MOD` writer - Selects the System Sleep mode"]
pub type SLP_MOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SLP_CTRL_SPEC, bool, O>;
#[doc = "Field `TEST` reader - Test bit"]
pub type TEST_R = crate::BitReader<bool>;
#[doc = "Field `TEST` writer - Test bit"]
pub type TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SLP_CTRL_SPEC, bool, O>;
#[doc = "Field `SLP_ALL` reader - Initiates the System Sleep mode"]
pub type SLP_ALL_R = crate::BitReader<bool>;
#[doc = "Field `SLP_ALL` writer - Initiates the System Sleep mode"]
pub type SLP_ALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYS_SLP_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Selects the System Sleep mode"]
    #[inline(always)]
    pub fn slp_mod(&self) -> SLP_MOD_R {
        SLP_MOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Test bit"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Initiates the System Sleep mode"]
    #[inline(always)]
    pub fn slp_all(&self) -> SLP_ALL_R {
        SLP_ALL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the System Sleep mode"]
    #[inline(always)]
    pub fn slp_mod(&mut self) -> SLP_MOD_W<0> {
        SLP_MOD_W::new(self)
    }
    #[doc = "Bit 2 - Test bit"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W<2> {
        TEST_W::new(self)
    }
    #[doc = "Bit 3 - Initiates the System Sleep mode"]
    #[inline(always)]
    pub fn slp_all(&mut self) -> SLP_ALL_W<3> {
        SLP_ALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Sleep Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_slp_ctrl](index.html) module"]
pub struct SYS_SLP_CTRL_SPEC;
impl crate::RegisterSpec for SYS_SLP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_slp_ctrl::R](R) reader structure"]
impl crate::Readable for SYS_SLP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_slp_ctrl::W](W) writer structure"]
impl crate::Writable for SYS_SLP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SLP_CTRL to value 0"]
impl crate::Resettable for SYS_SLP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
