#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable. 1=Clock enabled, 0=Clock is disabled (Default)"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable. 1=Clock enabled, 0=Clock is disabled (Default)"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
#[doc = "Field `EDGE_SEL` reader - 1= Data is shifted out on the falling edge of the debug clock, 0= Data is shifted out on the rising edge of the debug clock (Default)"]
pub type EDGE_SEL_R = crate::BitReader<bool>;
#[doc = "Field `EDGE_SEL` writer - 1= Data is shifted out on the falling edge of the debug clock, 0= Data is shifted out on the rising edge of the debug clock (Default)"]
pub type EDGE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
#[doc = "Field `DIVSEL` reader - Clock Divider Select."]
pub type DIVSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVSEL` writer - Clock Divider Select."]
pub type DIVSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_DLY` reader - Inter-packet Delay. The delay is in terms of TFDP Debug output clocks."]
pub type IP_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_DLY` writer - Inter-packet Delay. The delay is in terms of TFDP Debug output clocks."]
pub type IP_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Enable. 1=Clock enabled, 0=Clock is disabled (Default)"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1= Data is shifted out on the falling edge of the debug clock, 0= Data is shifted out on the rising edge of the debug clock (Default)"]
    #[inline(always)]
    pub fn edge_sel(&self) -> EDGE_SEL_R {
        EDGE_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Clock Divider Select."]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Inter-packet Delay. The delay is in terms of TFDP Debug output clocks."]
    #[inline(always)]
    pub fn ip_dly(&self) -> IP_DLY_R {
        IP_DLY_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable. 1=Clock enabled, 0=Clock is disabled (Default)"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - 1= Data is shifted out on the falling edge of the debug clock, 0= Data is shifted out on the rising edge of the debug clock (Default)"]
    #[inline(always)]
    pub fn edge_sel(&mut self) -> EDGE_SEL_W<1> {
        EDGE_SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - Clock Divider Select."]
    #[inline(always)]
    pub fn divsel(&mut self) -> DIVSEL_W<2> {
        DIVSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Inter-packet Delay. The delay is in terms of TFDP Debug output clocks."]
    #[inline(always)]
    pub fn ip_dly(&mut self) -> IP_DLY_W<4> {
        IP_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
