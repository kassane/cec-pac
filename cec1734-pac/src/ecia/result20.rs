#[doc = "Register `RESULT20` reader"]
pub struct R(crate::R<RESULT20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IMSPI` reader - IMSPI"]
pub type IMSPI_R = crate::BitReader<bool>;
#[doc = "Field `CLK_MON` reader - CLK_MON"]
pub type CLK_MON_R = crate::BitReader<bool>;
#[doc = "Field `VTR1_PAD_MON` reader - VTR1_PAD_MON"]
pub type VTR1_PAD_MON_R = crate::BitReader<bool>;
#[doc = "Field `VTR2_PAD_MON` reader - VTR2_PAD_MON"]
pub type VTR2_PAD_MON_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 8 - IMSPI"]
    #[inline(always)]
    pub fn imspi(&self) -> IMSPI_R {
        IMSPI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CLK_MON"]
    #[inline(always)]
    pub fn clk_mon(&self) -> CLK_MON_R {
        CLK_MON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VTR1_PAD_MON"]
    #[inline(always)]
    pub fn vtr1_pad_mon(&self) -> VTR1_PAD_MON_R {
        VTR1_PAD_MON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - VTR2_PAD_MON"]
    #[inline(always)]
    pub fn vtr2_pad_mon(&self) -> VTR2_PAD_MON_R {
        VTR2_PAD_MON_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "GIRQ20 RESULT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result20](index.html) module"]
pub struct RESULT20_SPEC;
impl crate::RegisterSpec for RESULT20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result20::R](R) reader structure"]
impl crate::Readable for RESULT20_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT20 to value 0"]
impl crate::Resettable for RESULT20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
