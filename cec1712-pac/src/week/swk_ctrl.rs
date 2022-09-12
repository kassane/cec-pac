#[doc = "Register `SWK_CTRL` reader"]
pub struct R(crate::R<SWK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWKTMR_PWRUP_EVT_STS` reader - This bit is set to 1 when the Sub-Week Alarm Counter Register decrements from 1 to 0 and the POWERUP_EN is 1.\n Writes of 1 clear this bit. Writes of 0 have no effect. Note: This bit MUST be cleared to remove a Sub-Week Timer Power-Up Event."]
pub type SWKTMR_PWRUP_EVT_STS_R = crate::BitReader<bool>;
#[doc = "Field `WKTMR_PWRUP_EVT_STS` reader - This bit is set to 1 when the Week Alarm Counter Register is greater than or equal the contents of the Week Timer Compare\n Register and the POWERUP_EN is 1. Writes of 1 clear this bit. Writes of 0 have no effect.\n Note: This bit does not have to be cleared to remove a Week Timer Power-Up Event."]
pub type WKTMR_PWRUP_EVT_STS_R = crate::BitReader<bool>;
#[doc = "Field `TEST` reader - Test"]
pub type TEST_R = crate::BitReader<bool>;
#[doc = "Field `TEST0` reader - Test"]
pub type TEST0_R = crate::BitReader<bool>;
#[doc = "Field `AU_RLD` reader - 1= No reload occurs when the Sub-Week Counter expires\n 0= Reloads the SUBWEEK_COUNTER_LOAD field into the Sub- Week Counter when the counter expires."]
pub type AU_RLD_R = crate::BitReader<bool>;
#[doc = "Field `SWK_TICK` reader - This field selects the clock source for the Sub-Week Counter."]
pub type SWK_TICK_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - This bit is set to 1 when the Sub-Week Alarm Counter Register decrements from 1 to 0 and the POWERUP_EN is 1.\n Writes of 1 clear this bit. Writes of 0 have no effect. Note: This bit MUST be cleared to remove a Sub-Week Timer Power-Up Event."]
    #[inline(always)]
    pub fn swktmr_pwrup_evt_sts(&self) -> SWKTMR_PWRUP_EVT_STS_R {
        SWKTMR_PWRUP_EVT_STS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set to 1 when the Week Alarm Counter Register is greater than or equal the contents of the Week Timer Compare\n Register and the POWERUP_EN is 1. Writes of 1 clear this bit. Writes of 0 have no effect.\n Note: This bit does not have to be cleared to remove a Week Timer Power-Up Event."]
    #[inline(always)]
    pub fn wktmr_pwrup_evt_sts(&self) -> WKTMR_PWRUP_EVT_STS_R {
        WKTMR_PWRUP_EVT_STS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Test"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Test"]
    #[inline(always)]
    pub fn test0(&self) -> TEST0_R {
        TEST0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1= No reload occurs when the Sub-Week Counter expires\n 0= Reloads the SUBWEEK_COUNTER_LOAD field into the Sub- Week Counter when the counter expires."]
    #[inline(always)]
    pub fn au_rld(&self) -> AU_RLD_R {
        AU_RLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - This field selects the clock source for the Sub-Week Counter."]
    #[inline(always)]
    pub fn swk_tick(&self) -> SWK_TICK_R {
        SWK_TICK_R::new(((self.bits >> 7) & 7) as u8)
    }
}
#[doc = "Sub-Week Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swk_ctrl](index.html) module"]
pub struct SWK_CTRL_SPEC;
impl crate::RegisterSpec for SWK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swk_ctrl::R](R) reader structure"]
impl crate::Readable for SWK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SWK_CTRL to value 0"]
impl crate::Resettable for SWK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
