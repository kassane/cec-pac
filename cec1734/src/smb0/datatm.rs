#[doc = "Register `DATATM` reader"]
pub struct R(crate::R<DATATM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATATM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATATM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATATM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATATM` writer"]
pub struct W(crate::W<DATATM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATATM_SPEC>;
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
impl From<crate::W<DATATM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATATM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_HOLD` reader - The Data Hold \\[7:0\\]
timer determines the SDAT hold time following SCLK driven low."]
pub type DATA_HOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_HOLD` writer - The Data Hold \\[7:0\\]
timer determines the SDAT hold time following SCLK driven low."]
pub type DATA_HOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATATM_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESTART_SETUP` reader - The Restart Setup \\[7:0\\]
timer determines the SDAT setup time from the rising edge of SCLK for a repeated START condition."]
pub type RESTART_SETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESTART_SETUP` writer - The Restart Setup \\[7:0\\]
timer determines the SDAT setup time from the rising edge of SCLK for a repeated START condition."]
pub type RESTART_SETUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATATM_SPEC, u8, u8, 8, O>;
#[doc = "Field `STOP_SETUP` reader - The Stop Setup \\[7:0\\]
timer determines the SDAT setup time from the rising edge of SCLK for a STOP condition."]
pub type STOP_SETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP_SETUP` writer - The Stop Setup \\[7:0\\]
timer determines the SDAT setup time from the rising edge of SCLK for a STOP condition."]
pub type STOP_SETUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATATM_SPEC, u8, u8, 8, O>;
#[doc = "Field `FIRST_START_HOLD` reader - This field determines the SCL hold time following SDA driven low during the first START bit in a transfer. It is the parameter THD:STA in the I2C Specification for an initial START bit. Repeated START hold time is determined by the Repeated START Hold Time Register."]
pub type FIRST_START_HOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIRST_START_HOLD` writer - This field determines the SCL hold time following SDA driven low during the first START bit in a transfer. It is the parameter THD:STA in the I2C Specification for an initial START bit. Repeated START hold time is determined by the Repeated START Hold Time Register."]
pub type FIRST_START_HOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATATM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The Data Hold \\[7:0\\]
timer determines the SDAT hold time following SCLK driven low."]
    #[inline(always)]
    pub fn data_hold(&self) -> DATA_HOLD_R {
        DATA_HOLD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The Restart Setup \\[7:0\\]
timer determines the SDAT setup time from the rising edge of SCLK for a repeated START condition."]
    #[inline(always)]
    pub fn restart_setup(&self) -> RESTART_SETUP_R {
        RESTART_SETUP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The Stop Setup \\[7:0\\]
timer determines the SDAT setup time from the rising edge of SCLK for a STOP condition."]
    #[inline(always)]
    pub fn stop_setup(&self) -> STOP_SETUP_R {
        STOP_SETUP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This field determines the SCL hold time following SDA driven low during the first START bit in a transfer. It is the parameter THD:STA in the I2C Specification for an initial START bit. Repeated START hold time is determined by the Repeated START Hold Time Register."]
    #[inline(always)]
    pub fn first_start_hold(&self) -> FIRST_START_HOLD_R {
        FIRST_START_HOLD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The Data Hold \\[7:0\\]
timer determines the SDAT hold time following SCLK driven low."]
    #[inline(always)]
    pub fn data_hold(&mut self) -> DATA_HOLD_W<0> {
        DATA_HOLD_W::new(self)
    }
    #[doc = "Bits 8:15 - The Restart Setup \\[7:0\\]
timer determines the SDAT setup time from the rising edge of SCLK for a repeated START condition."]
    #[inline(always)]
    pub fn restart_setup(&mut self) -> RESTART_SETUP_W<8> {
        RESTART_SETUP_W::new(self)
    }
    #[doc = "Bits 16:23 - The Stop Setup \\[7:0\\]
timer determines the SDAT setup time from the rising edge of SCLK for a STOP condition."]
    #[inline(always)]
    pub fn stop_setup(&mut self) -> STOP_SETUP_W<16> {
        STOP_SETUP_W::new(self)
    }
    #[doc = "Bits 24:31 - This field determines the SCL hold time following SDA driven low during the first START bit in a transfer. It is the parameter THD:STA in the I2C Specification for an initial START bit. Repeated START hold time is determined by the Repeated START Hold Time Register."]
    #[inline(always)]
    pub fn first_start_hold(&mut self) -> FIRST_START_HOLD_W<24> {
        FIRST_START_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datatm](index.html) module"]
pub struct DATATM_SPEC;
impl crate::RegisterSpec for DATATM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datatm::R](R) reader structure"]
impl crate::Readable for DATATM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datatm::W](W) writer structure"]
impl crate::Writable for DATATM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATATM to value 0x0c4d_5006"]
impl crate::Resettable for DATATM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c4d_5006
    }
}
