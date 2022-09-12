#[doc = "Register `SLOW_CLK_CTRL` reader"]
pub struct R(crate::R<SLOW_CLK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLOW_CLK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLOW_CLK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLOW_CLK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLOW_CLK_CTRL` writer"]
pub struct W(crate::W<SLOW_CLK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLOW_CLK_CTRL_SPEC>;
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
impl From<crate::W<SLOW_CLK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLOW_CLK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - SLOW_CLOCK_DIVIDE. n=Divide by n; 0=Clock off"]
pub type DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIV` writer - SLOW_CLOCK_DIVIDE. n=Divide by n; 0=Clock off"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLOW_CLK_CTRL_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - SLOW_CLOCK_DIVIDE. n=Divide by n; 0=Clock off"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - SLOW_CLOCK_DIVIDE. n=Divide by n; 0=Clock off"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the EC_CLK clock domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slow_clk_ctrl](index.html) module"]
pub struct SLOW_CLK_CTRL_SPEC;
impl crate::RegisterSpec for SLOW_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slow_clk_ctrl::R](R) reader structure"]
impl crate::Readable for SLOW_CLK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slow_clk_ctrl::W](W) writer structure"]
impl crate::Writable for SLOW_CLK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLOW_CLK_CTRL to value 0x01e0"]
impl crate::Resettable for SLOW_CLK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01e0
    }
}
