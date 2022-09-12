#[doc = "Register `HLDOFF_CNT` reader"]
pub struct R(crate::R<HLDOFF_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HLDOFF_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HLDOFF_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HLDOFF_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HLDOFF_CNT` writer"]
pub struct W(crate::W<HLDOFF_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HLDOFF_CNT_SPEC>;
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
impl From<crate::W<HLDOFF_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HLDOFF_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME` reader - These bits determine the period of time the VCI_OUT logic is inhibited from re-asserting VCI_OUT after a SYS_SHDN# event.\n FFh-01h=The Power On Inhibit Holdoff Time is set to a period between 125ms and 31.875 seconds.\n 0=The Power On Inhibit function is disabled."]
pub type TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIME` writer - These bits determine the period of time the VCI_OUT logic is inhibited from re-asserting VCI_OUT after a SYS_SHDN# event.\n FFh-01h=The Power On Inhibit Holdoff Time is set to a period between 125ms and 31.875 seconds.\n 0=The Power On Inhibit function is disabled."]
pub type TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HLDOFF_CNT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - These bits determine the period of time the VCI_OUT logic is inhibited from re-asserting VCI_OUT after a SYS_SHDN# event.\n FFh-01h=The Power On Inhibit Holdoff Time is set to a period between 125ms and 31.875 seconds.\n 0=The Power On Inhibit function is disabled."]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits determine the period of time the VCI_OUT logic is inhibited from re-asserting VCI_OUT after a SYS_SHDN# event.\n FFh-01h=The Power On Inhibit Holdoff Time is set to a period between 125ms and 31.875 seconds.\n 0=The Power On Inhibit function is disabled."]
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W<0> {
        TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Holdoff Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hldoff_cnt](index.html) module"]
pub struct HLDOFF_CNT_SPEC;
impl crate::RegisterSpec for HLDOFF_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hldoff_cnt::R](R) reader structure"]
impl crate::Readable for HLDOFF_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hldoff_cnt::W](W) writer structure"]
impl crate::Writable for HLDOFF_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HLDOFF_CNT to value 0"]
impl crate::Resettable for HLDOFF_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
