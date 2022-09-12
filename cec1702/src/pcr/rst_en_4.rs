#[doc = "Register `RST_EN_4` reader"]
pub struct R(crate::R<RST_EN_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_EN_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_EN_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_EN_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_EN_4` writer"]
pub struct W(crate::W<RST_EN_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_EN_4_SPEC>;
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
impl From<crate::W<RST_EN_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_EN_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM10_RST_EN` reader - PWM10 Reset Enable"]
pub type PWM10_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `PWM10_RST_EN` writer - PWM10 Reset Enable"]
pub type PWM10_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER0_RST_EN` reader - CNT_TMER0 Reset Enable"]
pub type CNT_TMER0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER0_RST_EN` writer - CNT_TMER0 Reset Enable"]
pub type CNT_TMER0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER1_RST_EN` reader - CNT_TMER1 Reset Enable"]
pub type CNT_TMER1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER1_RST_EN` writer - CNT_TMER1 Reset Enable"]
pub type CNT_TMER1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER2_RST_EN` reader - CNT_TMER2 Reset Enable"]
pub type CNT_TMER2_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER2_RST_EN` writer - CNT_TMER2 Reset Enable"]
pub type CNT_TMER2_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER3_RST_EN` reader - CNT_TMER3 Reset Enable"]
pub type CNT_TMER3_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER3_RST_EN` writer - CNT_TMER3 Reset Enable"]
pub type CNT_TMER3_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
#[doc = "Field `RTOS_RST_EN` reader - PWM6 Reset Enable"]
pub type RTOS_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTOS_RST_EN` writer - PWM6 Reset Enable"]
pub type RTOS_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
#[doc = "Field `RPMPWM1_RST_EN` reader - RPMPWM 1 Reset Enable"]
pub type RPMPWM1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `RPMPWM1_RST_EN` writer - RPMPWM 1 Reset Enable"]
pub type RPMPWM1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
#[doc = "Field `QSPI_RST_EN` reader - Quad SPI Reset Enable"]
pub type QSPI_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `QSPI_RST_EN` writer - Quad SPI Reset Enable"]
pub type QSPI_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
#[doc = "Field `RC_ID1_RST_EN` reader - RC_ID1 Reset Enable"]
pub type RC_ID1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `RC_ID1_RST_EN` writer - RC_ID1 Reset Enable"]
pub type RC_ID1_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
#[doc = "Field `RC_ID2_RST_EN` reader - RC_ID2 Reset Enable"]
pub type RC_ID2_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `RC_ID2_RST_EN` writer - RC_ID2 Reset Enable"]
pub type RC_ID2_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
#[doc = "Field `PROCHOT_RST_EN` reader - PROCHOT Reset Enable"]
pub type PROCHOT_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `PROCHOT_RST_EN` writer - PROCHOT Reset Enable"]
pub type PROCHOT_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
#[doc = "Field `FUJICL_RST_EN` reader - Fujitsu Custom Logic Reset Enable"]
pub type FUJICL_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `FUJICL_RST_EN` writer - Fujitsu Custom Logic Reset Enable"]
pub type FUJICL_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_EN_4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PWM10 Reset Enable"]
    #[inline(always)]
    pub fn pwm10_rst_en(&self) -> PWM10_RST_EN_R {
        PWM10_RST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CNT_TMER0 Reset Enable"]
    #[inline(always)]
    pub fn cnt_tmer0_rst_en(&self) -> CNT_TMER0_RST_EN_R {
        CNT_TMER0_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CNT_TMER1 Reset Enable"]
    #[inline(always)]
    pub fn cnt_tmer1_rst_en(&self) -> CNT_TMER1_RST_EN_R {
        CNT_TMER1_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CNT_TMER2 Reset Enable"]
    #[inline(always)]
    pub fn cnt_tmer2_rst_en(&self) -> CNT_TMER2_RST_EN_R {
        CNT_TMER2_RST_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CNT_TMER3 Reset Enable"]
    #[inline(always)]
    pub fn cnt_tmer3_rst_en(&self) -> CNT_TMER3_RST_EN_R {
        CNT_TMER3_RST_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM6 Reset Enable"]
    #[inline(always)]
    pub fn rtos_rst_en(&self) -> RTOS_RST_EN_R {
        RTOS_RST_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RPMPWM 1 Reset Enable"]
    #[inline(always)]
    pub fn rpmpwm1_rst_en(&self) -> RPMPWM1_RST_EN_R {
        RPMPWM1_RST_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Quad SPI Reset Enable"]
    #[inline(always)]
    pub fn qspi_rst_en(&self) -> QSPI_RST_EN_R {
        QSPI_RST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - RC_ID1 Reset Enable"]
    #[inline(always)]
    pub fn rc_id1_rst_en(&self) -> RC_ID1_RST_EN_R {
        RC_ID1_RST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RC_ID2 Reset Enable"]
    #[inline(always)]
    pub fn rc_id2_rst_en(&self) -> RC_ID2_RST_EN_R {
        RC_ID2_RST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PROCHOT Reset Enable"]
    #[inline(always)]
    pub fn prochot_rst_en(&self) -> PROCHOT_RST_EN_R {
        PROCHOT_RST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Fujitsu Custom Logic Reset Enable"]
    #[inline(always)]
    pub fn fujicl_rst_en(&self) -> FUJICL_RST_EN_R {
        FUJICL_RST_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM10 Reset Enable"]
    #[inline(always)]
    pub fn pwm10_rst_en(&mut self) -> PWM10_RST_EN_W<0> {
        PWM10_RST_EN_W::new(self)
    }
    #[doc = "Bit 2 - CNT_TMER0 Reset Enable"]
    #[inline(always)]
    pub fn cnt_tmer0_rst_en(&mut self) -> CNT_TMER0_RST_EN_W<2> {
        CNT_TMER0_RST_EN_W::new(self)
    }
    #[doc = "Bit 3 - CNT_TMER1 Reset Enable"]
    #[inline(always)]
    pub fn cnt_tmer1_rst_en(&mut self) -> CNT_TMER1_RST_EN_W<3> {
        CNT_TMER1_RST_EN_W::new(self)
    }
    #[doc = "Bit 4 - CNT_TMER2 Reset Enable"]
    #[inline(always)]
    pub fn cnt_tmer2_rst_en(&mut self) -> CNT_TMER2_RST_EN_W<4> {
        CNT_TMER2_RST_EN_W::new(self)
    }
    #[doc = "Bit 5 - CNT_TMER3 Reset Enable"]
    #[inline(always)]
    pub fn cnt_tmer3_rst_en(&mut self) -> CNT_TMER3_RST_EN_W<5> {
        CNT_TMER3_RST_EN_W::new(self)
    }
    #[doc = "Bit 6 - PWM6 Reset Enable"]
    #[inline(always)]
    pub fn rtos_rst_en(&mut self) -> RTOS_RST_EN_W<6> {
        RTOS_RST_EN_W::new(self)
    }
    #[doc = "Bit 7 - RPMPWM 1 Reset Enable"]
    #[inline(always)]
    pub fn rpmpwm1_rst_en(&mut self) -> RPMPWM1_RST_EN_W<7> {
        RPMPWM1_RST_EN_W::new(self)
    }
    #[doc = "Bit 8 - Quad SPI Reset Enable"]
    #[inline(always)]
    pub fn qspi_rst_en(&mut self) -> QSPI_RST_EN_W<8> {
        QSPI_RST_EN_W::new(self)
    }
    #[doc = "Bit 11 - RC_ID1 Reset Enable"]
    #[inline(always)]
    pub fn rc_id1_rst_en(&mut self) -> RC_ID1_RST_EN_W<11> {
        RC_ID1_RST_EN_W::new(self)
    }
    #[doc = "Bit 12 - RC_ID2 Reset Enable"]
    #[inline(always)]
    pub fn rc_id2_rst_en(&mut self) -> RC_ID2_RST_EN_W<12> {
        RC_ID2_RST_EN_W::new(self)
    }
    #[doc = "Bit 13 - PROCHOT Reset Enable"]
    #[inline(always)]
    pub fn prochot_rst_en(&mut self) -> PROCHOT_RST_EN_W<13> {
        PROCHOT_RST_EN_W::new(self)
    }
    #[doc = "Bit 15 - Fujitsu Custom Logic Reset Enable"]
    #[inline(always)]
    pub fn fujicl_rst_en(&mut self) -> FUJICL_RST_EN_W<15> {
        FUJICL_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Enable 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_en_4](index.html) module"]
pub struct RST_EN_4_SPEC;
impl crate::RegisterSpec for RST_EN_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_en_4::R](R) reader structure"]
impl crate::Readable for RST_EN_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_en_4::W](W) writer structure"]
impl crate::Writable for RST_EN_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_EN_4 to value 0"]
impl crate::Resettable for RST_EN_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
