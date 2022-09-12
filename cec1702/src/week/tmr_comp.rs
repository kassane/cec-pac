#[doc = "Register `TMR_COMP` reader"]
pub struct R(crate::R<TMR_COMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_COMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_COMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_COMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR_COMP` writer"]
pub struct W(crate::W<TMR_COMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR_COMP_SPEC>;
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
impl From<crate::W<TMR_COMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR_COMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WK_COMP` reader - A Week Alarm Interrupt and a Week Alarm Power-Up Event are asserted when the Week Alarm Counter Register is greater than\n or equal to the contents of this register. Reads and writes complete independently of the state of WT_ENABLE."]
pub type WK_COMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WK_COMP` writer - A Week Alarm Interrupt and a Week Alarm Power-Up Event are asserted when the Week Alarm Counter Register is greater than\n or equal to the contents of this register. Reads and writes complete independently of the state of WT_ENABLE."]
pub type WK_COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMR_COMP_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - A Week Alarm Interrupt and a Week Alarm Power-Up Event are asserted when the Week Alarm Counter Register is greater than\n or equal to the contents of this register. Reads and writes complete independently of the state of WT_ENABLE."]
    #[inline(always)]
    pub fn wk_comp(&self) -> WK_COMP_R {
        WK_COMP_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - A Week Alarm Interrupt and a Week Alarm Power-Up Event are asserted when the Week Alarm Counter Register is greater than\n or equal to the contents of this register. Reads and writes complete independently of the state of WT_ENABLE."]
    #[inline(always)]
    pub fn wk_comp(&mut self) -> WK_COMP_W<0> {
        WK_COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Week Timer Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr_comp](index.html) module"]
pub struct TMR_COMP_SPEC;
impl crate::RegisterSpec for TMR_COMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr_comp::R](R) reader structure"]
impl crate::Readable for TMR_COMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr_comp::W](W) writer structure"]
impl crate::Writable for TMR_COMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR_COMP to value 0x0fff_ffff"]
impl crate::Resettable for TMR_COMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_ffff
    }
}
