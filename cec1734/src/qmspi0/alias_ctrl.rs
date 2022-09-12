#[doc = "Register `ALIAS_CTRL` writer"]
pub struct W(crate::W<ALIAS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALIAS_CTRL_SPEC>;
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
impl From<crate::W<ALIAS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALIAS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALS_EXEC_STRT` writer - Alias for the QMSPI Execution:Start field."]
pub type ALS_EXEC_STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALIAS_CTRL_SPEC, bool, O>;
#[doc = "Field `CLS_ALTMODE_EN` writer - Enable the CS1 Clock Divide to be active if CS1 is the interface in use."]
pub type CLS_ALTMODE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALIAS_CTRL_SPEC, bool, O>;
#[doc = "Field `ALS_WR_TXBUF` writer - Alias that triggers a write to the Tx Buffer of 1 Byte using data from Alias Tx Buffer Data in this register."]
pub type ALS_WR_TXBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALIAS_CTRL_SPEC, bool, O>;
#[doc = "Field `ALS_WRDBUF_XFRLEN` writer - Alias that triggers a write to the Description Buffer pointed to by Alias Description Buffer Select in this register to modify the Transfer Length field with the value of Alias Description Buffer Transfer Length in this register."]
pub type ALS_WRDBUF_XFRLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALIAS_CTRL_SPEC, bool, O>;
#[doc = "Field `ALS_MOD_CS` writer - Alias for the QMSPI Mode:Chip Select field."]
pub type ALS_MOD_CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALIAS_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ALS_LDMA_INCR_ADD` writer - Alias that overrides the value in all QMSPI Local DMA * \\[Tx/Rx\\]
Control:Increment Address Enable fields w/ this value."]
pub type ALS_LDMA_INCR_ADD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALIAS_CTRL_SPEC, bool, O>;
#[doc = "Field `ALS_CTRL_DBUF_PTR` writer - Alias for the QMSPI Control:Description Buffer Pointer field."]
pub type ALS_CTRL_DBUF_PTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALIAS_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `ALS_DBUF_SEL` writer - Which Description Buffer will be modified by a write if Alias Write Description Buffer Transfer Length is set."]
pub type ALS_DBUF_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALIAS_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `ALS_DBUF_XFR_LEN` writer - The value of the data written to the Description Buffers Transfer Length field, if the write is enabled."]
pub type ALS_DBUF_XFR_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALIAS_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `ALS_TXDBUF_DATA` writer - The Byte of data written into the Tx Buffer if the write is enabled."]
pub type ALS_TXDBUF_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALIAS_CTRL_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bit 0 - Alias for the QMSPI Execution:Start field."]
    #[inline(always)]
    pub fn als_exec_strt(&mut self) -> ALS_EXEC_STRT_W<0> {
        ALS_EXEC_STRT_W::new(self)
    }
    #[doc = "Bit 1 - Enable the CS1 Clock Divide to be active if CS1 is the interface in use."]
    #[inline(always)]
    pub fn cls_altmode_en(&mut self) -> CLS_ALTMODE_EN_W<1> {
        CLS_ALTMODE_EN_W::new(self)
    }
    #[doc = "Bit 2 - Alias that triggers a write to the Tx Buffer of 1 Byte using data from Alias Tx Buffer Data in this register."]
    #[inline(always)]
    pub fn als_wr_txbuf(&mut self) -> ALS_WR_TXBUF_W<2> {
        ALS_WR_TXBUF_W::new(self)
    }
    #[doc = "Bit 3 - Alias that triggers a write to the Description Buffer pointed to by Alias Description Buffer Select in this register to modify the Transfer Length field with the value of Alias Description Buffer Transfer Length in this register."]
    #[inline(always)]
    pub fn als_wrdbuf_xfrlen(&mut self) -> ALS_WRDBUF_XFRLEN_W<3> {
        ALS_WRDBUF_XFRLEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Alias for the QMSPI Mode:Chip Select field."]
    #[inline(always)]
    pub fn als_mod_cs(&mut self) -> ALS_MOD_CS_W<4> {
        ALS_MOD_CS_W::new(self)
    }
    #[doc = "Bit 6 - Alias that overrides the value in all QMSPI Local DMA * \\[Tx/Rx\\]
Control:Increment Address Enable fields w/ this value."]
    #[inline(always)]
    pub fn als_ldma_incr_add(&mut self) -> ALS_LDMA_INCR_ADD_W<6> {
        ALS_LDMA_INCR_ADD_W::new(self)
    }
    #[doc = "Bits 8:11 - Alias for the QMSPI Control:Description Buffer Pointer field."]
    #[inline(always)]
    pub fn als_ctrl_dbuf_ptr(&mut self) -> ALS_CTRL_DBUF_PTR_W<8> {
        ALS_CTRL_DBUF_PTR_W::new(self)
    }
    #[doc = "Bits 12:15 - Which Description Buffer will be modified by a write if Alias Write Description Buffer Transfer Length is set."]
    #[inline(always)]
    pub fn als_dbuf_sel(&mut self) -> ALS_DBUF_SEL_W<12> {
        ALS_DBUF_SEL_W::new(self)
    }
    #[doc = "Bits 16:23 - The value of the data written to the Description Buffers Transfer Length field, if the write is enabled."]
    #[inline(always)]
    pub fn als_dbuf_xfr_len(&mut self) -> ALS_DBUF_XFR_LEN_W<16> {
        ALS_DBUF_XFR_LEN_W::new(self)
    }
    #[doc = "Bits 24:31 - The Byte of data written into the Tx Buffer if the write is enabled."]
    #[inline(always)]
    pub fn als_txdbuf_data(&mut self) -> ALS_TXDBUF_DATA_W<24> {
        ALS_TXDBUF_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QMSPI Alias Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alias_ctrl](index.html) module"]
pub struct ALIAS_CTRL_SPEC;
impl crate::RegisterSpec for ALIAS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [alias_ctrl::W](W) writer structure"]
impl crate::Writable for ALIAS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALIAS_CTRL to value 0"]
impl crate::Resettable for ALIAS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
