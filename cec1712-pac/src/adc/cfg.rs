#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKLW_TIM` reader - These bits define the low time count of the ADC clock.\n 0= not used.\n 1= 1 System Clock.\n 2= 2 System Clock.\n"]
pub type CLKLW_TIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKLW_TIM` writer - These bits define the low time count of the ADC clock.\n 0= not used.\n 1= 1 System Clock.\n 2= 2 System Clock.\n"]
pub type CLKLW_TIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLKHIGH_TIM` reader - These bits define the high time count of the ADC clock.\n 0= not used.\n 1= 1 System Clock.\n 2= 2 System Clock.\n"]
pub type CLKHIGH_TIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKHIGH_TIM` writer - These bits define the high time count of the ADC clock.\n 0= not used.\n 1= 1 System Clock.\n 2= 2 System Clock.\n"]
pub type CLKHIGH_TIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLKDUMY_TIM` reader - These bits define the dummy cycles of the ADC clock.\n Valid Values are from 0x0 to 0xF.\n"]
pub type CLKDUMY_TIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDUMY_TIM` writer - These bits define the dummy cycles of the ADC clock.\n Valid Values are from 0x0 to 0xF.\n"]
pub type CLKDUMY_TIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `PWRUP_DLY` reader - These bits define the power up delay in number of micro-seconds.\n Valid Values are from 0x0 to 0xF.\n"]
pub type PWRUP_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWRUP_DLY` writer - These bits define the power up delay in number of micro-seconds.\n Valid Values are from 0x0 to 0xF.\n"]
pub type PWRUP_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DUMYCYC_GAP` reader - These bits define the number of micro-seconds between consective Starts.\n"]
pub type DUMYCYC_GAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUMYCYC_GAP` writer - These bits define the number of micro-seconds between consective Starts.\n"]
pub type DUMYCYC_GAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - These bits define the low time count of the ADC clock.\n 0= not used.\n 1= 1 System Clock.\n 2= 2 System Clock.\n"]
    #[inline(always)]
    pub fn clklw_tim(&self) -> CLKLW_TIM_R {
        CLKLW_TIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - These bits define the high time count of the ADC clock.\n 0= not used.\n 1= 1 System Clock.\n 2= 2 System Clock.\n"]
    #[inline(always)]
    pub fn clkhigh_tim(&self) -> CLKHIGH_TIM_R {
        CLKHIGH_TIM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - These bits define the dummy cycles of the ADC clock.\n Valid Values are from 0x0 to 0xF.\n"]
    #[inline(always)]
    pub fn clkdumy_tim(&self) -> CLKDUMY_TIM_R {
        CLKDUMY_TIM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - These bits define the power up delay in number of micro-seconds.\n Valid Values are from 0x0 to 0xF.\n"]
    #[inline(always)]
    pub fn pwrup_dly(&self) -> PWRUP_DLY_R {
        PWRUP_DLY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - These bits define the number of micro-seconds between consective Starts.\n"]
    #[inline(always)]
    pub fn dumycyc_gap(&self) -> DUMYCYC_GAP_R {
        DUMYCYC_GAP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits define the low time count of the ADC clock.\n 0= not used.\n 1= 1 System Clock.\n 2= 2 System Clock.\n"]
    #[inline(always)]
    pub fn clklw_tim(&mut self) -> CLKLW_TIM_W<0> {
        CLKLW_TIM_W::new(self)
    }
    #[doc = "Bits 8:15 - These bits define the high time count of the ADC clock.\n 0= not used.\n 1= 1 System Clock.\n 2= 2 System Clock.\n"]
    #[inline(always)]
    pub fn clkhigh_tim(&mut self) -> CLKHIGH_TIM_W<8> {
        CLKHIGH_TIM_W::new(self)
    }
    #[doc = "Bits 16:19 - These bits define the dummy cycles of the ADC clock.\n Valid Values are from 0x0 to 0xF.\n"]
    #[inline(always)]
    pub fn clkdumy_tim(&mut self) -> CLKDUMY_TIM_W<16> {
        CLKDUMY_TIM_W::new(self)
    }
    #[doc = "Bits 20:23 - These bits define the power up delay in number of micro-seconds.\n Valid Values are from 0x0 to 0xF.\n"]
    #[inline(always)]
    pub fn pwrup_dly(&mut self) -> PWRUP_DLY_W<20> {
        PWRUP_DLY_W::new(self)
    }
    #[doc = "Bits 24:31 - These bits define the number of micro-seconds between consective Starts.\n"]
    #[inline(always)]
    pub fn dumycyc_gap(&mut self) -> DUMYCYC_GAP_W<24> {
        DUMYCYC_GAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The ADC Configuration Register is used to configure the ADC clock timing.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x0101"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101
    }
}
