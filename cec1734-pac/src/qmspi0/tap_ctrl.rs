#[doc = "Register `TAP_CTRL` reader"]
pub struct R(crate::R<TAP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAP_CTRL` writer"]
pub struct W(crate::W<TAP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAP_CTRL_SPEC>;
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
impl From<crate::W<TAP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTO_MOD` reader - This enables the automatic H/W trim of the Tap."]
pub type AUTO_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUTO_MOD` writer - This enables the automatic H/W trim of the Tap."]
pub type AUTO_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAP_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CTRL` reader - This is a signed value that will be added to the Select Control Tap to come up with the final value for the delay."]
pub type CTRL_R = crate::BitReader<bool>;
#[doc = "Field `CTRL` writer - This is a signed value that will be added to the Select Control Tap to come up with the final value for the delay."]
pub type CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAP_CTRL_SPEC, bool, O>;
#[doc = "Field `FW_GO` reader - This will force the auto-trim H/W to run and find a new trim value."]
pub type FW_GO_R = crate::BitReader<bool>;
#[doc = "Field `FW_GO` writer - This will force the auto-trim H/W to run and find a new trim value."]
pub type FW_GO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAP_CTRL_SPEC, bool, O>;
#[doc = "Field `AUTO_MULT` reader - This will multiply the target delay value the Auto-trim H/W will search for."]
pub type AUTO_MULT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUTO_MULT` writer - This will multiply the target delay value the Auto-trim H/W will search for."]
pub type AUTO_MULT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAP_CTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - This enables the automatic H/W trim of the Tap."]
    #[inline(always)]
    pub fn auto_mod(&self) -> AUTO_MOD_R {
        AUTO_MOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This is a signed value that will be added to the Select Control Tap to come up with the final value for the delay."]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - This will force the auto-trim H/W to run and find a new trim value."]
    #[inline(always)]
    pub fn fw_go(&self) -> FW_GO_R {
        FW_GO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:18 - This will multiply the target delay value the Auto-trim H/W will search for."]
    #[inline(always)]
    pub fn auto_mult(&self) -> AUTO_MULT_R {
        AUTO_MULT_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - This enables the automatic H/W trim of the Tap."]
    #[inline(always)]
    pub fn auto_mod(&mut self) -> AUTO_MOD_W<0> {
        AUTO_MOD_W::new(self)
    }
    #[doc = "Bit 2 - This is a signed value that will be added to the Select Control Tap to come up with the final value for the delay."]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W<2> {
        CTRL_W::new(self)
    }
    #[doc = "Bit 8 - This will force the auto-trim H/W to run and find a new trim value."]
    #[inline(always)]
    pub fn fw_go(&mut self) -> FW_GO_W<8> {
        FW_GO_W::new(self)
    }
    #[doc = "Bits 16:18 - This will multiply the target delay value the Auto-trim H/W will search for."]
    #[inline(always)]
    pub fn auto_mult(&mut self) -> AUTO_MULT_W<16> {
        AUTO_MULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI TAP Adjustment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tap_ctrl](index.html) module"]
pub struct TAP_CTRL_SPEC;
impl crate::RegisterSpec for TAP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tap_ctrl::R](R) reader structure"]
impl crate::Readable for TAP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tap_ctrl::W](W) writer structure"]
impl crate::Writable for TAP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAP_CTRL to value 0"]
impl crate::Resettable for TAP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
