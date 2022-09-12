#[doc = "Register `SRC11` reader"]
pub struct R(crate::R<SRC11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC11` writer"]
pub struct W(crate::W<SRC11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC11_SPEC>;
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
impl From<crate::W<SRC11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO000` reader - GPIO 000"]
pub type GPIO000_R = crate::BitReader<bool>;
#[doc = "Field `GPIO000` writer - GPIO 000"]
pub type GPIO000_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO002` reader - GPIO 002"]
pub type GPIO002_R = crate::BitReader<bool>;
#[doc = "Field `GPIO002` writer - GPIO 002"]
pub type GPIO002_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO003` reader - GPIO 003"]
pub type GPIO003_R = crate::BitReader<bool>;
#[doc = "Field `GPIO003` writer - GPIO 003"]
pub type GPIO003_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO004` reader - GPIO 004"]
pub type GPIO004_R = crate::BitReader<bool>;
#[doc = "Field `GPIO004` writer - GPIO 004"]
pub type GPIO004_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO012` reader - GPIO 012"]
pub type GPIO012_R = crate::BitReader<bool>;
#[doc = "Field `GPIO012` writer - GPIO 012"]
pub type GPIO012_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO013` reader - GPIO 013"]
pub type GPIO013_R = crate::BitReader<bool>;
#[doc = "Field `GPIO013` writer - GPIO 013"]
pub type GPIO013_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO015` reader - GPIO 015"]
pub type GPIO015_R = crate::BitReader<bool>;
#[doc = "Field `GPIO015` writer - GPIO 015"]
pub type GPIO015_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO016` reader - GPIO 016"]
pub type GPIO016_R = crate::BitReader<bool>;
#[doc = "Field `GPIO016` writer - GPIO 016"]
pub type GPIO016_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO020` reader - GPIO 020"]
pub type GPIO020_R = crate::BitReader<bool>;
#[doc = "Field `GPIO020` writer - GPIO 020"]
pub type GPIO020_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO021` reader - GPIO 021"]
pub type GPIO021_R = crate::BitReader<bool>;
#[doc = "Field `GPIO021` writer - GPIO 021"]
pub type GPIO021_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO022` reader - GPIO 022"]
pub type GPIO022_R = crate::BitReader<bool>;
#[doc = "Field `GPIO022` writer - GPIO 022"]
pub type GPIO022_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO023` reader - GPIO 023"]
pub type GPIO023_R = crate::BitReader<bool>;
#[doc = "Field `GPIO023` writer - GPIO 023"]
pub type GPIO023_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO024` reader - GPIO 024"]
pub type GPIO024_R = crate::BitReader<bool>;
#[doc = "Field `GPIO024` writer - GPIO 024"]
pub type GPIO024_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO026` reader - GPIO 026"]
pub type GPIO026_R = crate::BitReader<bool>;
#[doc = "Field `GPIO026` writer - GPIO 026"]
pub type GPIO026_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO027` reader - GPIO 027"]
pub type GPIO027_R = crate::BitReader<bool>;
#[doc = "Field `GPIO027` writer - GPIO 027"]
pub type GPIO027_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO030` reader - GPIO 030"]
pub type GPIO030_R = crate::BitReader<bool>;
#[doc = "Field `GPIO030` writer - GPIO 030"]
pub type GPIO030_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO031` reader - GPIO 031"]
pub type GPIO031_R = crate::BitReader<bool>;
#[doc = "Field `GPIO031` writer - GPIO 031"]
pub type GPIO031_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO032` reader - GPIO 032"]
pub type GPIO032_R = crate::BitReader<bool>;
#[doc = "Field `GPIO032` writer - GPIO 032"]
pub type GPIO032_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO033` reader - GPIO 033"]
pub type GPIO033_R = crate::BitReader<bool>;
#[doc = "Field `GPIO033` writer - GPIO 033"]
pub type GPIO033_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
#[doc = "Field `GPIO034` reader - GPIO 034"]
pub type GPIO034_R = crate::BitReader<bool>;
#[doc = "Field `GPIO034` writer - GPIO 034"]
pub type GPIO034_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC11_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIO 000"]
    #[inline(always)]
    pub fn gpio000(&self) -> GPIO000_R {
        GPIO000_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO 002"]
    #[inline(always)]
    pub fn gpio002(&self) -> GPIO002_R {
        GPIO002_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO 003"]
    #[inline(always)]
    pub fn gpio003(&self) -> GPIO003_R {
        GPIO003_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO 004"]
    #[inline(always)]
    pub fn gpio004(&self) -> GPIO004_R {
        GPIO004_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO 012"]
    #[inline(always)]
    pub fn gpio012(&self) -> GPIO012_R {
        GPIO012_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO 013"]
    #[inline(always)]
    pub fn gpio013(&self) -> GPIO013_R {
        GPIO013_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO 015"]
    #[inline(always)]
    pub fn gpio015(&self) -> GPIO015_R {
        GPIO015_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO 016"]
    #[inline(always)]
    pub fn gpio016(&self) -> GPIO016_R {
        GPIO016_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO 020"]
    #[inline(always)]
    pub fn gpio020(&self) -> GPIO020_R {
        GPIO020_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO 021"]
    #[inline(always)]
    pub fn gpio021(&self) -> GPIO021_R {
        GPIO021_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO 022"]
    #[inline(always)]
    pub fn gpio022(&self) -> GPIO022_R {
        GPIO022_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO 023"]
    #[inline(always)]
    pub fn gpio023(&self) -> GPIO023_R {
        GPIO023_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO 024"]
    #[inline(always)]
    pub fn gpio024(&self) -> GPIO024_R {
        GPIO024_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO 026"]
    #[inline(always)]
    pub fn gpio026(&self) -> GPIO026_R {
        GPIO026_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO 027"]
    #[inline(always)]
    pub fn gpio027(&self) -> GPIO027_R {
        GPIO027_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO 030"]
    #[inline(always)]
    pub fn gpio030(&self) -> GPIO030_R {
        GPIO030_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO 031"]
    #[inline(always)]
    pub fn gpio031(&self) -> GPIO031_R {
        GPIO031_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO 032"]
    #[inline(always)]
    pub fn gpio032(&self) -> GPIO032_R {
        GPIO032_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO 033"]
    #[inline(always)]
    pub fn gpio033(&self) -> GPIO033_R {
        GPIO033_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO 034"]
    #[inline(always)]
    pub fn gpio034(&self) -> GPIO034_R {
        GPIO034_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO 000"]
    #[inline(always)]
    pub fn gpio000(&mut self) -> GPIO000_W<0> {
        GPIO000_W::new(self)
    }
    #[doc = "Bit 2 - GPIO 002"]
    #[inline(always)]
    pub fn gpio002(&mut self) -> GPIO002_W<2> {
        GPIO002_W::new(self)
    }
    #[doc = "Bit 3 - GPIO 003"]
    #[inline(always)]
    pub fn gpio003(&mut self) -> GPIO003_W<3> {
        GPIO003_W::new(self)
    }
    #[doc = "Bit 4 - GPIO 004"]
    #[inline(always)]
    pub fn gpio004(&mut self) -> GPIO004_W<4> {
        GPIO004_W::new(self)
    }
    #[doc = "Bit 10 - GPIO 012"]
    #[inline(always)]
    pub fn gpio012(&mut self) -> GPIO012_W<10> {
        GPIO012_W::new(self)
    }
    #[doc = "Bit 11 - GPIO 013"]
    #[inline(always)]
    pub fn gpio013(&mut self) -> GPIO013_W<11> {
        GPIO013_W::new(self)
    }
    #[doc = "Bit 13 - GPIO 015"]
    #[inline(always)]
    pub fn gpio015(&mut self) -> GPIO015_W<13> {
        GPIO015_W::new(self)
    }
    #[doc = "Bit 14 - GPIO 016"]
    #[inline(always)]
    pub fn gpio016(&mut self) -> GPIO016_W<14> {
        GPIO016_W::new(self)
    }
    #[doc = "Bit 16 - GPIO 020"]
    #[inline(always)]
    pub fn gpio020(&mut self) -> GPIO020_W<16> {
        GPIO020_W::new(self)
    }
    #[doc = "Bit 17 - GPIO 021"]
    #[inline(always)]
    pub fn gpio021(&mut self) -> GPIO021_W<17> {
        GPIO021_W::new(self)
    }
    #[doc = "Bit 18 - GPIO 022"]
    #[inline(always)]
    pub fn gpio022(&mut self) -> GPIO022_W<18> {
        GPIO022_W::new(self)
    }
    #[doc = "Bit 19 - GPIO 023"]
    #[inline(always)]
    pub fn gpio023(&mut self) -> GPIO023_W<19> {
        GPIO023_W::new(self)
    }
    #[doc = "Bit 20 - GPIO 024"]
    #[inline(always)]
    pub fn gpio024(&mut self) -> GPIO024_W<20> {
        GPIO024_W::new(self)
    }
    #[doc = "Bit 22 - GPIO 026"]
    #[inline(always)]
    pub fn gpio026(&mut self) -> GPIO026_W<22> {
        GPIO026_W::new(self)
    }
    #[doc = "Bit 23 - GPIO 027"]
    #[inline(always)]
    pub fn gpio027(&mut self) -> GPIO027_W<23> {
        GPIO027_W::new(self)
    }
    #[doc = "Bit 24 - GPIO 030"]
    #[inline(always)]
    pub fn gpio030(&mut self) -> GPIO030_W<24> {
        GPIO030_W::new(self)
    }
    #[doc = "Bit 25 - GPIO 031"]
    #[inline(always)]
    pub fn gpio031(&mut self) -> GPIO031_W<25> {
        GPIO031_W::new(self)
    }
    #[doc = "Bit 26 - GPIO 032"]
    #[inline(always)]
    pub fn gpio032(&mut self) -> GPIO032_W<26> {
        GPIO032_W::new(self)
    }
    #[doc = "Bit 27 - GPIO 033"]
    #[inline(always)]
    pub fn gpio033(&mut self) -> GPIO033_W<27> {
        GPIO033_W::new(self)
    }
    #[doc = "Bit 28 - GPIO 034"]
    #[inline(always)]
    pub fn gpio034(&mut self) -> GPIO034_W<28> {
        GPIO034_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GIRQ11 SOURCE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src11](index.html) module"]
pub struct SRC11_SPEC;
impl crate::RegisterSpec for SRC11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src11::R](R) reader structure"]
impl crate::Readable for SRC11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src11::W](W) writer structure"]
impl crate::Writable for SRC11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRC11 to value 0"]
impl crate::Resettable for SRC11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
