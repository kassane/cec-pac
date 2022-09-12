#[doc = "Register `CSTM` reader"]
pub struct R(crate::R<CSTM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSTM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSTM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSTM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSTM` writer"]
pub struct W(crate::W<CSTM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSTM_SPEC>;
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
impl From<crate::W<CSTM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSTM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY_CS_ON_CLK_STRT` reader - "]
pub type DLY_CS_ON_CLK_STRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLY_CS_ON_CLK_STRT` writer - "]
pub type DLY_CS_ON_CLK_STRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSTM_SPEC, u8, u8, 4, O>;
#[doc = "Field `DLY_CLK_STOP_CS_OFF` reader - This selects the number of system clock cycles between the last clock edge and the deassertion of CS.\n"]
pub type DLY_CLK_STOP_CS_OFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLY_CLK_STOP_CS_OFF` writer - This selects the number of system clock cycles between the last clock edge and the deassertion of CS.\n"]
pub type DLY_CLK_STOP_CS_OFF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSTM_SPEC, u8, u8, 4, O>;
#[doc = "Field `DLY_LAST_DAT_HLD` reader - This selects the number of system clock cycles between CS deassertion to the data ports for WP and HOLD \n switching from input to output. This is only used if the WP/HOLD functions are in use and only on IO2/WP \n and IO3/HOLD pins."]
pub type DLY_LAST_DAT_HLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLY_LAST_DAT_HLD` writer - This selects the number of system clock cycles between CS deassertion to the data ports for WP and HOLD \n switching from input to output. This is only used if the WP/HOLD functions are in use and only on IO2/WP \n and IO3/HOLD pins."]
pub type DLY_LAST_DAT_HLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSTM_SPEC, u8, u8, 4, O>;
#[doc = "Field `DLY_OFF_TO_ON` reader - This selects the number of system clock cycles between CS deassertion to CS assertion. This is the minimum \n pulse width of CS deassertion."]
pub type DLY_OFF_TO_ON_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DLY_OFF_TO_ON` writer - This selects the number of system clock cycles between CS deassertion to CS assertion. This is the minimum \n pulse width of CS deassertion."]
pub type DLY_OFF_TO_ON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSTM_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dly_cs_on_clk_strt(&self) -> DLY_CS_ON_CLK_STRT_R {
        DLY_CS_ON_CLK_STRT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This selects the number of system clock cycles between the last clock edge and the deassertion of CS.\n"]
    #[inline(always)]
    pub fn dly_clk_stop_cs_off(&self) -> DLY_CLK_STOP_CS_OFF_R {
        DLY_CLK_STOP_CS_OFF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - This selects the number of system clock cycles between CS deassertion to the data ports for WP and HOLD \n switching from input to output. This is only used if the WP/HOLD functions are in use and only on IO2/WP \n and IO3/HOLD pins."]
    #[inline(always)]
    pub fn dly_last_dat_hld(&self) -> DLY_LAST_DAT_HLD_R {
        DLY_LAST_DAT_HLD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 23:31 - This selects the number of system clock cycles between CS deassertion to CS assertion. This is the minimum \n pulse width of CS deassertion."]
    #[inline(always)]
    pub fn dly_off_to_on(&self) -> DLY_OFF_TO_ON_R {
        DLY_OFF_TO_ON_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dly_cs_on_clk_strt(&mut self) -> DLY_CS_ON_CLK_STRT_W<0> {
        DLY_CS_ON_CLK_STRT_W::new(self)
    }
    #[doc = "Bits 8:11 - This selects the number of system clock cycles between the last clock edge and the deassertion of CS.\n"]
    #[inline(always)]
    pub fn dly_clk_stop_cs_off(&mut self) -> DLY_CLK_STOP_CS_OFF_W<8> {
        DLY_CLK_STOP_CS_OFF_W::new(self)
    }
    #[doc = "Bits 16:19 - This selects the number of system clock cycles between CS deassertion to the data ports for WP and HOLD \n switching from input to output. This is only used if the WP/HOLD functions are in use and only on IO2/WP \n and IO3/HOLD pins."]
    #[inline(always)]
    pub fn dly_last_dat_hld(&mut self) -> DLY_LAST_DAT_HLD_W<16> {
        DLY_LAST_DAT_HLD_W::new(self)
    }
    #[doc = "Bits 23:31 - This selects the number of system clock cycles between CS deassertion to CS assertion. This is the minimum \n pulse width of CS deassertion."]
    #[inline(always)]
    pub fn dly_off_to_on(&mut self) -> DLY_OFF_TO_ON_W<23> {
        DLY_OFF_TO_ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Chip Select Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cstm](index.html) module"]
pub struct CSTM_SPEC;
impl crate::RegisterSpec for CSTM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cstm::R](R) reader structure"]
impl crate::Readable for CSTM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cstm::W](W) writer structure"]
impl crate::Writable for CSTM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSTM to value 0x0606_0406"]
impl crate::Resettable for CSTM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0606_0406
    }
}
