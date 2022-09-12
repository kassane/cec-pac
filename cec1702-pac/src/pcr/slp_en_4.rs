#[doc = "Register `SLP_EN_4` reader"]
pub struct R(crate::R<SLP_EN_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_EN_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_EN_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_EN_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_EN_4` writer"]
pub struct W(crate::W<SLP_EN_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_EN_4_SPEC>;
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
impl From<crate::W<SLP_EN_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_EN_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM10_SLP_EN` reader - PWM10 Sleep Enable"]
pub type PWM10_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM10_SLP_EN` writer - PWM10 Sleep Enable"]
pub type PWM10_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER0_SLP_EN` reader - CNT_TMER0 Sleep Enable"]
pub type CNT_TMER0_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER0_SLP_EN` writer - CNT_TMER0 Sleep Enable"]
pub type CNT_TMER0_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER1_SLP_EN` reader - CNT_TMER1 Sleep Enable"]
pub type CNT_TMER1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER1_SLP_EN` writer - CNT_TMER1 Sleep Enable"]
pub type CNT_TMER1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER2_SLP_EN` reader - CNT_TMER2 Sleep Enable"]
pub type CNT_TMER2_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER2_SLP_EN` writer - CNT_TMER2 Sleep Enable"]
pub type CNT_TMER2_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER3_SLP_EN` reader - CNT_TMER3 Sleep Enable"]
pub type CNT_TMER3_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER3_SLP_EN` writer - CNT_TMER3 Sleep Enable"]
pub type CNT_TMER3_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `RTOS_SLP_EN` reader - PWM6 Sleep Enable"]
pub type RTOS_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTOS_SLP_EN` writer - PWM6 Sleep Enable"]
pub type RTOS_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `RPMPWM1_SLP_EN` reader - RPMPWM 1 Sleep Enable"]
pub type RPMPWM1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `RPMPWM1_SLP_EN` writer - RPMPWM 1 Sleep Enable"]
pub type RPMPWM1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `QMSPI_SLP_EN` reader - Quad SPI Sleep Enable"]
pub type QMSPI_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `QMSPI_SLP_EN` writer - Quad SPI Sleep Enable"]
pub type QMSPI_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `RC_ID1_SLP_EN` reader - RC_ID1 Sleep Enable"]
pub type RC_ID1_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `RC_ID1_SLP_EN` writer - RC_ID1 Sleep Enable"]
pub type RC_ID1_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `RC_ID2_SLP_EN` reader - RC_ID2 Sleep Enable"]
pub type RC_ID2_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `RC_ID2_SLP_EN` writer - RC_ID2 Sleep Enable"]
pub type RC_ID2_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `PROCHOT_SLP_EN` reader - PROCHOT Sleep Enable"]
pub type PROCHOT_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `PROCHOT_SLP_EN` writer - PROCHOT Sleep Enable"]
pub type PROCHOT_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
#[doc = "Field `FUJICL_SLP_EN` reader - Fujitsu Custom Logic Sleep Enable"]
pub type FUJICL_SLP_EN_R = crate::BitReader<bool>;
#[doc = "Field `FUJICL_SLP_EN` writer - Fujitsu Custom Logic Sleep Enable"]
pub type FUJICL_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLP_EN_4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PWM10 Sleep Enable"]
    #[inline(always)]
    pub fn pwm10_slp_en(&self) -> PWM10_SLP_EN_R {
        PWM10_SLP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CNT_TMER0 Sleep Enable"]
    #[inline(always)]
    pub fn cnt_tmer0_slp_en(&self) -> CNT_TMER0_SLP_EN_R {
        CNT_TMER0_SLP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CNT_TMER1 Sleep Enable"]
    #[inline(always)]
    pub fn cnt_tmer1_slp_en(&self) -> CNT_TMER1_SLP_EN_R {
        CNT_TMER1_SLP_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CNT_TMER2 Sleep Enable"]
    #[inline(always)]
    pub fn cnt_tmer2_slp_en(&self) -> CNT_TMER2_SLP_EN_R {
        CNT_TMER2_SLP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CNT_TMER3 Sleep Enable"]
    #[inline(always)]
    pub fn cnt_tmer3_slp_en(&self) -> CNT_TMER3_SLP_EN_R {
        CNT_TMER3_SLP_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM6 Sleep Enable"]
    #[inline(always)]
    pub fn rtos_slp_en(&self) -> RTOS_SLP_EN_R {
        RTOS_SLP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RPMPWM 1 Sleep Enable"]
    #[inline(always)]
    pub fn rpmpwm1_slp_en(&self) -> RPMPWM1_SLP_EN_R {
        RPMPWM1_SLP_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Quad SPI Sleep Enable"]
    #[inline(always)]
    pub fn qmspi_slp_en(&self) -> QMSPI_SLP_EN_R {
        QMSPI_SLP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - RC_ID1 Sleep Enable"]
    #[inline(always)]
    pub fn rc_id1_slp_en(&self) -> RC_ID1_SLP_EN_R {
        RC_ID1_SLP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RC_ID2 Sleep Enable"]
    #[inline(always)]
    pub fn rc_id2_slp_en(&self) -> RC_ID2_SLP_EN_R {
        RC_ID2_SLP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PROCHOT Sleep Enable"]
    #[inline(always)]
    pub fn prochot_slp_en(&self) -> PROCHOT_SLP_EN_R {
        PROCHOT_SLP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Fujitsu Custom Logic Sleep Enable"]
    #[inline(always)]
    pub fn fujicl_slp_en(&self) -> FUJICL_SLP_EN_R {
        FUJICL_SLP_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM10 Sleep Enable"]
    #[inline(always)]
    pub fn pwm10_slp_en(&mut self) -> PWM10_SLP_EN_W<0> {
        PWM10_SLP_EN_W::new(self)
    }
    #[doc = "Bit 2 - CNT_TMER0 Sleep Enable"]
    #[inline(always)]
    pub fn cnt_tmer0_slp_en(&mut self) -> CNT_TMER0_SLP_EN_W<2> {
        CNT_TMER0_SLP_EN_W::new(self)
    }
    #[doc = "Bit 3 - CNT_TMER1 Sleep Enable"]
    #[inline(always)]
    pub fn cnt_tmer1_slp_en(&mut self) -> CNT_TMER1_SLP_EN_W<3> {
        CNT_TMER1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 4 - CNT_TMER2 Sleep Enable"]
    #[inline(always)]
    pub fn cnt_tmer2_slp_en(&mut self) -> CNT_TMER2_SLP_EN_W<4> {
        CNT_TMER2_SLP_EN_W::new(self)
    }
    #[doc = "Bit 5 - CNT_TMER3 Sleep Enable"]
    #[inline(always)]
    pub fn cnt_tmer3_slp_en(&mut self) -> CNT_TMER3_SLP_EN_W<5> {
        CNT_TMER3_SLP_EN_W::new(self)
    }
    #[doc = "Bit 6 - PWM6 Sleep Enable"]
    #[inline(always)]
    pub fn rtos_slp_en(&mut self) -> RTOS_SLP_EN_W<6> {
        RTOS_SLP_EN_W::new(self)
    }
    #[doc = "Bit 7 - RPMPWM 1 Sleep Enable"]
    #[inline(always)]
    pub fn rpmpwm1_slp_en(&mut self) -> RPMPWM1_SLP_EN_W<7> {
        RPMPWM1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 8 - Quad SPI Sleep Enable"]
    #[inline(always)]
    pub fn qmspi_slp_en(&mut self) -> QMSPI_SLP_EN_W<8> {
        QMSPI_SLP_EN_W::new(self)
    }
    #[doc = "Bit 11 - RC_ID1 Sleep Enable"]
    #[inline(always)]
    pub fn rc_id1_slp_en(&mut self) -> RC_ID1_SLP_EN_W<11> {
        RC_ID1_SLP_EN_W::new(self)
    }
    #[doc = "Bit 12 - RC_ID2 Sleep Enable"]
    #[inline(always)]
    pub fn rc_id2_slp_en(&mut self) -> RC_ID2_SLP_EN_W<12> {
        RC_ID2_SLP_EN_W::new(self)
    }
    #[doc = "Bit 13 - PROCHOT Sleep Enable"]
    #[inline(always)]
    pub fn prochot_slp_en(&mut self) -> PROCHOT_SLP_EN_W<13> {
        PROCHOT_SLP_EN_W::new(self)
    }
    #[doc = "Bit 15 - Fujitsu Custom Logic Sleep Enable"]
    #[inline(always)]
    pub fn fujicl_slp_en(&mut self) -> FUJICL_SLP_EN_W<15> {
        FUJICL_SLP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Enable 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_en_4](index.html) module"]
pub struct SLP_EN_4_SPEC;
impl crate::RegisterSpec for SLP_EN_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_en_4::R](R) reader structure"]
impl crate::Readable for SLP_EN_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_en_4::W](W) writer structure"]
impl crate::Writable for SLP_EN_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLP_EN_4 to value 0"]
impl crate::Resettable for SLP_EN_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
