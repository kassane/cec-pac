#[doc = "Register `CLK_REQ_4` reader"]
pub struct R(crate::R<CLK_REQ_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REQ_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_REQ_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_REQ_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_REQ_4` writer"]
pub struct W(crate::W<CLK_REQ_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_REQ_4_SPEC>;
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
impl From<crate::W<CLK_REQ_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_REQ_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWM10_CLK_REQ` reader - PWM10 Clock Reuqired"]
pub type PWM10_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PWM10_CLK_REQ` writer - PWM10 Clock Reuqired"]
pub type PWM10_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER0_CLK_REQ` reader - CNT_TMER0 Clock Reuqired"]
pub type CNT_TMER0_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER0_CLK_REQ` writer - CNT_TMER0 Clock Reuqired"]
pub type CNT_TMER0_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER1_CLK_REQ` reader - CNT_TMER1 Clock Reuqired"]
pub type CNT_TMER1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER1_CLK_REQ` writer - CNT_TMER1 Clock Reuqired"]
pub type CNT_TMER1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER2_CLK_REQ` reader - CNT_TMER2 Clock Reuqired"]
pub type CNT_TMER2_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER2_CLK_REQ` writer - CNT_TMER2 Clock Reuqired"]
pub type CNT_TMER2_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `CNT_TMER3_CLK_REQ` reader - CNT_TMER3 Clock Reuqired"]
pub type CNT_TMER3_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `CNT_TMER3_CLK_REQ` writer - CNT_TMER3 Clock Reuqired"]
pub type CNT_TMER3_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `RTOS_CLK_REQ` reader - PWM6 Clock Reuqired"]
pub type RTOS_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RTOS_CLK_REQ` writer - PWM6 Clock Reuqired"]
pub type RTOS_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `RPMPWM1_CLK_REQ` reader - RPMPWM 1 Clock Reuqired"]
pub type RPMPWM1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RPMPWM1_CLK_REQ` writer - RPMPWM 1 Clock Reuqired"]
pub type RPMPWM1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `QSPI_CLK_REQ` reader - Quad SPI Clock Reuqired"]
pub type QSPI_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `QSPI_CLK_REQ` writer - Quad SPI Clock Reuqired"]
pub type QSPI_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `RC_ID1_CLK_REQ` reader - RC_ID1 Clock Reuqired"]
pub type RC_ID1_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RC_ID1_CLK_REQ` writer - RC_ID1 Clock Reuqired"]
pub type RC_ID1_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `RC_ID2_CLK_REQ` reader - RC_ID2 Clock Reuqired"]
pub type RC_ID2_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RC_ID2_CLK_REQ` writer - RC_ID2 Clock Reuqired"]
pub type RC_ID2_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `PROCHOT_CLK_REQ` reader - PROCHOT Clock Reuqired"]
pub type PROCHOT_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `PROCHOT_CLK_REQ` writer - PROCHOT Clock Reuqired"]
pub type PROCHOT_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `FUJICL_CLK_REQ` reader - Fujitsu Custom Logic Clock Reuqired"]
pub type FUJICL_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `FUJICL_CLK_REQ` writer - Fujitsu Custom Logic Clock Reuqired"]
pub type FUJICL_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PWM10 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm10_clk_req(&self) -> PWM10_CLK_REQ_R {
        PWM10_CLK_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CNT_TMER0 Clock Reuqired"]
    #[inline(always)]
    pub fn cnt_tmer0_clk_req(&self) -> CNT_TMER0_CLK_REQ_R {
        CNT_TMER0_CLK_REQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CNT_TMER1 Clock Reuqired"]
    #[inline(always)]
    pub fn cnt_tmer1_clk_req(&self) -> CNT_TMER1_CLK_REQ_R {
        CNT_TMER1_CLK_REQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CNT_TMER2 Clock Reuqired"]
    #[inline(always)]
    pub fn cnt_tmer2_clk_req(&self) -> CNT_TMER2_CLK_REQ_R {
        CNT_TMER2_CLK_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CNT_TMER3 Clock Reuqired"]
    #[inline(always)]
    pub fn cnt_tmer3_clk_req(&self) -> CNT_TMER3_CLK_REQ_R {
        CNT_TMER3_CLK_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM6 Clock Reuqired"]
    #[inline(always)]
    pub fn rtos_clk_req(&self) -> RTOS_CLK_REQ_R {
        RTOS_CLK_REQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RPMPWM 1 Clock Reuqired"]
    #[inline(always)]
    pub fn rpmpwm1_clk_req(&self) -> RPMPWM1_CLK_REQ_R {
        RPMPWM1_CLK_REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Quad SPI Clock Reuqired"]
    #[inline(always)]
    pub fn qspi_clk_req(&self) -> QSPI_CLK_REQ_R {
        QSPI_CLK_REQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - RC_ID1 Clock Reuqired"]
    #[inline(always)]
    pub fn rc_id1_clk_req(&self) -> RC_ID1_CLK_REQ_R {
        RC_ID1_CLK_REQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RC_ID2 Clock Reuqired"]
    #[inline(always)]
    pub fn rc_id2_clk_req(&self) -> RC_ID2_CLK_REQ_R {
        RC_ID2_CLK_REQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PROCHOT Clock Reuqired"]
    #[inline(always)]
    pub fn prochot_clk_req(&self) -> PROCHOT_CLK_REQ_R {
        PROCHOT_CLK_REQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Fujitsu Custom Logic Clock Reuqired"]
    #[inline(always)]
    pub fn fujicl_clk_req(&self) -> FUJICL_CLK_REQ_R {
        FUJICL_CLK_REQ_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM10 Clock Reuqired"]
    #[inline(always)]
    pub fn pwm10_clk_req(&mut self) -> PWM10_CLK_REQ_W<0> {
        PWM10_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 2 - CNT_TMER0 Clock Reuqired"]
    #[inline(always)]
    pub fn cnt_tmer0_clk_req(&mut self) -> CNT_TMER0_CLK_REQ_W<2> {
        CNT_TMER0_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 3 - CNT_TMER1 Clock Reuqired"]
    #[inline(always)]
    pub fn cnt_tmer1_clk_req(&mut self) -> CNT_TMER1_CLK_REQ_W<3> {
        CNT_TMER1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 4 - CNT_TMER2 Clock Reuqired"]
    #[inline(always)]
    pub fn cnt_tmer2_clk_req(&mut self) -> CNT_TMER2_CLK_REQ_W<4> {
        CNT_TMER2_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 5 - CNT_TMER3 Clock Reuqired"]
    #[inline(always)]
    pub fn cnt_tmer3_clk_req(&mut self) -> CNT_TMER3_CLK_REQ_W<5> {
        CNT_TMER3_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 6 - PWM6 Clock Reuqired"]
    #[inline(always)]
    pub fn rtos_clk_req(&mut self) -> RTOS_CLK_REQ_W<6> {
        RTOS_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 7 - RPMPWM 1 Clock Reuqired"]
    #[inline(always)]
    pub fn rpmpwm1_clk_req(&mut self) -> RPMPWM1_CLK_REQ_W<7> {
        RPMPWM1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 8 - Quad SPI Clock Reuqired"]
    #[inline(always)]
    pub fn qspi_clk_req(&mut self) -> QSPI_CLK_REQ_W<8> {
        QSPI_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 11 - RC_ID1 Clock Reuqired"]
    #[inline(always)]
    pub fn rc_id1_clk_req(&mut self) -> RC_ID1_CLK_REQ_W<11> {
        RC_ID1_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 12 - RC_ID2 Clock Reuqired"]
    #[inline(always)]
    pub fn rc_id2_clk_req(&mut self) -> RC_ID2_CLK_REQ_W<12> {
        RC_ID2_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 13 - PROCHOT Clock Reuqired"]
    #[inline(always)]
    pub fn prochot_clk_req(&mut self) -> PROCHOT_CLK_REQ_W<13> {
        PROCHOT_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 15 - Fujitsu Custom Logic Clock Reuqired"]
    #[inline(always)]
    pub fn fujicl_clk_req(&mut self) -> FUJICL_CLK_REQ_W<15> {
        FUJICL_CLK_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Required 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_req_4](index.html) module"]
pub struct CLK_REQ_4_SPEC;
impl crate::RegisterSpec for CLK_REQ_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_req_4::R](R) reader structure"]
impl crate::Readable for CLK_REQ_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_req_4::W](W) writer structure"]
impl crate::Writable for CLK_REQ_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_REQ_4 to value 0"]
impl crate::Resettable for CLK_REQ_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
