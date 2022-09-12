#[doc = "Register `OP_PRMT[%s]` reader"]
pub struct R(crate::R<OP_PRMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OP_PRMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OP_PRMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OP_PRMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OP_PRMT[%s]` writer"]
pub struct W(crate::W<OP_PRMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OP_PRMT_SPEC>;
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
impl From<crate::W<OP_PRMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OP_PRMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRMT` reader - Permit. Each array organized into 8 32-bit registers set per Flash device. 1 = Permit, and if not recognized then ignore. 0 = Kill immediately upon seeing this opcode."]
pub type PRMT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRMT` writer - Permit. Each array organized into 8 32-bit registers set per Flash device. 1 = Permit, and if not recognized then ignore. 0 = Kill immediately upon seeing this opcode."]
pub type PRMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OP_PRMT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Permit. Each array organized into 8 32-bit registers set per Flash device. 1 = Permit, and if not recognized then ignore. 0 = Kill immediately upon seeing this opcode."]
    #[inline(always)]
    pub fn prmt(&self) -> PRMT_R {
        PRMT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Permit. Each array organized into 8 32-bit registers set per Flash device. 1 = Permit, and if not recognized then ignore. 0 = Kill immediately upon seeing this opcode."]
    #[inline(always)]
    pub fn prmt(&mut self) -> PRMT_W<0> {
        PRMT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Permit Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op_prmt](index.html) module"]
pub struct OP_PRMT_SPEC;
impl crate::RegisterSpec for OP_PRMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [op_prmt::R](R) reader structure"]
impl crate::Readable for OP_PRMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [op_prmt::W](W) writer structure"]
impl crate::Writable for OP_PRMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OP_PRMT[%s]
to value 0"]
impl crate::Resettable for OP_PRMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
