#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Seconds Register"]
    pub sec: SEC,
    #[doc = "0x01 - Seconds Alarm Register"]
    pub sec_alarm: SEC_ALARM,
    #[doc = "0x02 - Minutes Register"]
    pub min: MIN,
    #[doc = "0x03 - Minutes Alarm Register"]
    pub min_alarm: MIN_ALARM,
    #[doc = "0x04 - Hours Register"]
    pub hr: HR,
    #[doc = "0x05 - Hours Alarm Register"]
    pub hr_alarm: HR_ALARM,
    #[doc = "0x06 - Day of Week Register"]
    pub day_of_wk: DAY_OF_WK,
    #[doc = "0x07 - Day of Month Register"]
    pub day_of_mon: DAY_OF_MON,
    #[doc = "0x08 - Month Register"]
    pub month: MONTH,
    #[doc = "0x09 - Year Register"]
    pub year: YEAR,
    #[doc = "0x0a - Register A"]
    pub rega: REGA,
    #[doc = "0x0b - Register B"]
    pub regb: REGB,
    #[doc = "0x0c - Register C"]
    pub regc: REGC,
    #[doc = "0x0d - Register D"]
    pub regd: REGD,
    _reserved14: [u8; 0x02],
    #[doc = "0x10 - RTC Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x14 - Week Alarm Register\\[7:0\\]
- ALARM_DAY_OF_WEEK This register, if written to a value in the range 1- -7, will inhibit the Alarm\n interrupt unless this field matches the contents of the Day of Week Register also."]
    pub wk_alarm: WK_ALARM,
    #[doc = "0x18 - Daylight Savings Forward Register"]
    pub daylt_savf: DAYLT_SAVF,
    #[doc = "0x1c - Daylight Savings Backward Register"]
    pub daylt_savb: DAYLT_SAVB,
}
#[doc = "SEC (rw) register accessor: an alias for `Reg<SEC_SPEC>`"]
pub type SEC = crate::Reg<sec::SEC_SPEC>;
#[doc = "Seconds Register"]
pub mod sec;
#[doc = "SEC_ALARM (rw) register accessor: an alias for `Reg<SEC_ALARM_SPEC>`"]
pub type SEC_ALARM = crate::Reg<sec_alarm::SEC_ALARM_SPEC>;
#[doc = "Seconds Alarm Register"]
pub mod sec_alarm;
#[doc = "MIN (rw) register accessor: an alias for `Reg<MIN_SPEC>`"]
pub type MIN = crate::Reg<min::MIN_SPEC>;
#[doc = "Minutes Register"]
pub mod min;
#[doc = "MIN_ALARM (rw) register accessor: an alias for `Reg<MIN_ALARM_SPEC>`"]
pub type MIN_ALARM = crate::Reg<min_alarm::MIN_ALARM_SPEC>;
#[doc = "Minutes Alarm Register"]
pub mod min_alarm;
#[doc = "HR (rw) register accessor: an alias for `Reg<HR_SPEC>`"]
pub type HR = crate::Reg<hr::HR_SPEC>;
#[doc = "Hours Register"]
pub mod hr;
#[doc = "HR_ALARM (rw) register accessor: an alias for `Reg<HR_ALARM_SPEC>`"]
pub type HR_ALARM = crate::Reg<hr_alarm::HR_ALARM_SPEC>;
#[doc = "Hours Alarm Register"]
pub mod hr_alarm;
#[doc = "DAY_OF_WK (rw) register accessor: an alias for `Reg<DAY_OF_WK_SPEC>`"]
pub type DAY_OF_WK = crate::Reg<day_of_wk::DAY_OF_WK_SPEC>;
#[doc = "Day of Week Register"]
pub mod day_of_wk;
#[doc = "DAY_OF_MON (rw) register accessor: an alias for `Reg<DAY_OF_MON_SPEC>`"]
pub type DAY_OF_MON = crate::Reg<day_of_mon::DAY_OF_MON_SPEC>;
#[doc = "Day of Month Register"]
pub mod day_of_mon;
#[doc = "MONTH (rw) register accessor: an alias for `Reg<MONTH_SPEC>`"]
pub type MONTH = crate::Reg<month::MONTH_SPEC>;
#[doc = "Month Register"]
pub mod month;
#[doc = "YEAR (rw) register accessor: an alias for `Reg<YEAR_SPEC>`"]
pub type YEAR = crate::Reg<year::YEAR_SPEC>;
#[doc = "Year Register"]
pub mod year;
#[doc = "REGA (rw) register accessor: an alias for `Reg<REGA_SPEC>`"]
pub type REGA = crate::Reg<rega::REGA_SPEC>;
#[doc = "Register A"]
pub mod rega;
#[doc = "REGB (rw) register accessor: an alias for `Reg<REGB_SPEC>`"]
pub type REGB = crate::Reg<regb::REGB_SPEC>;
#[doc = "Register B"]
pub mod regb;
#[doc = "REGC (rw) register accessor: an alias for `Reg<REGC_SPEC>`"]
pub type REGC = crate::Reg<regc::REGC_SPEC>;
#[doc = "Register C"]
pub mod regc;
#[doc = "REGD (rw) register accessor: an alias for `Reg<REGD_SPEC>`"]
pub type REGD = crate::Reg<regd::REGD_SPEC>;
#[doc = "Register D"]
pub mod regd;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTC Control Register"]
pub mod ctrl;
#[doc = "WK_ALARM (rw) register accessor: an alias for `Reg<WK_ALARM_SPEC>`"]
pub type WK_ALARM = crate::Reg<wk_alarm::WK_ALARM_SPEC>;
#[doc = "Week Alarm Register\\[7:0\\]
- ALARM_DAY_OF_WEEK This register, if written to a value in the range 1- -7, will inhibit the Alarm\n interrupt unless this field matches the contents of the Day of Week Register also."]
pub mod wk_alarm;
#[doc = "DAYLT_SAVF (rw) register accessor: an alias for `Reg<DAYLT_SAVF_SPEC>`"]
pub type DAYLT_SAVF = crate::Reg<daylt_savf::DAYLT_SAVF_SPEC>;
#[doc = "Daylight Savings Forward Register"]
pub mod daylt_savf;
#[doc = "DAYLT_SAVB (rw) register accessor: an alias for `Reg<DAYLT_SAVB_SPEC>`"]
pub type DAYLT_SAVB = crate::Reg<daylt_savb::DAYLT_SAVB_SPEC>;
#[doc = "Daylight Savings Backward Register"]
pub mod daylt_savb;
