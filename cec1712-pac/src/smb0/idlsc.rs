#[doc = "Register `IDLSC` reader"]
pub struct R(crate::R<IDLSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDLSC` writer"]
pub struct W(crate::W<IDLSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDLSC_SPEC>;
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
impl From<crate::W<IDLSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDLSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAIR_BUS_IDL_MIN` reader - This field defines the number of ticks of the baud clock required to satisfy the fairness protocol. The default value for this field sets the idle window to 31us, which is the appropriate value for a 100 KHz bus"]
pub type FAIR_BUS_IDL_MIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FAIR_BUS_IDL_MIN` writer - This field defines the number of ticks of the baud clock required to satisfy the fairness protocol. The default value for this field sets the idle window to 31us, which is the appropriate value for a 100 KHz bus"]
pub type FAIR_BUS_IDL_MIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IDLSC_SPEC, u16, u16, 12, O>;
#[doc = "Field `FAIR_IDL_DLY` reader - This field defines the number of ticks of the baud clock required to program the delay. The default value for this field sets the delay period to 32us, which is the appropriate value for a 100 KHz bus"]
pub type FAIR_IDL_DLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FAIR_IDL_DLY` writer - This field defines the number of ticks of the baud clock required to program the delay. The default value for this field sets the delay period to 32us, which is the appropriate value for a 100 KHz bus"]
pub type FAIR_IDL_DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDLSC_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - This field defines the number of ticks of the baud clock required to satisfy the fairness protocol. The default value for this field sets the idle window to 31us, which is the appropriate value for a 100 KHz bus"]
    #[inline(always)]
    pub fn fair_bus_idl_min(&self) -> FAIR_BUS_IDL_MIN_R {
        FAIR_BUS_IDL_MIN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - This field defines the number of ticks of the baud clock required to program the delay. The default value for this field sets the delay period to 32us, which is the appropriate value for a 100 KHz bus"]
    #[inline(always)]
    pub fn fair_idl_dly(&self) -> FAIR_IDL_DLY_R {
        FAIR_IDL_DLY_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This field defines the number of ticks of the baud clock required to satisfy the fairness protocol. The default value for this field sets the idle window to 31us, which is the appropriate value for a 100 KHz bus"]
    #[inline(always)]
    pub fn fair_bus_idl_min(&mut self) -> FAIR_BUS_IDL_MIN_W<0> {
        FAIR_BUS_IDL_MIN_W::new(self)
    }
    #[doc = "Bits 16:27 - This field defines the number of ticks of the baud clock required to program the delay. The default value for this field sets the delay period to 32us, which is the appropriate value for a 100 KHz bus"]
    #[inline(always)]
    pub fn fair_idl_dly(&mut self) -> FAIR_IDL_DLY_W<16> {
        FAIR_IDL_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Idle Scaling Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idlsc](index.html) module"]
pub struct IDLSC_SPEC;
impl crate::RegisterSpec for IDLSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idlsc::R](R) reader structure"]
impl crate::Readable for IDLSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idlsc::W](W) writer structure"]
impl crate::Writable for IDLSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDLSC to value 0x01fc_01ed"]
impl crate::Resettable for IDLSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01fc_01ed
    }
}
