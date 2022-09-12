#[doc = "Register `DAYLT_SAVF` reader"]
pub struct R(crate::R<DAYLT_SAVF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAYLT_SAVF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAYLT_SAVF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAYLT_SAVF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAYLT_SAVF` writer"]
pub struct W(crate::W<DAYLT_SAVF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAYLT_SAVF_SPEC>;
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
impl From<crate::W<DAYLT_SAVF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAYLT_SAVF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DST_MON` reader - This field matches the Month Register."]
pub type DST_MON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DST_MON` writer - This field matches the Month Register."]
pub type DST_MON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAYLT_SAVF_SPEC, u8, u8, 8, O>;
#[doc = "Field `DST_DAY_OF_WK` reader - This field matches the Day of Week Register bits\\[2:0\\]."]
pub type DST_DAY_OF_WK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DST_DAY_OF_WK` writer - This field matches the Day of Week Register bits\\[2:0\\]."]
pub type DST_DAY_OF_WK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAYLT_SAVF_SPEC, u8, u8, 3, O>;
#[doc = "Field `DST_WK` reader - 5=Last week of month, 4 =Fourth week of month, 3=Third week of month, 2=Second week of month, 1=First week of month"]
pub type DST_WK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DST_WK` writer - 5=Last week of month, 4 =Fourth week of month, 3=Third week of month, 2=Second week of month, 1=First week of month"]
pub type DST_WK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAYLT_SAVF_SPEC, u8, u8, 3, O>;
#[doc = "Field `DST_HR` reader - This field holds the matching value for bits\\[6:0\\]
of the Hours register. The written value will be interpreted according\n to the 24/12 Hour mode and DM mode settings at the time of writing."]
pub type DST_HR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DST_HR` writer - This field holds the matching value for bits\\[6:0\\]
of the Hours register. The written value will be interpreted according\n to the 24/12 Hour mode and DM mode settings at the time of writing."]
pub type DST_HR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAYLT_SAVF_SPEC, u8, u8, 7, O>;
#[doc = "Field `DST_AM_PM` reader - This bit selects AM vs. PM, to match bit\\[7\\]
of the Hours Register if 12-Hour mode is selected in Register B at the time\n of writing."]
pub type DST_AM_PM_R = crate::BitReader<bool>;
#[doc = "Field `DST_AM_PM` writer - This bit selects AM vs. PM, to match bit\\[7\\]
of the Hours Register if 12-Hour mode is selected in Register B at the time\n of writing."]
pub type DST_AM_PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAYLT_SAVF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - This field matches the Month Register."]
    #[inline(always)]
    pub fn dst_mon(&self) -> DST_MON_R {
        DST_MON_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - This field matches the Day of Week Register bits\\[2:0\\]."]
    #[inline(always)]
    pub fn dst_day_of_wk(&self) -> DST_DAY_OF_WK_R {
        DST_DAY_OF_WK_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 5=Last week of month, 4 =Fourth week of month, 3=Third week of month, 2=Second week of month, 1=First week of month"]
    #[inline(always)]
    pub fn dst_wk(&self) -> DST_WK_R {
        DST_WK_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:30 - This field holds the matching value for bits\\[6:0\\]
of the Hours register. The written value will be interpreted according\n to the 24/12 Hour mode and DM mode settings at the time of writing."]
    #[inline(always)]
    pub fn dst_hr(&self) -> DST_HR_R {
        DST_HR_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - This bit selects AM vs. PM, to match bit\\[7\\]
of the Hours Register if 12-Hour mode is selected in Register B at the time\n of writing."]
    #[inline(always)]
    pub fn dst_am_pm(&self) -> DST_AM_PM_R {
        DST_AM_PM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - This field matches the Month Register."]
    #[inline(always)]
    pub fn dst_mon(&mut self) -> DST_MON_W<0> {
        DST_MON_W::new(self)
    }
    #[doc = "Bits 8:10 - This field matches the Day of Week Register bits\\[2:0\\]."]
    #[inline(always)]
    pub fn dst_day_of_wk(&mut self) -> DST_DAY_OF_WK_W<8> {
        DST_DAY_OF_WK_W::new(self)
    }
    #[doc = "Bits 16:18 - 5=Last week of month, 4 =Fourth week of month, 3=Third week of month, 2=Second week of month, 1=First week of month"]
    #[inline(always)]
    pub fn dst_wk(&mut self) -> DST_WK_W<16> {
        DST_WK_W::new(self)
    }
    #[doc = "Bits 24:30 - This field holds the matching value for bits\\[6:0\\]
of the Hours register. The written value will be interpreted according\n to the 24/12 Hour mode and DM mode settings at the time of writing."]
    #[inline(always)]
    pub fn dst_hr(&mut self) -> DST_HR_W<24> {
        DST_HR_W::new(self)
    }
    #[doc = "Bit 31 - This bit selects AM vs. PM, to match bit\\[7\\]
of the Hours Register if 12-Hour mode is selected in Register B at the time\n of writing."]
    #[inline(always)]
    pub fn dst_am_pm(&mut self) -> DST_AM_PM_W<31> {
        DST_AM_PM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Daylight Savings Forward Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daylt_savf](index.html) module"]
pub struct DAYLT_SAVF_SPEC;
impl crate::RegisterSpec for DAYLT_SAVF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daylt_savf::R](R) reader structure"]
impl crate::Readable for DAYLT_SAVF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daylt_savf::W](W) writer structure"]
impl crate::Writable for DAYLT_SAVF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAYLT_SAVF to value 0"]
impl crate::Resettable for DAYLT_SAVF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
