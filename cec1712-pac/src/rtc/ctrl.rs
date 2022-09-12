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
#[doc = "Field `BLK_EN` reader - BLOCK_ENABLE This bit must be '1' in order for the block to function internally. Registers may be initialized first, before\n setting this bit to '1' to start operation."]
pub type BLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `BLK_EN` writer - BLOCK_ENABLE This bit must be '1' in order for the block to function internally. Registers may be initialized first, before\n setting this bit to '1' to start operation."]
pub type BLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SOFT_RST` reader - SOFT_RESET A '1' written to this bit position will trigger the RTC_RST reset, resetting the block and all registers except\n this one and the Test Register. This bit is self-clearing at the end of the reset, one cycle of Host Bus Clock later, and so requires\n no waiting."]
pub type SOFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `SOFT_RST` writer - SOFT_RESET A '1' written to this bit position will trigger the RTC_RST reset, resetting the block and all registers except\n this one and the Test Register. This bit is self-clearing at the end of the reset, one cycle of Host Bus Clock later, and so requires\n no waiting."]
pub type SOFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TEST` reader - TEST"]
pub type TEST_R = crate::BitReader<bool>;
#[doc = "Field `TEST` writer - TEST"]
pub type TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ALM_EN` reader - ALARM_ENABLE 1=Enables the Alarm features, 0=Disables the Alarm features"]
pub type ALM_EN_R = crate::BitReader<bool>;
#[doc = "Field `ALM_EN` writer - ALARM_ENABLE 1=Enables the Alarm features, 0=Disables the Alarm features"]
pub type ALM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BLOCK_ENABLE This bit must be '1' in order for the block to function internally. Registers may be initialized first, before\n setting this bit to '1' to start operation."]
    #[inline(always)]
    pub fn blk_en(&self) -> BLK_EN_R {
        BLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SOFT_RESET A '1' written to this bit position will trigger the RTC_RST reset, resetting the block and all registers except\n this one and the Test Register. This bit is self-clearing at the end of the reset, one cycle of Host Bus Clock later, and so requires\n no waiting."]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TEST"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ALARM_ENABLE 1=Enables the Alarm features, 0=Disables the Alarm features"]
    #[inline(always)]
    pub fn alm_en(&self) -> ALM_EN_R {
        ALM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BLOCK_ENABLE This bit must be '1' in order for the block to function internally. Registers may be initialized first, before\n setting this bit to '1' to start operation."]
    #[inline(always)]
    pub fn blk_en(&mut self) -> BLK_EN_W<0> {
        BLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - SOFT_RESET A '1' written to this bit position will trigger the RTC_RST reset, resetting the block and all registers except\n this one and the Test Register. This bit is self-clearing at the end of the reset, one cycle of Host Bus Clock later, and so requires\n no waiting."]
    #[inline(always)]
    pub fn soft_rst(&mut self) -> SOFT_RST_W<1> {
        SOFT_RST_W::new(self)
    }
    #[doc = "Bit 2 - TEST"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W<2> {
        TEST_W::new(self)
    }
    #[doc = "Bit 3 - ALARM_ENABLE 1=Enables the Alarm features, 0=Disables the Alarm features"]
    #[inline(always)]
    pub fn alm_en(&mut self) -> ALM_EN_W<3> {
        ALM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
