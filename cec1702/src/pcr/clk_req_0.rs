#[doc = "Register `CLK_REQ_0` reader"]
pub struct R(crate::R<CLK_REQ_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_REQ_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_REQ_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_REQ_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_REQ_0` writer"]
pub struct W(crate::W<CLK_REQ_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_REQ_0_SPEC>;
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
impl From<crate::W<CLK_REQ_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_REQ_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JTAG_STAP_CLK_REQ` reader - JTAG Clock Reuqired"]
pub type JTAG_STAP_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `JTAG_STAP_CLK_REQ` writer - JTAG Clock Reuqired"]
pub type JTAG_STAP_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_0_SPEC, bool, O>;
#[doc = "Field `EFUSE_CLK_REQ` reader - eFuse Clock Reuqired"]
pub type EFUSE_CLK_REQ_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_CLK_REQ` writer - eFuse Clock Reuqired"]
pub type EFUSE_CLK_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_REQ_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - JTAG Clock Reuqired"]
    #[inline(always)]
    pub fn jtag_stap_clk_req(&self) -> JTAG_STAP_CLK_REQ_R {
        JTAG_STAP_CLK_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - eFuse Clock Reuqired"]
    #[inline(always)]
    pub fn efuse_clk_req(&self) -> EFUSE_CLK_REQ_R {
        EFUSE_CLK_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - JTAG Clock Reuqired"]
    #[inline(always)]
    pub fn jtag_stap_clk_req(&mut self) -> JTAG_STAP_CLK_REQ_W<0> {
        JTAG_STAP_CLK_REQ_W::new(self)
    }
    #[doc = "Bit 1 - eFuse Clock Reuqired"]
    #[inline(always)]
    pub fn efuse_clk_req(&mut self) -> EFUSE_CLK_REQ_W<1> {
        EFUSE_CLK_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Required 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_req_0](index.html) module"]
pub struct CLK_REQ_0_SPEC;
impl crate::RegisterSpec for CLK_REQ_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_req_0::R](R) reader structure"]
impl crate::Readable for CLK_REQ_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_req_0::W](W) writer structure"]
impl crate::Writable for CLK_REQ_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_REQ_0 to value 0"]
impl crate::Resettable for CLK_REQ_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
