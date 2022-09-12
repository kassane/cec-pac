#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - KSO Select and control"]
    pub kso_sel: KSO_SEL,
    #[doc = "0x08 - 7:0\\]
This field returns the current state of the KSI pins."]
    pub ksi: KSI,
    #[doc = "0x0c - 7:0\\]
Each bit in this field is set on the falling edge of the corresponding KSI input pin.\n A KSI interrupt is generated when its corresponding status bit and interrupt enable bit are both set. KSI interrupts are logically ORed together to produce KSC_INT and KSC_INT_WAKE.\n Writing a '1' to a bit will clear it. Writing a '0' to a bit has no effect."]
    pub ksi_sts: KSI_STS,
    #[doc = "0x10 - 7:0\\]
Each bit in KSI_IEN enables interrupt generation due to highto-low transition on a KSI input.\n An interrupt is generated when the corresponding bits in KSI_STATUS and KSI_INT_EN are both set."]
    pub ksi_ien: KSI_IEN,
    #[doc = "0x14 - 0:0\\]
PREDRIVE_ENABLE enables the \n PREDRIVE mode to actively drive the KSO pins high for approximately 100ns before switching to open-drain operation.\n 0=Disable predrive on KSO pins\n 1=Enable predrive on KSO pins."]
    pub ext_ctrl: EXT_CTRL,
}
#[doc = "KSO_SEL (rw) register accessor: an alias for `Reg<KSO_SEL_SPEC>`"]
pub type KSO_SEL = crate::Reg<kso_sel::KSO_SEL_SPEC>;
#[doc = "KSO Select and control"]
pub mod kso_sel;
#[doc = "KSI (r) register accessor: an alias for `Reg<KSI_SPEC>`"]
pub type KSI = crate::Reg<ksi::KSI_SPEC>;
#[doc = "7:0\\]
This field returns the current state of the KSI pins."]
pub mod ksi;
#[doc = "KSI_STS (rw) register accessor: an alias for `Reg<KSI_STS_SPEC>`"]
pub type KSI_STS = crate::Reg<ksi_sts::KSI_STS_SPEC>;
#[doc = "7:0\\]
Each bit in this field is set on the falling edge of the corresponding KSI input pin.\n A KSI interrupt is generated when its corresponding status bit and interrupt enable bit are both set. KSI interrupts are logically ORed together to produce KSC_INT and KSC_INT_WAKE.\n Writing a '1' to a bit will clear it. Writing a '0' to a bit has no effect."]
pub mod ksi_sts;
#[doc = "KSI_IEN (rw) register accessor: an alias for `Reg<KSI_IEN_SPEC>`"]
pub type KSI_IEN = crate::Reg<ksi_ien::KSI_IEN_SPEC>;
#[doc = "7:0\\]
Each bit in KSI_IEN enables interrupt generation due to highto-low transition on a KSI input.\n An interrupt is generated when the corresponding bits in KSI_STATUS and KSI_INT_EN are both set."]
pub mod ksi_ien;
#[doc = "EXT_CTRL (rw) register accessor: an alias for `Reg<EXT_CTRL_SPEC>`"]
pub type EXT_CTRL = crate::Reg<ext_ctrl::EXT_CTRL_SPEC>;
#[doc = "0:0\\]
PREDRIVE_ENABLE enables the \n PREDRIVE mode to actively drive the KSO pins high for approximately 100ns before switching to open-drain operation.\n 0=Disable predrive on KSO pins\n 1=Enable predrive on KSO pins."]
pub mod ext_ctrl;
