#[doc = "Register `VREF_CTRL` reader"]
pub struct R(crate::R<VREF_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREF_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREF_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREF_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREF_CTRL` writer"]
pub struct W(crate::W<VREF_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREF_CTRL_SPEC>;
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
impl From<crate::W<VREF_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREF_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHRG_DLY` reader - This field represnts the delay time to charge up the external VREF capacitor.\n"]
pub type CHRG_DLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CHRG_DLY` writer - This field represnts the delay time to charge up the external VREF capacitor.\n"]
pub type CHRG_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CTRL_SPEC, u16, u16, 16, O>;
#[doc = "Field `SWITCH_DLY` reader - This field represnts the delay time interval between switching VREF Selects.\n"]
pub type SWITCH_DLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SWITCH_DLY` writer - This field represnts the delay time interval between switching VREF Selects.\n"]
pub type SWITCH_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VREF_CTRL_SPEC, u16, u16, 13, O>;
#[doc = "Field `PADCTRL` reader - This fields give the choice to the application whether to float the unused PAD's or to Drive them to 0.\n 1= Drive unused PAD's Low 0b\n 0= Leave unused PAD's Floating.\n"]
pub type PADCTRL_R = crate::BitReader<bool>;
#[doc = "Field `PADCTRL` writer - This fields give the choice to the application whether to float the unused PAD's or to Drive them to 0.\n 1= Drive unused PAD's Low 0b\n 0= Leave unused PAD's Floating.\n"]
pub type PADCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREF_CTRL_SPEC, bool, O>;
#[doc = "Field `SELSTAT` reader - This fields gives information about the current VREF selected.\n 0x0= VREF0\n 0x1= VREF1\n 0x2= Reserved\n 0x3= Reserved\n"]
pub type SELSTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELSTAT` writer - This fields gives information about the current VREF selected.\n 0x0= VREF0\n 0x1= VREF1\n 0x2= Reserved\n 0x3= Reserved\n"]
pub type SELSTAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREF_CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:15 - This field represnts the delay time to charge up the external VREF capacitor.\n"]
    #[inline(always)]
    pub fn chrg_dly(&self) -> CHRG_DLY_R {
        CHRG_DLY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:28 - This field represnts the delay time interval between switching VREF Selects.\n"]
    #[inline(always)]
    pub fn switch_dly(&self) -> SWITCH_DLY_R {
        SWITCH_DLY_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 29 - This fields give the choice to the application whether to float the unused PAD's or to Drive them to 0.\n 1= Drive unused PAD's Low 0b\n 0= Leave unused PAD's Floating.\n"]
    #[inline(always)]
    pub fn padctrl(&self) -> PADCTRL_R {
        PADCTRL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - This fields gives information about the current VREF selected.\n 0x0= VREF0\n 0x1= VREF1\n 0x2= Reserved\n 0x3= Reserved\n"]
    #[inline(always)]
    pub fn selstat(&self) -> SELSTAT_R {
        SELSTAT_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field represnts the delay time to charge up the external VREF capacitor.\n"]
    #[inline(always)]
    pub fn chrg_dly(&mut self) -> CHRG_DLY_W<0> {
        CHRG_DLY_W::new(self)
    }
    #[doc = "Bits 16:28 - This field represnts the delay time interval between switching VREF Selects.\n"]
    #[inline(always)]
    pub fn switch_dly(&mut self) -> SWITCH_DLY_W<16> {
        SWITCH_DLY_W::new(self)
    }
    #[doc = "Bit 29 - This fields give the choice to the application whether to float the unused PAD's or to Drive them to 0.\n 1= Drive unused PAD's Low 0b\n 0= Leave unused PAD's Floating.\n"]
    #[inline(always)]
    pub fn padctrl(&mut self) -> PADCTRL_W<29> {
        PADCTRL_W::new(self)
    }
    #[doc = "Bits 30:31 - This fields gives information about the current VREF selected.\n 0x0= VREF0\n 0x1= VREF1\n 0x2= Reserved\n 0x3= Reserved\n"]
    #[inline(always)]
    pub fn selstat(&mut self) -> SELSTAT_W<30> {
        SELSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the VREF Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vref_ctrl](index.html) module"]
pub struct VREF_CTRL_SPEC;
impl crate::RegisterSpec for VREF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vref_ctrl::R](R) reader structure"]
impl crate::Readable for VREF_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vref_ctrl::W](W) writer structure"]
impl crate::Writable for VREF_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREF_CTRL to value 0"]
impl crate::Resettable for VREF_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
