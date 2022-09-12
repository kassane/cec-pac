#[doc = "Register `LDN` reader"]
pub struct R(crate::R<LDN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDN` writer"]
pub struct W(crate::W<LDN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDN_SPEC>;
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
impl From<crate::W<LDN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDN_SPEC>) -> Self {
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
#[doc = "A write to this register selects the current logical device. This allows access to the control and configuration\n registers for each logical device. Note: The Activate command operates only on the selected logical device.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldn](index.html) module"]
pub struct LDN_SPEC;
impl crate::RegisterSpec for LDN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ldn::R](R) reader structure"]
impl crate::Readable for LDN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldn::W](W) writer structure"]
impl crate::Writable for LDN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDN to value 0"]
impl crate::Resettable for LDN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
