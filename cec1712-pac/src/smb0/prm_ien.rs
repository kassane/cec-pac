#[doc = "Register `PRM_IEN` reader"]
pub struct R(crate::R<PRM_IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRM_IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRM_IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRM_IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRM_IEN` writer"]
pub struct W(crate::W<PRM_IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRM_IEN_SPEC>;
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
impl From<crate::W<PRM_IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRM_IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - This is the Promiscuous interrupt enable register.\n 1= Interrupt Enable. \n 0= Interrupt Disabled. \n"]
pub type ADDR_R = crate::BitReader<bool>;
#[doc = "Field `ADDR` writer - This is the Promiscuous interrupt enable register.\n 1= Interrupt Enable. \n 0= Interrupt Disabled. \n"]
pub type ADDR_W<'a, const O: u8> = crate::BitWriter<'a, u8, PRM_IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This is the Promiscuous interrupt enable register.\n 1= Interrupt Enable. \n 0= Interrupt Disabled. \n"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is the Promiscuous interrupt enable register.\n 1= Interrupt Enable. \n 0= Interrupt Disabled. \n"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the Promiscuous Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prm_ien](index.html) module"]
pub struct PRM_IEN_SPEC;
impl crate::RegisterSpec for PRM_IEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [prm_ien::R](R) reader structure"]
impl crate::Readable for PRM_IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prm_ien::W](W) writer structure"]
impl crate::Writable for PRM_IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRM_IEN to value 0"]
impl crate::Resettable for PRM_IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
