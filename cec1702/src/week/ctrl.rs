#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WT_EN` reader - The WT_EN bit is used to start and stop the Week Alarm Counter Register and the Clock Divider Register.\n The value in the Counter Register is held when the WT_ENABLE bit is not asserted (0) and the count is resumed from the last value when the bit is asserted (1).\n The 15-Bit Clock Divider is reset to 00h and the Week Alarm Interface is in its lowest power consumption state when the WT_ENABLE bit is not asserted."]
pub type WT_EN_R = crate::BitReader<bool>;
#[doc = "Field `WT_EN` writer - The WT_EN bit is used to start and stop the Week Alarm Counter Register and the Clock Divider Register.\n The value in the Counter Register is held when the WT_ENABLE bit is not asserted (0) and the count is resumed from the last value when the bit is asserted (1).\n The 15-Bit Clock Divider is reset to 00h and the Week Alarm Interface is in its lowest power consumption state when the WT_ENABLE bit is not asserted."]
pub type WT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PWRUP_EN` reader - This bit controls the state of the Power-Up Event Output and enables Week POWER-UP Event decoding in the VBAT-Powered Control Interface.\n 1=Power-Up Event Output Enabled\n 0=Power-Up Event Output Disabled and Reset"]
pub type PWRUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWRUP_EN` writer - This bit controls the state of the Power-Up Event Output and enables Week POWER-UP Event decoding in the VBAT-Powered Control Interface.\n 1=Power-Up Event Output Enabled\n 0=Power-Up Event Output Disabled and Reset"]
pub type PWRUP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The WT_EN bit is used to start and stop the Week Alarm Counter Register and the Clock Divider Register.\n The value in the Counter Register is held when the WT_ENABLE bit is not asserted (0) and the count is resumed from the last value when the bit is asserted (1).\n The 15-Bit Clock Divider is reset to 00h and the Week Alarm Interface is in its lowest power consumption state when the WT_ENABLE bit is not asserted."]
    #[inline(always)]
    pub fn wt_en(&self) -> WT_EN_R {
        WT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 6 - This bit controls the state of the Power-Up Event Output and enables Week POWER-UP Event decoding in the VBAT-Powered Control Interface.\n 1=Power-Up Event Output Enabled\n 0=Power-Up Event Output Disabled and Reset"]
    #[inline(always)]
    pub fn pwrup_en(&self) -> PWRUP_EN_R {
        PWRUP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The WT_EN bit is used to start and stop the Week Alarm Counter Register and the Clock Divider Register.\n The value in the Counter Register is held when the WT_ENABLE bit is not asserted (0) and the count is resumed from the last value when the bit is asserted (1).\n The 15-Bit Clock Divider is reset to 00h and the Week Alarm Interface is in its lowest power consumption state when the WT_ENABLE bit is not asserted."]
    #[inline(always)]
    pub fn wt_en(&mut self) -> WT_EN_W<0> {
        WT_EN_W::new(self)
    }
    #[doc = "Bit 6 - This bit controls the state of the Power-Up Event Output and enables Week POWER-UP Event decoding in the VBAT-Powered Control Interface.\n 1=Power-Up Event Output Enabled\n 0=Power-Up Event Output Disabled and Reset"]
    #[inline(always)]
    pub fn pwrup_en(&mut self) -> PWRUP_EN_W<6> {
        PWRUP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x01"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
