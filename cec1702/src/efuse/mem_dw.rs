#[doc = "Register `MEM_DW[%s]` reader"]
pub struct R(crate::R<MEM_DW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_DW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_DW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_DW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_DW[%s]` writer"]
pub struct W(crate::W<MEM_DW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_DW_SPEC>;
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
impl From<crate::W<MEM_DW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_DW_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "512 Bytes of EFUSE Memory (IP_MEM) Represented in 128 DW chunks:\n eFUSE memory read-back data. Access to this region depends on the operating mode: NORMAL MODE: Reading any of the bytes\n starting at this base will automatically start the controller to sequence all eFUSE signals to generate read data. Wait cycles added\n to the read cycle as appropriate. MANUAL MODE: Refer to the manual mode section for the proper procedure for accessing data in this mode.\n See REG_MAN_CTRL.MAN_EN and REG_CTRL.EXT_PRGM bits for controlling the operating mode of the block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_dw](index.html) module"]
pub struct MEM_DW_SPEC;
impl crate::RegisterSpec for MEM_DW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_dw::R](R) reader structure"]
impl crate::Readable for MEM_DW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_dw::W](W) writer structure"]
impl crate::Writable for MEM_DW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_DW[%s]
to value 0"]
impl crate::Resettable for MEM_DW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
