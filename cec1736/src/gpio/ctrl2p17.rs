#[doc = "Register `CTRL2P17[%s]` reader"]
pub struct R(crate::R<CTRL2P17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2P17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2P17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2P17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2P17[%s]` writer"]
pub struct W(crate::W<CTRL2P17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2P17_SPEC>;
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
impl From<crate::W<CTRL2P17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2P17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEW_CTRL` reader - Selects slew rate on the pin. 1=fast 0=slow"]
pub type SLEW_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `SLEW_CTRL` writer - Selects slew rate on the pin. 1=fast 0=slow"]
pub type SLEW_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2P17_SPEC, bool, O>;
#[doc = "Field `DRIV_STREN` reader - Selects the drive strength on the pin. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
pub type DRIV_STREN_R = crate::FieldReader<u8, DRIV_STRENSELECT_A>;
#[doc = "Selects the drive strength on the pin. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIV_STRENSELECT_A {
    #[doc = "0: For PIO12 = 2mA, PIO24 = 4mA"]
    LEVEL0 = 0,
    #[doc = "1: For PIO12 = 4mA, PIO24 = 8mA"]
    LEVEL1 = 1,
    #[doc = "2: For PIO12 = 8mA, PIO24 = 16mA"]
    LEVEL2 = 2,
    #[doc = "3: For PIO12 = 12mA, PIO24 = 24mA"]
    LEVEL3 = 3,
}
impl From<DRIV_STRENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIV_STRENSELECT_A) -> Self {
        variant as _
    }
}
impl DRIV_STREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIV_STRENSELECT_A {
        match self.bits {
            0 => DRIV_STRENSELECT_A::LEVEL0,
            1 => DRIV_STRENSELECT_A::LEVEL1,
            2 => DRIV_STRENSELECT_A::LEVEL2,
            3 => DRIV_STRENSELECT_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == DRIV_STRENSELECT_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == DRIV_STRENSELECT_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == DRIV_STRENSELECT_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == DRIV_STRENSELECT_A::LEVEL3
    }
}
#[doc = "Field `DRIV_STREN` writer - Selects the drive strength on the pin. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
pub type DRIV_STREN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL2P17_SPEC, u8, DRIV_STRENSELECT_A, 2, O>;
impl<'a, const O: u8> DRIV_STREN_W<'a, O> {
    #[doc = "For PIO12 = 2mA, PIO24 = 4mA"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(DRIV_STRENSELECT_A::LEVEL0)
    }
    #[doc = "For PIO12 = 4mA, PIO24 = 8mA"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(DRIV_STRENSELECT_A::LEVEL1)
    }
    #[doc = "For PIO12 = 8mA, PIO24 = 16mA"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(DRIV_STRENSELECT_A::LEVEL2)
    }
    #[doc = "For PIO12 = 12mA, PIO24 = 24mA"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(DRIV_STRENSELECT_A::LEVEL3)
    }
}
impl R {
    #[doc = "Bit 0 - Selects slew rate on the pin. 1=fast 0=slow"]
    #[inline(always)]
    pub fn slew_ctrl(&self) -> SLEW_CTRL_R {
        SLEW_CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Selects the drive strength on the pin. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
    #[inline(always)]
    pub fn driv_stren(&self) -> DRIV_STREN_R {
        DRIV_STREN_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selects slew rate on the pin. 1=fast 0=slow"]
    #[inline(always)]
    pub fn slew_ctrl(&mut self) -> SLEW_CTRL_W<0> {
        SLEW_CTRL_W::new(self)
    }
    #[doc = "Bits 4:5 - Selects the drive strength on the pin. 00 = 2mA, 01 = 4mA, 10 = 8mA, 11 = 12mA"]
    #[inline(always)]
    pub fn driv_stren(&mut self) -> DRIV_STREN_W<4> {
        DRIV_STREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The GPIO PIN_CTRL2 Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2p17](index.html) module"]
pub struct CTRL2P17_SPEC;
impl crate::RegisterSpec for CTRL2P17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2p17::R](R) reader structure"]
impl crate::Readable for CTRL2P17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2p17::W](W) writer structure"]
impl crate::Writable for CTRL2P17_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2P17[%s]
to value 0"]
impl crate::Resettable for CTRL2P17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
