#[doc = "Register `OP_WPROT[%s]` reader"]
pub struct R(crate::R<OP_WPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OP_WPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OP_WPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OP_WPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OP_WPROT[%s]` writer"]
pub struct W(crate::W<OP_WPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OP_WPROT_SPEC>;
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
impl From<crate::W<OP_WPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OP_WPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WPROT` reader - Write Protect. Each array organized into 8 32-bit registers set per Flash device. 1 = Make the corresponding bits RO in Permit and Killmd regs. 0 = Corresponding bits are R/W."]
pub type WPROT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WPROT` writer - Write Protect. Each array organized into 8 32-bit registers set per Flash device. 1 = Make the corresponding bits RO in Permit and Killmd regs. 0 = Corresponding bits are R/W."]
pub type WPROT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OP_WPROT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Write Protect. Each array organized into 8 32-bit registers set per Flash device. 1 = Make the corresponding bits RO in Permit and Killmd regs. 0 = Corresponding bits are R/W."]
    #[inline(always)]
    pub fn wprot(&self) -> WPROT_R {
        WPROT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write Protect. Each array organized into 8 32-bit registers set per Flash device. 1 = Make the corresponding bits RO in Permit and Killmd regs. 0 = Corresponding bits are R/W."]
    #[inline(always)]
    pub fn wprot(&mut self) -> WPROT_W<0> {
        WPROT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protect Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op_wprot](index.html) module"]
pub struct OP_WPROT_SPEC;
impl crate::RegisterSpec for OP_WPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [op_wprot::R](R) reader structure"]
impl crate::Readable for OP_WPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [op_wprot::W](W) writer structure"]
impl crate::Writable for OP_WPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OP_WPROT[%s]
to value 0"]
impl crate::Resettable for OP_WPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
