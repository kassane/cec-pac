#[doc = "Register `TMOUTSC` reader"]
pub struct R(crate::R<TMOUTSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMOUTSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMOUTSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMOUTSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMOUTSC` writer"]
pub struct W(crate::W<TMOUTSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMOUTSC_SPEC>;
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
impl From<crate::W<TMOUTSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMOUTSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUS_IDLE_MIN` reader - Bus Idle Minimum time = Bus Idle Min \\[7:0\\]
x Baud_Clock_Period"]
pub type BUS_IDLE_MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUS_IDLE_MIN` writer - Bus Idle Minimum time = Bus Idle Min \\[7:0\\]
x Baud_Clock_Period"]
pub type BUS_IDLE_MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMOUTSC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - Bus Idle Minimum time = Bus Idle Min \\[7:0\\]
x Baud_Clock_Period"]
    #[inline(always)]
    pub fn bus_idle_min(&self) -> BUS_IDLE_MIN_R {
        BUS_IDLE_MIN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Bus Idle Minimum time = Bus Idle Min \\[7:0\\]
x Baud_Clock_Period"]
    #[inline(always)]
    pub fn bus_idle_min(&mut self) -> BUS_IDLE_MIN_W<24> {
        BUS_IDLE_MIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time-Out Scaling Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmoutsc](index.html) module"]
pub struct TMOUTSC_SPEC;
impl crate::RegisterSpec for TMOUTSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmoutsc::R](R) reader structure"]
impl crate::Readable for TMOUTSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmoutsc::W](W) writer structure"]
impl crate::Writable for TMOUTSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMOUTSC to value 0x4b9c_c2c7"]
impl crate::Resettable for TMOUTSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4b9c_c2c7
    }
}
