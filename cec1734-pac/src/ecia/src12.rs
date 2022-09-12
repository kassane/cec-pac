#[doc = "Register `SRC12` reader"]
pub struct R(crate::R<SRC12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC12` writer"]
pub struct W(crate::W<SRC12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC12_SPEC>;
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
impl From<crate::W<SRC12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO200` reader - GPIO 200"]
pub type GPIO200_R = crate::BitReader<bool>;
#[doc = "Field `GPIO200` writer - GPIO 200"]
pub type GPIO200_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC12_SPEC, bool, O>;
#[doc = "Field `GPIO201` reader - GPIO 201"]
pub type GPIO201_R = crate::BitReader<bool>;
#[doc = "Field `GPIO201` writer - GPIO 201"]
pub type GPIO201_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC12_SPEC, bool, O>;
#[doc = "Field `GPIO202` reader - GPIO 202"]
pub type GPIO202_R = crate::BitReader<bool>;
#[doc = "Field `GPIO202` writer - GPIO 202"]
pub type GPIO202_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC12_SPEC, bool, O>;
#[doc = "Field `GPIO203` reader - GPIO 203"]
pub type GPIO203_R = crate::BitReader<bool>;
#[doc = "Field `GPIO203` writer - GPIO 203"]
pub type GPIO203_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC12_SPEC, bool, O>;
#[doc = "Field `GPIO204` reader - GPIO 204"]
pub type GPIO204_R = crate::BitReader<bool>;
#[doc = "Field `GPIO204` writer - GPIO 204"]
pub type GPIO204_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC12_SPEC, bool, O>;
#[doc = "Field `GPIO223` reader - GPIO 223"]
pub type GPIO223_R = crate::BitReader<bool>;
#[doc = "Field `GPIO223` writer - GPIO 223"]
pub type GPIO223_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC12_SPEC, bool, O>;
#[doc = "Field `GPIO224` reader - GPIO 224"]
pub type GPIO224_R = crate::BitReader<bool>;
#[doc = "Field `GPIO224` writer - GPIO 224"]
pub type GPIO224_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC12_SPEC, bool, O>;
#[doc = "Field `GPIO227` reader - GPIO 227"]
pub type GPIO227_R = crate::BitReader<bool>;
#[doc = "Field `GPIO227` writer - GPIO 227"]
pub type GPIO227_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC12_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIO 200"]
    #[inline(always)]
    pub fn gpio200(&self) -> GPIO200_R {
        GPIO200_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO 201"]
    #[inline(always)]
    pub fn gpio201(&self) -> GPIO201_R {
        GPIO201_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO 202"]
    #[inline(always)]
    pub fn gpio202(&self) -> GPIO202_R {
        GPIO202_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO 203"]
    #[inline(always)]
    pub fn gpio203(&self) -> GPIO203_R {
        GPIO203_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO 204"]
    #[inline(always)]
    pub fn gpio204(&self) -> GPIO204_R {
        GPIO204_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO 223"]
    #[inline(always)]
    pub fn gpio223(&self) -> GPIO223_R {
        GPIO223_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO 224"]
    #[inline(always)]
    pub fn gpio224(&self) -> GPIO224_R {
        GPIO224_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO 227"]
    #[inline(always)]
    pub fn gpio227(&self) -> GPIO227_R {
        GPIO227_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO 200"]
    #[inline(always)]
    pub fn gpio200(&mut self) -> GPIO200_W<0> {
        GPIO200_W::new(self)
    }
    #[doc = "Bit 1 - GPIO 201"]
    #[inline(always)]
    pub fn gpio201(&mut self) -> GPIO201_W<1> {
        GPIO201_W::new(self)
    }
    #[doc = "Bit 2 - GPIO 202"]
    #[inline(always)]
    pub fn gpio202(&mut self) -> GPIO202_W<2> {
        GPIO202_W::new(self)
    }
    #[doc = "Bit 3 - GPIO 203"]
    #[inline(always)]
    pub fn gpio203(&mut self) -> GPIO203_W<3> {
        GPIO203_W::new(self)
    }
    #[doc = "Bit 4 - GPIO 204"]
    #[inline(always)]
    pub fn gpio204(&mut self) -> GPIO204_W<4> {
        GPIO204_W::new(self)
    }
    #[doc = "Bit 19 - GPIO 223"]
    #[inline(always)]
    pub fn gpio223(&mut self) -> GPIO223_W<19> {
        GPIO223_W::new(self)
    }
    #[doc = "Bit 20 - GPIO 224"]
    #[inline(always)]
    pub fn gpio224(&mut self) -> GPIO224_W<20> {
        GPIO224_W::new(self)
    }
    #[doc = "Bit 23 - GPIO 227"]
    #[inline(always)]
    pub fn gpio227(&mut self) -> GPIO227_W<23> {
        GPIO227_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ12 SOURCE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src12](index.html) module"]
pub struct SRC12_SPEC;
impl crate::RegisterSpec for SRC12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src12::R](R) reader structure"]
impl crate::Readable for SRC12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src12::W](W) writer structure"]
impl crate::Writable for SRC12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC12 to value 0"]
impl crate::Resettable for SRC12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
