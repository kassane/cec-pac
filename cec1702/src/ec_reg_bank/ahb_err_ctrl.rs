#[doc = "Register `AHB_ERR_CTRL` reader"]
pub struct R(crate::R<AHB_ERR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_ERR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_ERR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_ERR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_ERR_CTRL` writer"]
pub struct W(crate::W<AHB_ERR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_ERR_CTRL_SPEC>;
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
impl From<crate::W<AHB_ERR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_ERR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Error Control \\[0:0\\]
AHB_ERROR_DISABLE,\n 0: EC memory exceptions are enabled. 1: EC memory exceptions are disabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_err_ctrl](index.html) module"]
pub struct AHB_ERR_CTRL_SPEC;
impl crate::RegisterSpec for AHB_ERR_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ahb_err_ctrl::R](R) reader structure"]
impl crate::Readable for AHB_ERR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_err_ctrl::W](W) writer structure"]
impl crate::Writable for AHB_ERR_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB_ERR_CTRL to value 0"]
impl crate::Resettable for AHB_ERR_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
