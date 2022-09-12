#[doc = "Register `BGPO_RST` reader"]
pub struct R(crate::R<BGPO_RST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGPO_RST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGPO_RST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGPO_RST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGPO_RST` writer"]
pub struct W(crate::W<BGPO_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGPO_RST_SPEC>;
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
impl From<crate::W<BGPO_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGPO_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BGPO_RESET` reader - Battery powered General Purpose Output reset event. For each bit \\[i\\]
in the field:\n 1=BGPO\\[i\\]
is reset to 0 on RESET_VTR; 0=BGPO\\[i\\]
is reset to 0 on RESET_SYS."]
pub type BGPO_RESET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BGPO_RESET` writer - Battery powered General Purpose Output reset event. For each bit \\[i\\]
in the field:\n 1=BGPO\\[i\\]
is reset to 0 on RESET_VTR; 0=BGPO\\[i\\]
is reset to 0 on RESET_SYS."]
pub type BGPO_RESET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BGPO_RST_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Battery powered General Purpose Output reset event. For each bit \\[i\\]
in the field:\n 1=BGPO\\[i\\]
is reset to 0 on RESET_VTR; 0=BGPO\\[i\\]
is reset to 0 on RESET_SYS."]
    #[inline(always)]
    pub fn bgpo_reset(&self) -> BGPO_RESET_R {
        BGPO_RESET_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Battery powered General Purpose Output reset event. For each bit \\[i\\]
in the field:\n 1=BGPO\\[i\\]
is reset to 0 on RESET_VTR; 0=BGPO\\[i\\]
is reset to 0 on RESET_SYS."]
    #[inline(always)]
    pub fn bgpo_reset(&mut self) -> BGPO_RESET_W<0> {
        BGPO_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BGPO Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgpo_rst](index.html) module"]
pub struct BGPO_RST_SPEC;
impl crate::RegisterSpec for BGPO_RST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgpo_rst::R](R) reader structure"]
impl crate::Readable for BGPO_RST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgpo_rst::W](W) writer structure"]
impl crate::Writable for BGPO_RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGPO_RST to value 0"]
impl crate::Resettable for BGPO_RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
