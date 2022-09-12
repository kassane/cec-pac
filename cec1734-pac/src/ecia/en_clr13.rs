#[doc = "Register `EN_CLR13` reader"]
pub struct R(crate::R<EN_CLR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EN_CLR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EN_CLR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EN_CLR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EN_CLR13` writer"]
pub struct W(crate::W<EN_CLR13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_CLR13_SPEC>;
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
impl From<crate::W<EN_CLR13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_CLR13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CSMB0` reader - I2CSMB0"]
pub type I2CSMB0_R = crate::BitReader<bool>;
#[doc = "Field `I2CSMB0` writer - I2CSMB0"]
pub type I2CSMB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR13_SPEC, bool, O>;
#[doc = "Field `I2CSMB1` reader - I2CSMB1"]
pub type I2CSMB1_R = crate::BitReader<bool>;
#[doc = "Field `I2CSMB1` writer - I2CSMB1"]
pub type I2CSMB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR13_SPEC, bool, O>;
#[doc = "Field `I2CSMB2` reader - I2CSMB2"]
pub type I2CSMB2_R = crate::BitReader<bool>;
#[doc = "Field `I2CSMB2` writer - I2CSMB2"]
pub type I2CSMB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR13_SPEC, bool, O>;
#[doc = "Field `I2CSMB3` reader - I2CSMB3"]
pub type I2CSMB3_R = crate::BitReader<bool>;
#[doc = "Field `I2CSMB3` writer - I2CSMB3"]
pub type I2CSMB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR13_SPEC, bool, O>;
#[doc = "Field `I2CSMB4` reader - I2CSMB4"]
pub type I2CSMB4_R = crate::BitReader<bool>;
#[doc = "Field `I2CSMB4` writer - I2CSMB4"]
pub type I2CSMB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_CLR13_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I2CSMB0"]
    #[inline(always)]
    pub fn i2csmb0(&self) -> I2CSMB0_R {
        I2CSMB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2CSMB1"]
    #[inline(always)]
    pub fn i2csmb1(&self) -> I2CSMB1_R {
        I2CSMB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2CSMB2"]
    #[inline(always)]
    pub fn i2csmb2(&self) -> I2CSMB2_R {
        I2CSMB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2CSMB3"]
    #[inline(always)]
    pub fn i2csmb3(&self) -> I2CSMB3_R {
        I2CSMB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2CSMB4"]
    #[inline(always)]
    pub fn i2csmb4(&self) -> I2CSMB4_R {
        I2CSMB4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2CSMB0"]
    #[inline(always)]
    pub fn i2csmb0(&mut self) -> I2CSMB0_W<0> {
        I2CSMB0_W::new(self)
    }
    #[doc = "Bit 1 - I2CSMB1"]
    #[inline(always)]
    pub fn i2csmb1(&mut self) -> I2CSMB1_W<1> {
        I2CSMB1_W::new(self)
    }
    #[doc = "Bit 2 - I2CSMB2"]
    #[inline(always)]
    pub fn i2csmb2(&mut self) -> I2CSMB2_W<2> {
        I2CSMB2_W::new(self)
    }
    #[doc = "Bit 3 - I2CSMB3"]
    #[inline(always)]
    pub fn i2csmb3(&mut self) -> I2CSMB3_W<3> {
        I2CSMB3_W::new(self)
    }
    #[doc = "Bit 4 - I2CSMB4"]
    #[inline(always)]
    pub fn i2csmb4(&mut self) -> I2CSMB4_W<4> {
        I2CSMB4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ13 ENABLE CLEAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en_clr13](index.html) module"]
pub struct EN_CLR13_SPEC;
impl crate::RegisterSpec for EN_CLR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [en_clr13::R](R) reader structure"]
impl crate::Readable for EN_CLR13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [en_clr13::W](W) writer structure"]
impl crate::Writable for EN_CLR13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EN_CLR13 to value 0"]
impl crate::Resettable for EN_CLR13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
