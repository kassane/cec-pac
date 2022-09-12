#[doc = "Register `BB_CTRL` reader"]
pub struct R(crate::R<BB_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BB_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BB_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BB_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BB_CTRL` writer"]
pub struct W(crate::W<BB_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BB_CTRL_SPEC>;
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
impl From<crate::W<BB_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BB_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BBEN` reader - Bit-Bang Mode Enable. 0 - Bit Bang Mode Disabled. 1 - Bit Bang Mode Enabled"]
pub type BBEN_R = crate::BitReader<bool>;
#[doc = "Field `BBEN` writer - Bit-Bang Mode Enable. 0 - Bit Bang Mode Disabled. 1 - Bit Bang Mode Enabled"]
pub type BBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BB_CTRL_SPEC, bool, O>;
#[doc = "Field `CLDIR` reader - Bit-Bang Clock Direction. The CLDIR bit controls the direction of SCLK. 0 - Input, 1 - Output"]
pub type CLDIR_R = crate::BitReader<bool>;
#[doc = "Field `CLDIR` writer - Bit-Bang Clock Direction. The CLDIR bit controls the direction of SCLK. 0 - Input, 1 - Output"]
pub type CLDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BB_CTRL_SPEC, bool, O>;
#[doc = "Field `DADIR` reader - Bit-Bang Data Direction. The DADIR bit controls the direction of SDAT. 0 - Input. 1 - Output"]
pub type DADIR_R = crate::BitReader<bool>;
#[doc = "Field `DADIR` writer - Bit-Bang Data Direction. The DADIR bit controls the direction of SDAT. 0 - Input. 1 - Output"]
pub type DADIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BB_CTRL_SPEC, bool, O>;
#[doc = "Field `BBCLK` reader - Bit-Bang Clock. The BBCLK bit controls the state of SCLK when BBEN = and CLDIR = '1'"]
pub type BBCLK_R = crate::BitReader<bool>;
#[doc = "Field `BBCLK` writer - Bit-Bang Clock. The BBCLK bit controls the state of SCLK when BBEN = and CLDIR = '1'"]
pub type BBCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, BB_CTRL_SPEC, bool, O>;
#[doc = "Field `BBDAT` reader - Bit-Bang Data. The BBDAT bit controls the state of SDAT when BBEN = and DADIR = '1'"]
pub type BBDAT_R = crate::BitReader<bool>;
#[doc = "Field `BBDAT` writer - Bit-Bang Data. The BBDAT bit controls the state of SDAT when BBEN = and DADIR = '1'"]
pub type BBDAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BB_CTRL_SPEC, bool, O>;
#[doc = "Field `BBCLKI` reader - Bit-Bang Clock In. The BBCLKI bit always returns the state of SCLK."]
pub type BBCLKI_R = crate::BitReader<bool>;
#[doc = "Field `BBCLKI` writer - Bit-Bang Clock In. The BBCLKI bit always returns the state of SCLK."]
pub type BBCLKI_W<'a, const O: u8> = crate::BitWriter<'a, u32, BB_CTRL_SPEC, bool, O>;
#[doc = "Field `BBDATI` reader - Bit-Bang Data In. The BBDATI bit always returns the state of SDAT"]
pub type BBDATI_R = crate::BitReader<bool>;
#[doc = "Field `BBDATI` writer - Bit-Bang Data In. The BBDATI bit always returns the state of SDAT"]
pub type BBDATI_W<'a, const O: u8> = crate::BitWriter<'a, u32, BB_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Bit-Bang Mode Enable. 0 - Bit Bang Mode Disabled. 1 - Bit Bang Mode Enabled"]
    #[inline(always)]
    pub fn bben(&self) -> BBEN_R {
        BBEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit-Bang Clock Direction. The CLDIR bit controls the direction of SCLK. 0 - Input, 1 - Output"]
    #[inline(always)]
    pub fn cldir(&self) -> CLDIR_R {
        CLDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit-Bang Data Direction. The DADIR bit controls the direction of SDAT. 0 - Input. 1 - Output"]
    #[inline(always)]
    pub fn dadir(&self) -> DADIR_R {
        DADIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit-Bang Clock. The BBCLK bit controls the state of SCLK when BBEN = and CLDIR = '1'"]
    #[inline(always)]
    pub fn bbclk(&self) -> BBCLK_R {
        BBCLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit-Bang Data. The BBDAT bit controls the state of SDAT when BBEN = and DADIR = '1'"]
    #[inline(always)]
    pub fn bbdat(&self) -> BBDAT_R {
        BBDAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit-Bang Clock In. The BBCLKI bit always returns the state of SCLK."]
    #[inline(always)]
    pub fn bbclki(&self) -> BBCLKI_R {
        BBCLKI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit-Bang Data In. The BBDATI bit always returns the state of SDAT"]
    #[inline(always)]
    pub fn bbdati(&self) -> BBDATI_R {
        BBDATI_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit-Bang Mode Enable. 0 - Bit Bang Mode Disabled. 1 - Bit Bang Mode Enabled"]
    #[inline(always)]
    pub fn bben(&mut self) -> BBEN_W<0> {
        BBEN_W::new(self)
    }
    #[doc = "Bit 1 - Bit-Bang Clock Direction. The CLDIR bit controls the direction of SCLK. 0 - Input, 1 - Output"]
    #[inline(always)]
    pub fn cldir(&mut self) -> CLDIR_W<1> {
        CLDIR_W::new(self)
    }
    #[doc = "Bit 2 - Bit-Bang Data Direction. The DADIR bit controls the direction of SDAT. 0 - Input. 1 - Output"]
    #[inline(always)]
    pub fn dadir(&mut self) -> DADIR_W<2> {
        DADIR_W::new(self)
    }
    #[doc = "Bit 3 - Bit-Bang Clock. The BBCLK bit controls the state of SCLK when BBEN = and CLDIR = '1'"]
    #[inline(always)]
    pub fn bbclk(&mut self) -> BBCLK_W<3> {
        BBCLK_W::new(self)
    }
    #[doc = "Bit 4 - Bit-Bang Data. The BBDAT bit controls the state of SDAT when BBEN = and DADIR = '1'"]
    #[inline(always)]
    pub fn bbdat(&mut self) -> BBDAT_W<4> {
        BBDAT_W::new(self)
    }
    #[doc = "Bit 5 - Bit-Bang Clock In. The BBCLKI bit always returns the state of SCLK."]
    #[inline(always)]
    pub fn bbclki(&mut self) -> BBCLKI_W<5> {
        BBCLKI_W::new(self)
    }
    #[doc = "Bit 6 - Bit-Bang Data In. The BBDATI bit always returns the state of SDAT"]
    #[inline(always)]
    pub fn bbdati(&mut self) -> BBDATI_W<6> {
        BBDATI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit-Bang Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb_ctrl](index.html) module"]
pub struct BB_CTRL_SPEC;
impl crate::RegisterSpec for BB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bb_ctrl::R](R) reader structure"]
impl crate::Readable for BB_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bb_ctrl::W](W) writer structure"]
impl crate::Writable for BB_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BB_CTRL to value 0x60"]
impl crate::Resettable for BB_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x60
    }
}
