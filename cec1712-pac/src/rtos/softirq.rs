#[doc = "Register `SOFTIRQ` writer"]
pub struct W(crate::W<SOFTIRQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTIRQ_SPEC>;
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
impl From<crate::W<SOFTIRQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTIRQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI0` writer - Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC.\n Writes of a '0' have no effect. Reads return '0'."]
pub type SWI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOFTIRQ_SPEC, bool, O>;
#[doc = "Field `SWI1` writer - Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC.\n Writes of a '0' have no effect. Reads return '0'."]
pub type SWI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOFTIRQ_SPEC, bool, O>;
#[doc = "Field `SWI2` writer - Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC.\n Writes of a '0' have no effect. Reads return '0'."]
pub type SWI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOFTIRQ_SPEC, bool, O>;
#[doc = "Field `SWI3` writer - Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC.\n Writes of a '0' have no effect. Reads return '0'."]
pub type SWI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOFTIRQ_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC.\n Writes of a '0' have no effect. Reads return '0'."]
    #[inline(always)]
    pub fn swi0(&mut self) -> SWI0_W<0> {
        SWI0_W::new(self)
    }
    #[doc = "Bit 1 - Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC.\n Writes of a '0' have no effect. Reads return '0'."]
    #[inline(always)]
    pub fn swi1(&mut self) -> SWI1_W<1> {
        SWI1_W::new(self)
    }
    #[doc = "Bit 2 - Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC.\n Writes of a '0' have no effect. Reads return '0'."]
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W<2> {
        SWI2_W::new(self)
    }
    #[doc = "Bit 3 - Software Interrupt. A write of a '1' to this bit will generate an SWI interrupt to the EC.\n Writes of a '0' have no effect. Reads return '0'."]
    #[inline(always)]
    pub fn swi3(&mut self) -> SWI3_W<3> {
        SWI3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Soft Interrupt Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softirq](index.html) module"]
pub struct SOFTIRQ_SPEC;
impl crate::RegisterSpec for SOFTIRQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [softirq::W](W) writer structure"]
impl crate::Writable for SOFTIRQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOFTIRQ to value 0"]
impl crate::Resettable for SOFTIRQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
