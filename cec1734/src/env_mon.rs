#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External Diode 1 Temp Register"]
    pub ext1_temp: EXT1_TEMP,
    #[doc = "0x02 - Internal Temp Register"]
    pub int_temp: INT_TEMP,
    #[doc = "0x04 - External Diode 2 Temp Register"]
    pub ext2_temp: EXT2_TEMP,
    #[doc = "0x06 - External Diode 3 Temp Register"]
    pub ext3_temp: EXT3_TEMP,
    #[doc = "0x08 - External Diode 4 Temp Register"]
    pub ext4_temp: EXT4_TEMP,
    _reserved5: [u8; 0x04],
    #[doc = "0x0e - Stores the voltage Measured on VIN channel"]
    pub vin_volt: VIN_VOLT,
    _reserved6: [u8; 0x03],
    #[doc = "0x12 - Stores the fractional and integer data for External Diode 1A Register"]
    pub ext1a_temp: EXT1A_TEMP,
    #[doc = "0x14 - Stores the fractional and integer data for External Diode 2A Register"]
    pub ext2a_temp: EXT2A_TEMP,
    #[doc = "0x16 - Stores the fractional and integer data for External Diode 3A Register"]
    pub ext3a_temp: EXT3A_TEMP,
    #[doc = "0x18 - Stores the fractional and integer data for External Diode 4A Register"]
    pub ext4a_temp: EXT4A_TEMP,
    _reserved10: [u8; 0x07],
    #[doc = "0x21 - Stores the VCP Voltage Monitor data"]
    pub vcp_volt: VCP_VOLT,
    _reserved11: [u8; 0x01],
    #[doc = "0x23 - Stores the VTT Voltage Monitor data"]
    pub vtt_volt: VTT_VOLT,
    _reserved12: [u8; 0x07],
    #[doc = "0x2b - Controls temp sensing for external diodes"]
    pub temp_cfg1: TEMP_CFG1,
    #[doc = "0x2c - Controls temp sensing for external diodes"]
    pub temp_cfg2: TEMP_CFG2,
    #[doc = "0x2d - Controls Voltage sensing for external voltages"]
    pub volt_cfg: VOLT_CFG,
    #[doc = "0x2e - Controls Thermistor or diodes Configuration"]
    pub them_cfg: THEM_CFG,
    #[doc = "0x2f - Controls Temperature Conversion for the temperature channels"]
    pub cnvr_cfg: CNVR_CFG,
    #[doc = "0x30 - Software Averaging Enable"]
    pub avg_en: AVG_EN,
    _reserved18: [u8; 0x07],
    #[doc = "0x38 - Configure Beta compensation settings for External Diode1"]
    pub bcomp1_en: BCOMP1_EN,
    #[doc = "0x39 - Configure Beta compensation settings for External Diode2"]
    pub bcomp2_en: BCOMP2_EN,
    _reserved20: [u8; 0x06],
    #[doc = "0x40 - Lock Start Register"]
    pub lck_strt: LCK_STRT,
    #[doc = "0x41 - Fault Interrupt Status Register"]
    pub flt_intsts: FLT_INTSTS,
    #[doc = "0x42 - Fault temperature Status Register"]
    pub flt_tempsts: FLT_TEMPSTS,
    #[doc = "0x43 - ThermTrip Pin Status Register"]
    pub thrmtrp_sts: THRMTRP_STS,
    #[doc = "0x44 - Temperature of Internal Diode Register"]
    pub int_temp_sts: INT_TEMP_STS,
    #[doc = "0x45 - Volt Interrupt Status Register"]
    pub vlt_intsts: VLT_INTSTS,
    #[doc = "0x46 - VCP Limit Register"]
    pub vcp_limit: VCP_LIMIT,
    #[doc = "0x48 - VTR Limit Register"]
    pub vtr_limit: VTR_LIMIT,
    #[doc = "0x4a - VTT Limit Register"]
    pub vtt_limit: VTT_LIMIT,
    #[doc = "0x4c - VIN Limit Register"]
    pub vin_limit: VIN_LIMIT,
    #[doc = "0x4e - Low limit for External Diode 1 Register"]
    pub ext1_tmplo_lmt: EXT1_TMPLO_LMT,
    #[doc = "0x4f - High limit for External Diode 1 Register"]
    pub ext1_tmphi_lmt: EXT1_TMPHI_LMT,
    #[doc = "0x50 - Low limit for Internal Diode Register"]
    pub int_tmplo_lmt: INT_TMPLO_LMT,
    #[doc = "0x51 - High limit for Internal Diode Register"]
    pub int_tmphi_lmt: INT_TMPHI_LMT,
    #[doc = "0x52 - Low limit for External Diode 2 Register"]
    pub ext2_tmplo_lmt: EXT2_TMPLO_LMT,
    #[doc = "0x53 - High limit for External Diode 2 Register"]
    pub ext2_tmphi_lmt: EXT2_TMPHI_LMT,
    #[doc = "0x54 - Low limit for External Diode 3 Register"]
    pub ext3_tmplo_lmt: EXT3_TMPLO_LMT,
    #[doc = "0x55 - High limit for External Diode 3 Register"]
    pub ext3_tmphi_lmt: EXT3_TMPHI_LMT,
    #[doc = "0x56 - Low limit for External Diode 4 Register"]
    pub ext4_tmplo_lmt: EXT4_TMPLO_LMT,
    #[doc = "0x57 - High limit for External Diode 4 Register"]
    pub ext4_tmphi_lmt: EXT4_TMPHI_LMT,
    #[doc = "0x58 - Low limit for External Diode 1A Register"]
    pub ext1a_tmplo_lmt: EXT1A_TMPLO_LMT,
    #[doc = "0x59 - High limit for External Diode 1A Register"]
    pub ext1a_tmphi_lmt: EXT1A_TMPHI_LMT,
    #[doc = "0x5a - Low limit for External Diode 2A Register"]
    pub ext2a_tmplo_lmt: EXT2A_TMPLO_LMT,
    #[doc = "0x5b - High limit for External Diode 2A Register"]
    pub ext2a_tmphi_lmt: EXT2A_TMPHI_LMT,
    #[doc = "0x5c - Low limit for External Diode 3A Register"]
    pub ext3a_tmplo_lmt: EXT3A_TMPLO_LMT,
    #[doc = "0x5d - High limit for External Diode 3A Register"]
    pub ext3a_tmphi_lmt: EXT3A_TMPHI_LMT,
    #[doc = "0x5e - Low limit for External Diode 4A Register"]
    pub ext4a_tmplo_lmt: EXT4A_TMPLO_LMT,
    #[doc = "0x5f - High limit for External Diode 4A Register"]
    pub ext4a_tmphi_lmt: EXT4A_TMPHI_LMT,
    _reserved48: [u8; 0x04],
    #[doc = "0x64 - External Diode3 Beta compensation Register"]
    pub bcomp3_en: BCOMP3_EN,
    #[doc = "0x65 - External Diode4 Beta compensation Register"]
    pub bcomp4_en: BCOMP4_EN,
    _reserved50: [u8; 0x01],
    #[doc = "0x67 - Internal Diode Beta compensation Register"]
    pub bcomp_intd_en: BCOMP_INTD_EN,
    _reserved51: [u8; 0x04],
    #[doc = "0x6c - Conversion Seconds Rate Register"]
    pub conv_srate: CONV_SRATE,
    _reserved52: [u8; 0x01],
    #[doc = "0x6e - Conversion Mode Register"]
    pub conv_mod: CONV_MOD,
    _reserved53: [u8; 0x01],
    #[doc = "0x70 - REC Enable Register"]
    pub rec_en: REC_EN,
    #[doc = "0x71 - VSET Voltage Reading Register"]
    pub vset_vlt: VSET_VLT,
    _reserved55: [u8; 0x03],
    #[doc = "0x75 - Thermal Trip Temperature Diode 1 Register"]
    pub therm1: THERM1,
    #[doc = "0x76 - FailSafe Status Register"]
    pub flsf_sts: FLSF_STS,
    #[doc = "0x77 - FailSafe Configuration Register"]
    pub flsf_cfg: FLSF_CFG,
    #[doc = "0x78 - Shutdown Status Register"]
    pub shdn_sts: SHDN_STS,
    #[doc = "0x79 - Shutdown Configuration Register"]
    pub shdn_cfg: SHDN_CFG,
    #[doc = "0x7a - Fault Interrupt Status Enable Register"]
    pub flt_intsts_en: FLT_INTSTS_EN,
    #[doc = "0x7b - Temp Interrupt Status Enable Register"]
    pub tmp_intsts: TMP_INTSTS,
    #[doc = "0x7c - Special Function Register"]
    pub spcl_fn: SPCL_FN,
    #[doc = "0x7d - Int Temp Interrupt Status Enable Register"]
    pub inttmp_inten: INTTMP_INTEN,
    #[doc = "0x7e - Volt Interrupt Status Enable Register"]
    pub vlt_inten: VLT_INTEN,
    _reserved65: [u8; 0x01],
    #[doc = "0x80 - Thermal Trip Temperature Diode 2 Register"]
    pub thrmtrp_tmp2: THRMTRP_TMP2,
    #[doc = "0x81 - Thermal Trip Temperature Diode 3 Register"]
    pub thrmtrp_tmp3: THRMTRP_TMP3,
    #[doc = "0x82 - Thermal Trip Temperature Diode 4 Register"]
    pub thrmtrp_tmp4: THRMTRP_TMP4,
    #[doc = "0x83 - Thermal Trip Temperature Diode 1A Register"]
    pub thrmtrp_tmp1a: THRMTRP_TMP1A,
    #[doc = "0x84 - Thermal Trip Temperature Diode 2A Register"]
    pub thrmtrp_tmp2a: THRMTRP_TMP2A,
    #[doc = "0x85 - Thermal Trip Temperature Diode 3A Register"]
    pub thrmtrp_tmp3a: THRMTRP_TMP3A,
    #[doc = "0x86 - Thermal Trip Temperature Diode 4A Register"]
    pub thrmtrp_tmp4a: THRMTRP_TMP4A,
    _reserved72: [u8; 0x01],
    #[doc = "0x88 - Adjusted Channel 1 Register"]
    pub adj_ch1: ADJ_CH1,
    #[doc = "0x89 - Adjusted Channel 2 Register"]
    pub adj_ch2: ADJ_CH2,
    #[doc = "0x8a - Adjusted Channel 3 Register"]
    pub adj_ch3: ADJ_CH3,
    #[doc = "0x8b - Adjusted Channel 4 Register"]
    pub adj_ch4: ADJ_CH4,
    #[doc = "0x8c - Adjusted Channel 1A Register"]
    pub adj_ch1a: ADJ_CH1A,
    #[doc = "0x8d - Adjusted Channel 2A Register"]
    pub adj_ch2a: ADJ_CH2A,
    #[doc = "0x8e - Adjusted Channel 3A Register"]
    pub adj_ch3a: ADJ_CH3A,
    #[doc = "0x8f - Adjusted Channel 4A Register"]
    pub adj_ch4a: ADJ_CH4A,
    _reserved80: [u8; 0x6c],
    #[doc = "0xfc - Unlock Register"]
    pub unlck: UNLCK,
    _reserved81: [u8; 0x0303],
    #[doc = "0x400 - System Shutdown Reset Register"]
    pub sys_shdn_rst: SYS_SHDN_RST,
}
#[doc = "EXT1_TEMP (r) register accessor: an alias for `Reg<EXT1_TEMP_SPEC>`"]
pub type EXT1_TEMP = crate::Reg<ext1_temp::EXT1_TEMP_SPEC>;
#[doc = "External Diode 1 Temp Register"]
pub mod ext1_temp;
#[doc = "INT_TEMP (r) register accessor: an alias for `Reg<INT_TEMP_SPEC>`"]
pub type INT_TEMP = crate::Reg<int_temp::INT_TEMP_SPEC>;
#[doc = "Internal Temp Register"]
pub mod int_temp;
#[doc = "EXT2_TEMP (r) register accessor: an alias for `Reg<EXT2_TEMP_SPEC>`"]
pub type EXT2_TEMP = crate::Reg<ext2_temp::EXT2_TEMP_SPEC>;
#[doc = "External Diode 2 Temp Register"]
pub mod ext2_temp;
#[doc = "EXT3_TEMP (r) register accessor: an alias for `Reg<EXT3_TEMP_SPEC>`"]
pub type EXT3_TEMP = crate::Reg<ext3_temp::EXT3_TEMP_SPEC>;
#[doc = "External Diode 3 Temp Register"]
pub mod ext3_temp;
#[doc = "EXT4_TEMP (r) register accessor: an alias for `Reg<EXT4_TEMP_SPEC>`"]
pub type EXT4_TEMP = crate::Reg<ext4_temp::EXT4_TEMP_SPEC>;
#[doc = "External Diode 4 Temp Register"]
pub mod ext4_temp;
#[doc = "VIN_VOLT (r) register accessor: an alias for `Reg<VIN_VOLT_SPEC>`"]
pub type VIN_VOLT = crate::Reg<vin_volt::VIN_VOLT_SPEC>;
#[doc = "Stores the voltage Measured on VIN channel"]
pub mod vin_volt;
#[doc = "EXT1A_TEMP (r) register accessor: an alias for `Reg<EXT1A_TEMP_SPEC>`"]
pub type EXT1A_TEMP = crate::Reg<ext1a_temp::EXT1A_TEMP_SPEC>;
#[doc = "Stores the fractional and integer data for External Diode 1A Register"]
pub mod ext1a_temp;
#[doc = "EXT2A_TEMP (r) register accessor: an alias for `Reg<EXT2A_TEMP_SPEC>`"]
pub type EXT2A_TEMP = crate::Reg<ext2a_temp::EXT2A_TEMP_SPEC>;
#[doc = "Stores the fractional and integer data for External Diode 2A Register"]
pub mod ext2a_temp;
#[doc = "EXT3A_TEMP (r) register accessor: an alias for `Reg<EXT3A_TEMP_SPEC>`"]
pub type EXT3A_TEMP = crate::Reg<ext3a_temp::EXT3A_TEMP_SPEC>;
#[doc = "Stores the fractional and integer data for External Diode 3A Register"]
pub mod ext3a_temp;
#[doc = "EXT4A_TEMP (r) register accessor: an alias for `Reg<EXT4A_TEMP_SPEC>`"]
pub type EXT4A_TEMP = crate::Reg<ext4a_temp::EXT4A_TEMP_SPEC>;
#[doc = "Stores the fractional and integer data for External Diode 4A Register"]
pub mod ext4a_temp;
#[doc = "VCP_VOLT (r) register accessor: an alias for `Reg<VCP_VOLT_SPEC>`"]
pub type VCP_VOLT = crate::Reg<vcp_volt::VCP_VOLT_SPEC>;
#[doc = "Stores the VCP Voltage Monitor data"]
pub mod vcp_volt;
#[doc = "VTT_VOLT (r) register accessor: an alias for `Reg<VTT_VOLT_SPEC>`"]
pub type VTT_VOLT = crate::Reg<vtt_volt::VTT_VOLT_SPEC>;
#[doc = "Stores the VTT Voltage Monitor data"]
pub mod vtt_volt;
#[doc = "TEMP_CFG1 (rw) register accessor: an alias for `Reg<TEMP_CFG1_SPEC>`"]
pub type TEMP_CFG1 = crate::Reg<temp_cfg1::TEMP_CFG1_SPEC>;
#[doc = "Controls temp sensing for external diodes"]
pub mod temp_cfg1;
#[doc = "TEMP_CFG2 (rw) register accessor: an alias for `Reg<TEMP_CFG2_SPEC>`"]
pub type TEMP_CFG2 = crate::Reg<temp_cfg2::TEMP_CFG2_SPEC>;
#[doc = "Controls temp sensing for external diodes"]
pub mod temp_cfg2;
#[doc = "VOLT_CFG (rw) register accessor: an alias for `Reg<VOLT_CFG_SPEC>`"]
pub type VOLT_CFG = crate::Reg<volt_cfg::VOLT_CFG_SPEC>;
#[doc = "Controls Voltage sensing for external voltages"]
pub mod volt_cfg;
#[doc = "THEM_CFG (rw) register accessor: an alias for `Reg<THEM_CFG_SPEC>`"]
pub type THEM_CFG = crate::Reg<them_cfg::THEM_CFG_SPEC>;
#[doc = "Controls Thermistor or diodes Configuration"]
pub mod them_cfg;
#[doc = "CNVR_CFG (rw) register accessor: an alias for `Reg<CNVR_CFG_SPEC>`"]
pub type CNVR_CFG = crate::Reg<cnvr_cfg::CNVR_CFG_SPEC>;
#[doc = "Controls Temperature Conversion for the temperature channels"]
pub mod cnvr_cfg;
#[doc = "AVG_EN (rw) register accessor: an alias for `Reg<AVG_EN_SPEC>`"]
pub type AVG_EN = crate::Reg<avg_en::AVG_EN_SPEC>;
#[doc = "Software Averaging Enable"]
pub mod avg_en;
#[doc = "BCOMP1_EN (rw) register accessor: an alias for `Reg<BCOMP1_EN_SPEC>`"]
pub type BCOMP1_EN = crate::Reg<bcomp1_en::BCOMP1_EN_SPEC>;
#[doc = "Configure Beta compensation settings for External Diode1"]
pub mod bcomp1_en;
#[doc = "BCOMP2_EN (rw) register accessor: an alias for `Reg<BCOMP2_EN_SPEC>`"]
pub type BCOMP2_EN = crate::Reg<bcomp2_en::BCOMP2_EN_SPEC>;
#[doc = "Configure Beta compensation settings for External Diode2"]
pub mod bcomp2_en;
#[doc = "LCK_STRT (rw) register accessor: an alias for `Reg<LCK_STRT_SPEC>`"]
pub type LCK_STRT = crate::Reg<lck_strt::LCK_STRT_SPEC>;
#[doc = "Lock Start Register"]
pub mod lck_strt;
#[doc = "FLT_INTSTS (rw) register accessor: an alias for `Reg<FLT_INTSTS_SPEC>`"]
pub type FLT_INTSTS = crate::Reg<flt_intsts::FLT_INTSTS_SPEC>;
#[doc = "Fault Interrupt Status Register"]
pub mod flt_intsts;
#[doc = "FLT_TEMPSTS (rw) register accessor: an alias for `Reg<FLT_TEMPSTS_SPEC>`"]
pub type FLT_TEMPSTS = crate::Reg<flt_tempsts::FLT_TEMPSTS_SPEC>;
#[doc = "Fault temperature Status Register"]
pub mod flt_tempsts;
#[doc = "THRMTRP_STS (r) register accessor: an alias for `Reg<THRMTRP_STS_SPEC>`"]
pub type THRMTRP_STS = crate::Reg<thrmtrp_sts::THRMTRP_STS_SPEC>;
#[doc = "ThermTrip Pin Status Register"]
pub mod thrmtrp_sts;
#[doc = "INT_TEMP_STS (rw) register accessor: an alias for `Reg<INT_TEMP_STS_SPEC>`"]
pub type INT_TEMP_STS = crate::Reg<int_temp_sts::INT_TEMP_STS_SPEC>;
#[doc = "Temperature of Internal Diode Register"]
pub mod int_temp_sts;
#[doc = "VLT_INTSTS (rw) register accessor: an alias for `Reg<VLT_INTSTS_SPEC>`"]
pub type VLT_INTSTS = crate::Reg<vlt_intsts::VLT_INTSTS_SPEC>;
#[doc = "Volt Interrupt Status Register"]
pub mod vlt_intsts;
#[doc = "VCP_LIMIT (rw) register accessor: an alias for `Reg<VCP_LIMIT_SPEC>`"]
pub type VCP_LIMIT = crate::Reg<vcp_limit::VCP_LIMIT_SPEC>;
#[doc = "VCP Limit Register"]
pub mod vcp_limit;
#[doc = "VTR_LIMIT (rw) register accessor: an alias for `Reg<VTR_LIMIT_SPEC>`"]
pub type VTR_LIMIT = crate::Reg<vtr_limit::VTR_LIMIT_SPEC>;
#[doc = "VTR Limit Register"]
pub mod vtr_limit;
#[doc = "VTT_LIMIT (rw) register accessor: an alias for `Reg<VTT_LIMIT_SPEC>`"]
pub type VTT_LIMIT = crate::Reg<vtt_limit::VTT_LIMIT_SPEC>;
#[doc = "VTT Limit Register"]
pub mod vtt_limit;
#[doc = "VIN_LIMIT (rw) register accessor: an alias for `Reg<VIN_LIMIT_SPEC>`"]
pub type VIN_LIMIT = crate::Reg<vin_limit::VIN_LIMIT_SPEC>;
#[doc = "VIN Limit Register"]
pub mod vin_limit;
#[doc = "EXT1_TMPLO_LMT (rw) register accessor: an alias for `Reg<EXT1_TMPLO_LMT_SPEC>`"]
pub type EXT1_TMPLO_LMT = crate::Reg<ext1_tmplo_lmt::EXT1_TMPLO_LMT_SPEC>;
#[doc = "Low limit for External Diode 1 Register"]
pub mod ext1_tmplo_lmt;
#[doc = "EXT1_TMPHI_LMT (rw) register accessor: an alias for `Reg<EXT1_TMPHI_LMT_SPEC>`"]
pub type EXT1_TMPHI_LMT = crate::Reg<ext1_tmphi_lmt::EXT1_TMPHI_LMT_SPEC>;
#[doc = "High limit for External Diode 1 Register"]
pub mod ext1_tmphi_lmt;
#[doc = "INT_TMPLO_LMT (rw) register accessor: an alias for `Reg<INT_TMPLO_LMT_SPEC>`"]
pub type INT_TMPLO_LMT = crate::Reg<int_tmplo_lmt::INT_TMPLO_LMT_SPEC>;
#[doc = "Low limit for Internal Diode Register"]
pub mod int_tmplo_lmt;
#[doc = "INT_TMPHI_LMT (rw) register accessor: an alias for `Reg<INT_TMPHI_LMT_SPEC>`"]
pub type INT_TMPHI_LMT = crate::Reg<int_tmphi_lmt::INT_TMPHI_LMT_SPEC>;
#[doc = "High limit for Internal Diode Register"]
pub mod int_tmphi_lmt;
#[doc = "EXT2_TMPLO_LMT (rw) register accessor: an alias for `Reg<EXT2_TMPLO_LMT_SPEC>`"]
pub type EXT2_TMPLO_LMT = crate::Reg<ext2_tmplo_lmt::EXT2_TMPLO_LMT_SPEC>;
#[doc = "Low limit for External Diode 2 Register"]
pub mod ext2_tmplo_lmt;
#[doc = "EXT2_TMPHI_LMT (rw) register accessor: an alias for `Reg<EXT2_TMPHI_LMT_SPEC>`"]
pub type EXT2_TMPHI_LMT = crate::Reg<ext2_tmphi_lmt::EXT2_TMPHI_LMT_SPEC>;
#[doc = "High limit for External Diode 2 Register"]
pub mod ext2_tmphi_lmt;
#[doc = "EXT3_TMPLO_LMT (rw) register accessor: an alias for `Reg<EXT3_TMPLO_LMT_SPEC>`"]
pub type EXT3_TMPLO_LMT = crate::Reg<ext3_tmplo_lmt::EXT3_TMPLO_LMT_SPEC>;
#[doc = "Low limit for External Diode 3 Register"]
pub mod ext3_tmplo_lmt;
#[doc = "EXT3_TMPHI_LMT (rw) register accessor: an alias for `Reg<EXT3_TMPHI_LMT_SPEC>`"]
pub type EXT3_TMPHI_LMT = crate::Reg<ext3_tmphi_lmt::EXT3_TMPHI_LMT_SPEC>;
#[doc = "High limit for External Diode 3 Register"]
pub mod ext3_tmphi_lmt;
#[doc = "EXT4_TMPLO_LMT (rw) register accessor: an alias for `Reg<EXT4_TMPLO_LMT_SPEC>`"]
pub type EXT4_TMPLO_LMT = crate::Reg<ext4_tmplo_lmt::EXT4_TMPLO_LMT_SPEC>;
#[doc = "Low limit for External Diode 4 Register"]
pub mod ext4_tmplo_lmt;
#[doc = "EXT4_TMPHI_LMT (rw) register accessor: an alias for `Reg<EXT4_TMPHI_LMT_SPEC>`"]
pub type EXT4_TMPHI_LMT = crate::Reg<ext4_tmphi_lmt::EXT4_TMPHI_LMT_SPEC>;
#[doc = "High limit for External Diode 4 Register"]
pub mod ext4_tmphi_lmt;
#[doc = "EXT1A_TMPLO_LMT (rw) register accessor: an alias for `Reg<EXT1A_TMPLO_LMT_SPEC>`"]
pub type EXT1A_TMPLO_LMT = crate::Reg<ext1a_tmplo_lmt::EXT1A_TMPLO_LMT_SPEC>;
#[doc = "Low limit for External Diode 1A Register"]
pub mod ext1a_tmplo_lmt;
#[doc = "EXT1A_TMPHI_LMT (rw) register accessor: an alias for `Reg<EXT1A_TMPHI_LMT_SPEC>`"]
pub type EXT1A_TMPHI_LMT = crate::Reg<ext1a_tmphi_lmt::EXT1A_TMPHI_LMT_SPEC>;
#[doc = "High limit for External Diode 1A Register"]
pub mod ext1a_tmphi_lmt;
#[doc = "EXT2A_TMPLO_LMT (rw) register accessor: an alias for `Reg<EXT2A_TMPLO_LMT_SPEC>`"]
pub type EXT2A_TMPLO_LMT = crate::Reg<ext2a_tmplo_lmt::EXT2A_TMPLO_LMT_SPEC>;
#[doc = "Low limit for External Diode 2A Register"]
pub mod ext2a_tmplo_lmt;
#[doc = "EXT2A_TMPHI_LMT (rw) register accessor: an alias for `Reg<EXT2A_TMPHI_LMT_SPEC>`"]
pub type EXT2A_TMPHI_LMT = crate::Reg<ext2a_tmphi_lmt::EXT2A_TMPHI_LMT_SPEC>;
#[doc = "High limit for External Diode 2A Register"]
pub mod ext2a_tmphi_lmt;
#[doc = "EXT3A_TMPLO_LMT (rw) register accessor: an alias for `Reg<EXT3A_TMPLO_LMT_SPEC>`"]
pub type EXT3A_TMPLO_LMT = crate::Reg<ext3a_tmplo_lmt::EXT3A_TMPLO_LMT_SPEC>;
#[doc = "Low limit for External Diode 3A Register"]
pub mod ext3a_tmplo_lmt;
#[doc = "EXT3A_TMPHI_LMT (rw) register accessor: an alias for `Reg<EXT3A_TMPHI_LMT_SPEC>`"]
pub type EXT3A_TMPHI_LMT = crate::Reg<ext3a_tmphi_lmt::EXT3A_TMPHI_LMT_SPEC>;
#[doc = "High limit for External Diode 3A Register"]
pub mod ext3a_tmphi_lmt;
#[doc = "EXT4A_TMPLO_LMT (rw) register accessor: an alias for `Reg<EXT4A_TMPLO_LMT_SPEC>`"]
pub type EXT4A_TMPLO_LMT = crate::Reg<ext4a_tmplo_lmt::EXT4A_TMPLO_LMT_SPEC>;
#[doc = "Low limit for External Diode 4A Register"]
pub mod ext4a_tmplo_lmt;
#[doc = "EXT4A_TMPHI_LMT (rw) register accessor: an alias for `Reg<EXT4A_TMPHI_LMT_SPEC>`"]
pub type EXT4A_TMPHI_LMT = crate::Reg<ext4a_tmphi_lmt::EXT4A_TMPHI_LMT_SPEC>;
#[doc = "High limit for External Diode 4A Register"]
pub mod ext4a_tmphi_lmt;
#[doc = "BCOMP3_EN (rw) register accessor: an alias for `Reg<BCOMP3_EN_SPEC>`"]
pub type BCOMP3_EN = crate::Reg<bcomp3_en::BCOMP3_EN_SPEC>;
#[doc = "External Diode3 Beta compensation Register"]
pub mod bcomp3_en;
#[doc = "BCOMP4_EN (rw) register accessor: an alias for `Reg<BCOMP4_EN_SPEC>`"]
pub type BCOMP4_EN = crate::Reg<bcomp4_en::BCOMP4_EN_SPEC>;
#[doc = "External Diode4 Beta compensation Register"]
pub mod bcomp4_en;
#[doc = "BCOMP_INTD_EN (rw) register accessor: an alias for `Reg<BCOMP_INTD_EN_SPEC>`"]
pub type BCOMP_INTD_EN = crate::Reg<bcomp_intd_en::BCOMP_INTD_EN_SPEC>;
#[doc = "Internal Diode Beta compensation Register"]
pub mod bcomp_intd_en;
#[doc = "CONV_SRATE (rw) register accessor: an alias for `Reg<CONV_SRATE_SPEC>`"]
pub type CONV_SRATE = crate::Reg<conv_srate::CONV_SRATE_SPEC>;
#[doc = "Conversion Seconds Rate Register"]
pub mod conv_srate;
#[doc = "CONV_MOD (rw) register accessor: an alias for `Reg<CONV_MOD_SPEC>`"]
pub type CONV_MOD = crate::Reg<conv_mod::CONV_MOD_SPEC>;
#[doc = "Conversion Mode Register"]
pub mod conv_mod;
#[doc = "REC_EN (rw) register accessor: an alias for `Reg<REC_EN_SPEC>`"]
pub type REC_EN = crate::Reg<rec_en::REC_EN_SPEC>;
#[doc = "REC Enable Register"]
pub mod rec_en;
#[doc = "VSET_VLT (r) register accessor: an alias for `Reg<VSET_VLT_SPEC>`"]
pub type VSET_VLT = crate::Reg<vset_vlt::VSET_VLT_SPEC>;
#[doc = "VSET Voltage Reading Register"]
pub mod vset_vlt;
#[doc = "THERM1 (r) register accessor: an alias for `Reg<THERM1_SPEC>`"]
pub type THERM1 = crate::Reg<therm1::THERM1_SPEC>;
#[doc = "Thermal Trip Temperature Diode 1 Register"]
pub mod therm1;
#[doc = "FLSF_STS (r) register accessor: an alias for `Reg<FLSF_STS_SPEC>`"]
pub type FLSF_STS = crate::Reg<flsf_sts::FLSF_STS_SPEC>;
#[doc = "FailSafe Status Register"]
pub mod flsf_sts;
#[doc = "FLSF_CFG (rw) register accessor: an alias for `Reg<FLSF_CFG_SPEC>`"]
pub type FLSF_CFG = crate::Reg<flsf_cfg::FLSF_CFG_SPEC>;
#[doc = "FailSafe Configuration Register"]
pub mod flsf_cfg;
#[doc = "SHDN_STS (r) register accessor: an alias for `Reg<SHDN_STS_SPEC>`"]
pub type SHDN_STS = crate::Reg<shdn_sts::SHDN_STS_SPEC>;
#[doc = "Shutdown Status Register"]
pub mod shdn_sts;
#[doc = "SHDN_CFG (rw) register accessor: an alias for `Reg<SHDN_CFG_SPEC>`"]
pub type SHDN_CFG = crate::Reg<shdn_cfg::SHDN_CFG_SPEC>;
#[doc = "Shutdown Configuration Register"]
pub mod shdn_cfg;
#[doc = "FLT_INTSTS_EN (rw) register accessor: an alias for `Reg<FLT_INTSTS_EN_SPEC>`"]
pub type FLT_INTSTS_EN = crate::Reg<flt_intsts_en::FLT_INTSTS_EN_SPEC>;
#[doc = "Fault Interrupt Status Enable Register"]
pub mod flt_intsts_en;
#[doc = "TMP_INTSTS (rw) register accessor: an alias for `Reg<TMP_INTSTS_SPEC>`"]
pub type TMP_INTSTS = crate::Reg<tmp_intsts::TMP_INTSTS_SPEC>;
#[doc = "Temp Interrupt Status Enable Register"]
pub mod tmp_intsts;
#[doc = "SPCL_FN (rw) register accessor: an alias for `Reg<SPCL_FN_SPEC>`"]
pub type SPCL_FN = crate::Reg<spcl_fn::SPCL_FN_SPEC>;
#[doc = "Special Function Register"]
pub mod spcl_fn;
#[doc = "INTTMP_INTEN (rw) register accessor: an alias for `Reg<INTTMP_INTEN_SPEC>`"]
pub type INTTMP_INTEN = crate::Reg<inttmp_inten::INTTMP_INTEN_SPEC>;
#[doc = "Int Temp Interrupt Status Enable Register"]
pub mod inttmp_inten;
#[doc = "VLT_INTEN (rw) register accessor: an alias for `Reg<VLT_INTEN_SPEC>`"]
pub type VLT_INTEN = crate::Reg<vlt_inten::VLT_INTEN_SPEC>;
#[doc = "Volt Interrupt Status Enable Register"]
pub mod vlt_inten;
#[doc = "THRMTRP_TMP2 (rw) register accessor: an alias for `Reg<THRMTRP_TMP2_SPEC>`"]
pub type THRMTRP_TMP2 = crate::Reg<thrmtrp_tmp2::THRMTRP_TMP2_SPEC>;
#[doc = "Thermal Trip Temperature Diode 2 Register"]
pub mod thrmtrp_tmp2;
#[doc = "THRMTRP_TMP3 (rw) register accessor: an alias for `Reg<THRMTRP_TMP3_SPEC>`"]
pub type THRMTRP_TMP3 = crate::Reg<thrmtrp_tmp3::THRMTRP_TMP3_SPEC>;
#[doc = "Thermal Trip Temperature Diode 3 Register"]
pub mod thrmtrp_tmp3;
#[doc = "THRMTRP_TMP4 (rw) register accessor: an alias for `Reg<THRMTRP_TMP4_SPEC>`"]
pub type THRMTRP_TMP4 = crate::Reg<thrmtrp_tmp4::THRMTRP_TMP4_SPEC>;
#[doc = "Thermal Trip Temperature Diode 4 Register"]
pub mod thrmtrp_tmp4;
#[doc = "THRMTRP_TMP1A (rw) register accessor: an alias for `Reg<THRMTRP_TMP1A_SPEC>`"]
pub type THRMTRP_TMP1A = crate::Reg<thrmtrp_tmp1a::THRMTRP_TMP1A_SPEC>;
#[doc = "Thermal Trip Temperature Diode 1A Register"]
pub mod thrmtrp_tmp1a;
#[doc = "THRMTRP_TMP2A (rw) register accessor: an alias for `Reg<THRMTRP_TMP2A_SPEC>`"]
pub type THRMTRP_TMP2A = crate::Reg<thrmtrp_tmp2a::THRMTRP_TMP2A_SPEC>;
#[doc = "Thermal Trip Temperature Diode 2A Register"]
pub mod thrmtrp_tmp2a;
#[doc = "THRMTRP_TMP3A (rw) register accessor: an alias for `Reg<THRMTRP_TMP3A_SPEC>`"]
pub type THRMTRP_TMP3A = crate::Reg<thrmtrp_tmp3a::THRMTRP_TMP3A_SPEC>;
#[doc = "Thermal Trip Temperature Diode 3A Register"]
pub mod thrmtrp_tmp3a;
#[doc = "THRMTRP_TMP4A (rw) register accessor: an alias for `Reg<THRMTRP_TMP4A_SPEC>`"]
pub type THRMTRP_TMP4A = crate::Reg<thrmtrp_tmp4a::THRMTRP_TMP4A_SPEC>;
#[doc = "Thermal Trip Temperature Diode 4A Register"]
pub mod thrmtrp_tmp4a;
#[doc = "ADJ_CH1 (rw) register accessor: an alias for `Reg<ADJ_CH1_SPEC>`"]
pub type ADJ_CH1 = crate::Reg<adj_ch1::ADJ_CH1_SPEC>;
#[doc = "Adjusted Channel 1 Register"]
pub mod adj_ch1;
#[doc = "ADJ_CH2 (rw) register accessor: an alias for `Reg<ADJ_CH2_SPEC>`"]
pub type ADJ_CH2 = crate::Reg<adj_ch2::ADJ_CH2_SPEC>;
#[doc = "Adjusted Channel 2 Register"]
pub mod adj_ch2;
#[doc = "ADJ_CH3 (rw) register accessor: an alias for `Reg<ADJ_CH3_SPEC>`"]
pub type ADJ_CH3 = crate::Reg<adj_ch3::ADJ_CH3_SPEC>;
#[doc = "Adjusted Channel 3 Register"]
pub mod adj_ch3;
#[doc = "ADJ_CH4 (rw) register accessor: an alias for `Reg<ADJ_CH4_SPEC>`"]
pub type ADJ_CH4 = crate::Reg<adj_ch4::ADJ_CH4_SPEC>;
#[doc = "Adjusted Channel 4 Register"]
pub mod adj_ch4;
#[doc = "ADJ_CH1A (rw) register accessor: an alias for `Reg<ADJ_CH1A_SPEC>`"]
pub type ADJ_CH1A = crate::Reg<adj_ch1a::ADJ_CH1A_SPEC>;
#[doc = "Adjusted Channel 1A Register"]
pub mod adj_ch1a;
#[doc = "ADJ_CH2A (rw) register accessor: an alias for `Reg<ADJ_CH2A_SPEC>`"]
pub type ADJ_CH2A = crate::Reg<adj_ch2a::ADJ_CH2A_SPEC>;
#[doc = "Adjusted Channel 2A Register"]
pub mod adj_ch2a;
#[doc = "ADJ_CH3A (rw) register accessor: an alias for `Reg<ADJ_CH3A_SPEC>`"]
pub type ADJ_CH3A = crate::Reg<adj_ch3a::ADJ_CH3A_SPEC>;
#[doc = "Adjusted Channel 3A Register"]
pub mod adj_ch3a;
#[doc = "ADJ_CH4A (rw) register accessor: an alias for `Reg<ADJ_CH4A_SPEC>`"]
pub type ADJ_CH4A = crate::Reg<adj_ch4a::ADJ_CH4A_SPEC>;
#[doc = "Adjusted Channel 4A Register"]
pub mod adj_ch4a;
#[doc = "UNLCK (r) register accessor: an alias for `Reg<UNLCK_SPEC>`"]
pub type UNLCK = crate::Reg<unlck::UNLCK_SPEC>;
#[doc = "Unlock Register"]
pub mod unlck;
#[doc = "SYS_SHDN_RST (r) register accessor: an alias for `Reg<SYS_SHDN_RST_SPEC>`"]
pub type SYS_SHDN_RST = crate::Reg<sys_shdn_rst::SYS_SHDN_RST_SPEC>;
#[doc = "System Shutdown Reset Register"]
pub mod sys_shdn_rst;
