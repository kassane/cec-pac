#[doc = "Register `OP_KILLMD[%s]` reader"]
pub struct R(crate::R<OP_KILLMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OP_KILLMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OP_KILLMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OP_KILLMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OP_KILLMD[%s]` writer"]
pub struct W(crate::W<OP_KILLMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OP_KILLMD_SPEC>;
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
impl From<crate::W<OP_KILLMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OP_KILLMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KILL` reader - Kill. Each array organized into 8 32-bit registers set per Flash device. 1 = If Killed, then kill as a Write: Holding CS# low. 0 = If Killed, then kill as a Read: Force CS# high first."]
pub type KILL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KILL` writer - Kill. Each array organized into 8 32-bit registers set per Flash device. 1 = If Killed, then kill as a Write: Holding CS# low. 0 = If Killed, then kill as a Read: Force CS# high first."]
pub type KILL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OP_KILLMD_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Kill. Each array organized into 8 32-bit registers set per Flash device. 1 = If Killed, then kill as a Write: Holding CS# low. 0 = If Killed, then kill as a Read: Force CS# high first."]
    #[inline(always)]
    pub fn kill(&self) -> KILL_R {
        KILL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Kill. Each array organized into 8 32-bit registers set per Flash device. 1 = If Killed, then kill as a Write: Holding CS# low. 0 = If Killed, then kill as a Read: Force CS# high first."]
    #[inline(always)]
    pub fn kill(&mut self) -> KILL_W<0> {
        KILL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Kill Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op_killmd](index.html) module"]
pub struct OP_KILLMD_SPEC;
impl crate::RegisterSpec for OP_KILLMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [op_killmd::R](R) reader structure"]
impl crate::Readable for OP_KILLMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [op_killmd::W](W) writer structure"]
impl crate::Writable for OP_KILLMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OP_KILLMD[%s]
to value 0"]
impl crate::Resettable for OP_KILLMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
