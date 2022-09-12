#[doc = "Register `ALARM_CNT` reader"]
pub struct R(crate::R<ALARM_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARM_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARM_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARM_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARM_CNT` writer"]
pub struct W(crate::W<ALARM_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARM_CNT_SPEC>;
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
impl From<crate::W<ALARM_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARM_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WK_CNTR` reader - While the WT_ENABLE bit is 1, this register is incremented at a 1 Hz rate. Writes of this register may require one second\n to take effect. Reads return the current state of the register. Reads and writes complete independently of the state of WT_ENABLE."]
pub type WK_CNTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WK_CNTR` writer - While the WT_ENABLE bit is 1, this register is incremented at a 1 Hz rate. Writes of this register may require one second\n to take effect. Reads return the current state of the register. Reads and writes complete independently of the state of WT_ENABLE."]
pub type WK_CNTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALARM_CNT_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - While the WT_ENABLE bit is 1, this register is incremented at a 1 Hz rate. Writes of this register may require one second\n to take effect. Reads return the current state of the register. Reads and writes complete independently of the state of WT_ENABLE."]
    #[inline(always)]
    pub fn wk_cntr(&self) -> WK_CNTR_R {
        WK_CNTR_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - While the WT_ENABLE bit is 1, this register is incremented at a 1 Hz rate. Writes of this register may require one second\n to take effect. Reads return the current state of the register. Reads and writes complete independently of the state of WT_ENABLE."]
    #[inline(always)]
    pub fn wk_cntr(&mut self) -> WK_CNTR_W<0> {
        WK_CNTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Week Alarm Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarm_cnt](index.html) module"]
pub struct ALARM_CNT_SPEC;
impl crate::RegisterSpec for ALARM_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarm_cnt::R](R) reader structure"]
impl crate::Readable for ALARM_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarm_cnt::W](W) writer structure"]
impl crate::Writable for ALARM_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALARM_CNT to value 0"]
impl crate::Resettable for ALARM_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
