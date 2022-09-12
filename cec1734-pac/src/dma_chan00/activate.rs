#[doc = "Register `ACTIVATE` reader"]
pub struct R(crate::R<ACTIVATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIVATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTIVATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTIVATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTIVATE` writer"]
pub struct W(crate::W<ACTIVATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTIVATE_SPEC>;
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
impl From<crate::W<ACTIVATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTIVATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHN` reader - Enable this channel for operation. The DMA Main Control:Activate must also be enabled for this channel to be operational. 1=Enable channel(block). Each individual channel must be enabled separately. 0=Disable channel(block)."]
pub type CHN_R = crate::BitReader<bool>;
#[doc = "Field `CHN` writer - Enable this channel for operation. The DMA Main Control:Activate must also be enabled for this channel to be operational. 1=Enable channel(block). Each individual channel must be enabled separately. 0=Disable channel(block)."]
pub type CHN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ACTIVATE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable this channel for operation. The DMA Main Control:Activate must also be enabled for this channel to be operational. 1=Enable channel(block). Each individual channel must be enabled separately. 0=Disable channel(block)."]
    #[inline(always)]
    pub fn chn(&self) -> CHN_R {
        CHN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable this channel for operation. The DMA Main Control:Activate must also be enabled for this channel to be operational. 1=Enable channel(block). Each individual channel must be enabled separately. 0=Disable channel(block)."]
    #[inline(always)]
    pub fn chn(&mut self) -> CHN_W<0> {
        CHN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable this channel for operation. The DMA Main Control: Activate must also be enabled for this channel to be operational.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [activate](index.html) module"]
pub struct ACTIVATE_SPEC;
impl crate::RegisterSpec for ACTIVATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [activate::R](R) reader structure"]
impl crate::Readable for ACTIVATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [activate::W](W) writer structure"]
impl crate::Writable for ACTIVATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACTIVATE to value 0"]
impl crate::Resettable for ACTIVATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
