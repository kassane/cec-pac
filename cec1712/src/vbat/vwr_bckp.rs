#[doc = "Register `VWR_BCKP` reader"]
pub struct R(crate::R<VWR_BCKP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VWR_BCKP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VWR_BCKP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VWR_BCKP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VWR_BCKP` writer"]
pub struct W(crate::W<VWR_BCKP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VWR_BCKP_SPEC>;
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
impl From<crate::W<VWR_BCKP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VWR_BCKP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M2S_2H_BCKUP` reader - The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register\n that corresponds to Virtual Wire Index 2h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling\n edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
pub type M2S_2H_BCKUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M2S_2H_BCKUP` writer - The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register\n that corresponds to Virtual Wire Index 2h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling\n edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
pub type M2S_2H_BCKUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VWR_BCKP_SPEC, u8, u8, 4, O>;
#[doc = "Field `M2S_42H_BCKUP` reader - The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register\n that corresponds to Virtual Wire Index 42h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling\n edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
pub type M2S_42H_BCKUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M2S_42H_BCKUP` writer - The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register\n that corresponds to Virtual Wire Index 42h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling\n edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
pub type M2S_42H_BCKUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VWR_BCKP_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register\n that corresponds to Virtual Wire Index 2h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling\n edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
    #[inline(always)]
    pub fn m2s_2h_bckup(&self) -> M2S_2H_BCKUP_R {
        M2S_2H_BCKUP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register\n that corresponds to Virtual Wire Index 42h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling\n edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
    #[inline(always)]
    pub fn m2s_42h_bckup(&self) -> M2S_42H_BCKUP_R {
        M2S_42H_BCKUP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register\n that corresponds to Virtual Wire Index 2h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling\n edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
    #[inline(always)]
    pub fn m2s_2h_bckup(&mut self) -> M2S_2H_BCKUP_W<0> {
        M2S_2H_BCKUP_W::new(self)
    }
    #[doc = "Bits 4:7 - The Boot ROM firmware will copy this field into the SRC3 to SRC0 bits of the Master-to-Slave Virtual Wire Register\n that corresponds to Virtual Wire Index 42h on a RESET_SYS. If software always saves the state of the Index 2h SRC bits on the falling\n edge of the SUSWARN# virtual wire, the state of the four SRC bits will be synchronized to the state of the four bits in the core logic."]
    #[inline(always)]
    pub fn m2s_42h_bckup(&mut self) -> M2S_42H_BCKUP_W<4> {
        M2S_42H_BCKUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VWIRE_BACKUP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vwr_bckp](index.html) module"]
pub struct VWR_BCKP_SPEC;
impl crate::RegisterSpec for VWR_BCKP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vwr_bckp::R](R) reader structure"]
impl crate::Readable for VWR_BCKP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vwr_bckp::W](W) writer structure"]
impl crate::Writable for VWR_BCKP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VWR_BCKP to value 0"]
impl crate::Resettable for VWR_BCKP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
