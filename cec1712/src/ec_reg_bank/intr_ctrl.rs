#[doc = "Register `INTR_CTRL` reader"]
pub struct R(crate::R<INTR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_CTRL` writer"]
pub struct W(crate::W<INTR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_CTRL_SPEC>;
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
impl From<crate::W<INTR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control \\[0:0\\]
NVIC_EN (NVIC_EN) This bit enables Alternate NVIC IRQ's Vectors. The Alternate NVIC Vectors provides each interrupt event with a dedicated (direct) NVIC vector.\n 0 = Alternate NVIC vectors disabled, 1= Alternate NVIC vectors enabled\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_ctrl](index.html) module"]
pub struct INTR_CTRL_SPEC;
impl crate::RegisterSpec for INTR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_ctrl::R](R) reader structure"]
impl crate::Readable for INTR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_ctrl::W](W) writer structure"]
impl crate::Writable for INTR_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_CTRL to value 0x01"]
impl crate::Resettable for INTR_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
