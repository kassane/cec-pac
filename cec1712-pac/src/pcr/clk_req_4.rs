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
#[doc = "Field `RTOS_CLK_REQ` reader - RTOS Clock Required (RTOS_CLK_REQ)"]
pub type RTOS_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `RTOS_CLK_REQ` writer - RTOS Clock Required (RTOS_CLK_REQ)"]
pub type RTOS_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
#[doc = "Field `QMSPI_CLK_REQ` reader - Quad Master SPI Clock Required"]
pub type QMSPI_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `QMSPI_CLK_REQ` writer - Quad Master SPI Clock Required"]
pub type QMSPI_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - RTOS Clock Required (RTOS_CLK_REQ)"]
    #[inline(always)]
    pub fn rtos_clk_req(&self) -> RTOS_CLK_REQ_R {
        RTOS_CLK_REQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Quad Master SPI Clock Required"]
    #[inline(always)]
    pub fn qmspi_clk_req(&self) -> QMSPI_CLK_REQ_R {
        QMSPI_CLK_REQ_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - RTOS Clock Required (RTOS_CLK_REQ)"]
    #[inline(always)]
    pub fn rtos_clk_req(&mut self) -> RTOS_CLK_REQ_W<6> {
        RTOS_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 8 - Quad Master SPI Clock Required"]
    #[inline(always)]
    pub fn qmspi_clk_req(&mut self) -> QMSPI_CLK_REQ_W<8> {
        QMSPI_CLK_REQ_W::new(self)
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
