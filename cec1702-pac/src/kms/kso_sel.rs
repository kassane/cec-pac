#[doc = "Register `KSO_SEL` reader"]
pub struct R(crate::R<KSO_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KSO_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KSO_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KSO_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KSO_SEL` writer"]
pub struct W(crate::W<KSO_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KSO_SEL_SPEC>;
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
impl From<crate::W<KSO_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KSO_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - This field selects a KSO line (00000b = KSO\\[0\\]
etc.) for output according to the value off KSO_INVERT in this register."]
pub type SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL` writer - This field selects a KSO line (00000b = KSO\\[0\\]
etc.) for output according to the value off KSO_INVERT in this register."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KSO_SEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `ALL` reader - 0=When key scan is enabled, KSO output controlled by the KSO_SELECT field.\n 1=KSO\\[x\\]
driven high when selected."]
pub type ALL_R = crate::BitReader<bool>;
#[doc = "Field `ALL` writer - 0=When key scan is enabled, KSO output controlled by the KSO_SELECT field.\n 1=KSO\\[x\\]
driven high when selected."]
pub type ALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, KSO_SEL_SPEC, bool, O>;
#[doc = "Field `KSEN` reader - 0= Keyboard scan enabled, 1= Keyboard scan disabled. All KSO output buffers disabled."]
pub type KSEN_R = crate::BitReader<bool>;
#[doc = "Field `KSEN` writer - 0= Keyboard scan enabled, 1= Keyboard scan disabled. All KSO output buffers disabled."]
pub type KSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KSO_SEL_SPEC, bool, O>;
#[doc = "Field `INV` reader - 0= KSO\\[x\\]
driven low when selected, 1= KSO\\[x\\]
driven high when selected."]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - 0= KSO\\[x\\]
driven low when selected, 1= KSO\\[x\\]
driven high when selected."]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, KSO_SEL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - This field selects a KSO line (00000b = KSO\\[0\\]
etc.) for output according to the value off KSO_INVERT in this register."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 0=When key scan is enabled, KSO output controlled by the KSO_SELECT field.\n 1=KSO\\[x\\]
driven high when selected."]
    #[inline(always)]
    pub fn all(&self) -> ALL_R {
        ALL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0= Keyboard scan enabled, 1= Keyboard scan disabled. All KSO output buffers disabled."]
    #[inline(always)]
    pub fn ksen(&self) -> KSEN_R {
        KSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0= KSO\\[x\\]
driven low when selected, 1= KSO\\[x\\]
driven high when selected."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - This field selects a KSO line (00000b = KSO\\[0\\]
etc.) for output according to the value off KSO_INVERT in this register."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Bit 5 - 0=When key scan is enabled, KSO output controlled by the KSO_SELECT field.\n 1=KSO\\[x\\]
driven high when selected."]
    #[inline(always)]
    pub fn all(&mut self) -> ALL_W<5> {
        ALL_W::new(self)
    }
    #[doc = "Bit 6 - 0= Keyboard scan enabled, 1= Keyboard scan disabled. All KSO output buffers disabled."]
    #[inline(always)]
    pub fn ksen(&mut self) -> KSEN_W<6> {
        KSEN_W::new(self)
    }
    #[doc = "Bit 7 - 0= KSO\\[x\\]
driven low when selected, 1= KSO\\[x\\]
driven high when selected."]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<7> {
        INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KSO Select and control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kso_sel](index.html) module"]
pub struct KSO_SEL_SPEC;
impl crate::RegisterSpec for KSO_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [kso_sel::R](R) reader structure"]
impl crate::Readable for KSO_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kso_sel::W](W) writer structure"]
impl crate::Writable for KSO_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KSO_SEL to value 0"]
impl crate::Resettable for KSO_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
