#[doc = "Register `PD_MON_CTRL` reader"]
pub struct R(crate::R<PD_MON_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_MON_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_MON_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_MON_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD_MON_CTRL` writer"]
pub struct W(crate::W<PD_MON_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_MON_CTRL_SPEC>;
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
impl From<crate::W<PD_MON_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_MON_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRL_VTR1` reader - Pad Monitor Control VTR1. 0=OFF, 1=1ms delay, 2=10ms delay, 3=100ms delay"]
pub type CTRL_VTR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRL_VTR1` writer - Pad Monitor Control VTR1. 0=OFF, 1=1ms delay, 2=10ms delay, 3=100ms delay"]
pub type CTRL_VTR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PD_MON_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `OVRD_VTR1` reader - PAD Monitor Override VTR1 0=Normal Operation 1=Override input_disable and pad_protect_n"]
pub type OVRD_VTR1_R = crate::BitReader<bool>;
#[doc = "Field `OVRD_VTR1` writer - PAD Monitor Override VTR1 0=Normal Operation 1=Override input_disable and pad_protect_n"]
pub type OVRD_VTR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_CTRL_SPEC, bool, O>;
#[doc = "Field `VTR1_INPT_DIS` reader - PAD Monitor Override Input Disable VTR1. 0=Normal Operation 1=input_disable"]
pub type VTR1_INPT_DIS_R = crate::BitReader<bool>;
#[doc = "Field `VTR1_INPT_DIS` writer - PAD Monitor Override Input Disable VTR1. 0=Normal Operation 1=input_disable"]
pub type VTR1_INPT_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_CTRL_SPEC, bool, O>;
#[doc = "Field `VTR1_PROTECN` reader - PAD Monitor Override Protect N VTR1."]
pub type VTR1_PROTECN_R = crate::BitReader<bool>;
#[doc = "Field `VTR1_PROTECN` writer - PAD Monitor Override Protect N VTR1."]
pub type VTR1_PROTECN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_CTRL_SPEC, bool, O>;
#[doc = "Field `CTRL_VTR2` reader - Pad Monitor Control VTR2. 0=OFF, 1=1ms delay, 2=10ms delay, 3=100ms delay"]
pub type CTRL_VTR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRL_VTR2` writer - Pad Monitor Control VTR2. 0=OFF, 1=1ms delay, 2=10ms delay, 3=100ms delay"]
pub type CTRL_VTR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PD_MON_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `OVRD_VTR2` reader - PAD Monitor Override VTR2 0=Normal Operation 1=Override input_disable and pad_protect_n"]
pub type OVRD_VTR2_R = crate::BitReader<bool>;
#[doc = "Field `OVRD_VTR2` writer - PAD Monitor Override VTR2 0=Normal Operation 1=Override input_disable and pad_protect_n"]
pub type OVRD_VTR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_CTRL_SPEC, bool, O>;
#[doc = "Field `VTR2_INPT_DIS` reader - PAD Monitor Override Input Disable VTR2. 0=Normal Operation 1=input_disable"]
pub type VTR2_INPT_DIS_R = crate::BitReader<bool>;
#[doc = "Field `VTR2_INPT_DIS` writer - PAD Monitor Override Input Disable VTR2. 0=Normal Operation 1=input_disable"]
pub type VTR2_INPT_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_CTRL_SPEC, bool, O>;
#[doc = "Field `VTR2_PROTECN` reader - PAD Monitor Override Protect N VTR2."]
pub type VTR2_PROTECN_R = crate::BitReader<bool>;
#[doc = "Field `VTR2_PROTECN` writer - PAD Monitor Override Protect N VTR2."]
pub type VTR2_PROTECN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_MON_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Pad Monitor Control VTR1. 0=OFF, 1=1ms delay, 2=10ms delay, 3=100ms delay"]
    #[inline(always)]
    pub fn ctrl_vtr1(&self) -> CTRL_VTR1_R {
        CTRL_VTR1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - PAD Monitor Override VTR1 0=Normal Operation 1=Override input_disable and pad_protect_n"]
    #[inline(always)]
    pub fn ovrd_vtr1(&self) -> OVRD_VTR1_R {
        OVRD_VTR1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAD Monitor Override Input Disable VTR1. 0=Normal Operation 1=input_disable"]
    #[inline(always)]
    pub fn vtr1_inpt_dis(&self) -> VTR1_INPT_DIS_R {
        VTR1_INPT_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PAD Monitor Override Protect N VTR1."]
    #[inline(always)]
    pub fn vtr1_protecn(&self) -> VTR1_PROTECN_R {
        VTR1_PROTECN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Pad Monitor Control VTR2. 0=OFF, 1=1ms delay, 2=10ms delay, 3=100ms delay"]
    #[inline(always)]
    pub fn ctrl_vtr2(&self) -> CTRL_VTR2_R {
        CTRL_VTR2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - PAD Monitor Override VTR2 0=Normal Operation 1=Override input_disable and pad_protect_n"]
    #[inline(always)]
    pub fn ovrd_vtr2(&self) -> OVRD_VTR2_R {
        OVRD_VTR2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PAD Monitor Override Input Disable VTR2. 0=Normal Operation 1=input_disable"]
    #[inline(always)]
    pub fn vtr2_inpt_dis(&self) -> VTR2_INPT_DIS_R {
        VTR2_INPT_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PAD Monitor Override Protect N VTR2."]
    #[inline(always)]
    pub fn vtr2_protecn(&self) -> VTR2_PROTECN_R {
        VTR2_PROTECN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pad Monitor Control VTR1. 0=OFF, 1=1ms delay, 2=10ms delay, 3=100ms delay"]
    #[inline(always)]
    pub fn ctrl_vtr1(&mut self) -> CTRL_VTR1_W<0> {
        CTRL_VTR1_W::new(self)
    }
    #[doc = "Bit 4 - PAD Monitor Override VTR1 0=Normal Operation 1=Override input_disable and pad_protect_n"]
    #[inline(always)]
    pub fn ovrd_vtr1(&mut self) -> OVRD_VTR1_W<4> {
        OVRD_VTR1_W::new(self)
    }
    #[doc = "Bit 5 - PAD Monitor Override Input Disable VTR1. 0=Normal Operation 1=input_disable"]
    #[inline(always)]
    pub fn vtr1_inpt_dis(&mut self) -> VTR1_INPT_DIS_W<5> {
        VTR1_INPT_DIS_W::new(self)
    }
    #[doc = "Bit 6 - PAD Monitor Override Protect N VTR1."]
    #[inline(always)]
    pub fn vtr1_protecn(&mut self) -> VTR1_PROTECN_W<6> {
        VTR1_PROTECN_W::new(self)
    }
    #[doc = "Bits 8:9 - Pad Monitor Control VTR2. 0=OFF, 1=1ms delay, 2=10ms delay, 3=100ms delay"]
    #[inline(always)]
    pub fn ctrl_vtr2(&mut self) -> CTRL_VTR2_W<8> {
        CTRL_VTR2_W::new(self)
    }
    #[doc = "Bit 12 - PAD Monitor Override VTR2 0=Normal Operation 1=Override input_disable and pad_protect_n"]
    #[inline(always)]
    pub fn ovrd_vtr2(&mut self) -> OVRD_VTR2_W<12> {
        OVRD_VTR2_W::new(self)
    }
    #[doc = "Bit 13 - PAD Monitor Override Input Disable VTR2. 0=Normal Operation 1=input_disable"]
    #[inline(always)]
    pub fn vtr2_inpt_dis(&mut self) -> VTR2_INPT_DIS_W<13> {
        VTR2_INPT_DIS_W::new(self)
    }
    #[doc = "Bit 14 - PAD Monitor Override Protect N VTR2."]
    #[inline(always)]
    pub fn vtr2_protecn(&mut self) -> VTR2_PROTECN_W<14> {
        VTR2_PROTECN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAD Monitor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_mon_ctrl](index.html) module"]
pub struct PD_MON_CTRL_SPEC;
impl crate::RegisterSpec for PD_MON_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_mon_ctrl::R](R) reader structure"]
impl crate::Readable for PD_MON_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_mon_ctrl::W](W) writer structure"]
impl crate::Writable for PD_MON_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD_MON_CTRL to value 0"]
impl crate::Resettable for PD_MON_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
