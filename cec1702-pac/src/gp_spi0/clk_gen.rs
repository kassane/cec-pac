#[doc = "Register `CLK_GEN` reader"]
pub struct R(crate::R<CLK_GEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_GEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_GEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_GEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_GEN` writer"]
pub struct W(crate::W<CLK_GEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_GEN_SPEC>;
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
impl From<crate::W<CLK_GEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_GEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRLD` reader - SPI Clock Generator Preload Value"]
pub type PRLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRLD` writer - SPI Clock Generator Preload Value"]
pub type PRLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_GEN_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - SPI Clock Generator Preload Value"]
    #[inline(always)]
    pub fn prld(&self) -> PRLD_R {
        PRLD_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - SPI Clock Generator Preload Value"]
    #[inline(always)]
    pub fn prld(&mut self) -> PRLD_W<0> {
        PRLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "5:0\\]
PRELOAD SPI Clock Generator Preload value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gen](index.html) module"]
pub struct CLK_GEN_SPEC;
impl crate::RegisterSpec for CLK_GEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_gen::R](R) reader structure"]
impl crate::Readable for CLK_GEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_gen::W](W) writer structure"]
impl crate::Writable for CLK_GEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_GEN to value 0x02"]
impl crate::Resettable for CLK_GEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
