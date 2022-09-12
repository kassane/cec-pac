#[doc = "Register `BUSCLK` reader"]
pub struct R(crate::R<BUSCLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSCLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSCLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSCLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSCLK` writer"]
pub struct W(crate::W<BUSCLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSCLK_SPEC>;
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
impl From<crate::W<BUSCLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSCLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW_PER` reader - This field defines the number of I2C Baud Clock periods that make up the low phase of the I2C/SMBus bus clock."]
pub type LOW_PER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOW_PER` writer - This field defines the number of I2C Baud Clock periods that make up the low phase of the I2C/SMBus bus clock."]
pub type LOW_PER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUSCLK_SPEC, u8, u8, 8, O>;
#[doc = "Field `HIGH_PER` reader - This field defines the number of I2C Baud Clock periods that make up the high phase of the I2C/SMBus bus clock."]
pub type HIGH_PER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIGH_PER` writer - This field defines the number of I2C Baud Clock periods that make up the high phase of the I2C/SMBus bus clock."]
pub type HIGH_PER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUSCLK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This field defines the number of I2C Baud Clock periods that make up the low phase of the I2C/SMBus bus clock."]
    #[inline(always)]
    pub fn low_per(&self) -> LOW_PER_R {
        LOW_PER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This field defines the number of I2C Baud Clock periods that make up the high phase of the I2C/SMBus bus clock."]
    #[inline(always)]
    pub fn high_per(&self) -> HIGH_PER_R {
        HIGH_PER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This field defines the number of I2C Baud Clock periods that make up the low phase of the I2C/SMBus bus clock."]
    #[inline(always)]
    pub fn low_per(&mut self) -> LOW_PER_W<0> {
        LOW_PER_W::new(self)
    }
    #[doc = "Bits 8:15 - This field defines the number of I2C Baud Clock periods that make up the high phase of the I2C/SMBus bus clock."]
    #[inline(always)]
    pub fn high_per(&mut self) -> HIGH_PER_W<8> {
        HIGH_PER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busclk](index.html) module"]
pub struct BUSCLK_SPEC;
impl crate::RegisterSpec for BUSCLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busclk::R](R) reader structure"]
impl crate::Readable for BUSCLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [busclk::W](W) writer structure"]
impl crate::Writable for BUSCLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUSCLK to value 0x4f4f"]
impl crate::Resettable for BUSCLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4f4f
    }
}
