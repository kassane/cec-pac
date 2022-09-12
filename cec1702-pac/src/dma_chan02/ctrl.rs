#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN` reader - This is a control field. Note: This bit only applies to Hardware Flow Control mode.\n 1= This channel is enabled and will service transfer requests\n 0=This channel is disabled. All transfer requests are ignored."]
pub type RUN_R = crate::BitReader<bool>;
#[doc = "Field `RUN` writer - This is a control field. Note: This bit only applies to Hardware Flow Control mode.\n 1= This channel is enabled and will service transfer requests\n 0=This channel is disabled. All transfer requests are ignored."]
pub type RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `REQ` reader - This is a status field.\n 1= There is a transfer request from the Master Device\n 0= There is no transfer request from the Master Device"]
pub type REQ_R = crate::BitReader<bool>;
#[doc = "Field `REQ` writer - This is a status field.\n 1= There is a transfer request from the Master Device\n 0= There is no transfer request from the Master Device"]
pub type REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DONE` reader - This is a status signal. It is only valid while DMA Channel Control: Run is Enabled. \n This is the inverse of the DMA Channel Control: Busy field, except this is qualified with the DMA Channel Control:Run field.\n 1=Channel is done\n 0=Channel is not done or it is OFF"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - This is a status signal. It is only valid while DMA Channel Control: Run is Enabled. \n This is the inverse of the DMA Channel Control: Busy field, except this is qualified with the DMA Channel Control:Run field.\n 1=Channel is done\n 0=Channel is not done or it is OFF"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STS` reader - This is a status signal. The status decode is listed in priority order with the highest priority first.\n 3: Error detected by the DMA\n 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term\n 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address\n 0: DMA Channel Control:Run is Disabled (0x0)"]
pub type STS_R = crate::FieldReader<u8, STSSELECT_A>;
#[doc = "This is a status signal. The status decode is listed in priority order with the highest priority first.\n 3: Error detected by the DMA\n 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term\n 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address\n 0: DMA Channel Control:Run is Disabled (0x0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STSSELECT_A {
    #[doc = "3: 3: Error detected by the DMA"]
    ERROR = 3,
    #[doc = "2: 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term"]
    EXT_DONE = 2,
    #[doc = "1: 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address"]
    LOC_DONE = 1,
    #[doc = "0: 0: DMA Channel Control:Run is Disabled (0x0)"]
    DIS = 0,
}
impl From<STSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: STSSELECT_A) -> Self {
        variant as _
    }
}
impl STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STSSELECT_A {
        match self.bits {
            3 => STSSELECT_A::ERROR,
            2 => STSSELECT_A::EXT_DONE,
            1 => STSSELECT_A::LOC_DONE,
            0 => STSSELECT_A::DIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == STSSELECT_A::ERROR
    }
    #[doc = "Checks if the value of the field is `EXT_DONE`"]
    #[inline(always)]
    pub fn is_ext_done(&self) -> bool {
        *self == STSSELECT_A::EXT_DONE
    }
    #[doc = "Checks if the value of the field is `LOC_DONE`"]
    #[inline(always)]
    pub fn is_loc_done(&self) -> bool {
        *self == STSSELECT_A::LOC_DONE
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == STSSELECT_A::DIS
    }
}
#[doc = "Field `STS` writer - This is a status signal. The status decode is listed in priority order with the highest priority first.\n 3: Error detected by the DMA\n 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term\n 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address\n 0: DMA Channel Control:Run is Disabled (0x0)"]
pub type STS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, STSSELECT_A, 2, O>;
impl<'a, const O: u8> STS_W<'a, O> {
    #[doc = "3: Error detected by the DMA"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(STSSELECT_A::ERROR)
    }
    #[doc = "2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term"]
    #[inline(always)]
    pub fn ext_done(self) -> &'a mut W {
        self.variant(STSSELECT_A::EXT_DONE)
    }
    #[doc = "1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address"]
    #[inline(always)]
    pub fn loc_done(self) -> &'a mut W {
        self.variant(STSSELECT_A::LOC_DONE)
    }
    #[doc = "0: DMA Channel Control:Run is Disabled (0x0)"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(STSSELECT_A::DIS)
    }
}
#[doc = "Field `BUSY` reader - This is a status signal.\n 1=The DMA Channel is busy (FSM is not IDLE)\n 0=The DMA Channel is not busy (FSM is IDLE)"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - This is a status signal.\n 1=The DMA Channel is busy (FSM is not IDLE)\n 0=The DMA Channel is not busy (FSM is IDLE)"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TX_DIR` reader - This determines the direction of the DMA Transfer.\n 1=Data Packet Read from Memory Start Address followed by Data Packet Write to Device Address\n 0=Data Packet Read from Device Address followed by Data Packet Write to Memory Start Address"]
pub type TX_DIR_R = crate::BitReader<bool>;
#[doc = "Field `TX_DIR` writer - This determines the direction of the DMA Transfer.\n 1=Data Packet Read from Memory Start Address followed by Data Packet Write to Device Address\n 0=Data Packet Read from Device Address followed by Data Packet Write to Memory Start Address"]
pub type TX_DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `HW_FLOW_CTRL_DEV` reader - This is the device that is connected to this channel as its Hardware Flow Control master.\n The Flow Control Interface is a bus with each master concatenated onto it.\n This selects which bus index of the concatenated Flow Control Interface bus is targeted towards this channel.\n The Flow Control Interface Port list is dma_req, dma_term, and dma_done."]
pub type HW_FLOW_CTRL_DEV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HW_FLOW_CTRL_DEV` writer - This is the device that is connected to this channel as its Hardware Flow Control master.\n The Flow Control Interface is a bus with each master concatenated onto it.\n This selects which bus index of the concatenated Flow Control Interface bus is targeted towards this channel.\n The Flow Control Interface Port list is dma_req, dma_term, and dma_done."]
pub type HW_FLOW_CTRL_DEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `INC_MEM_ADDR` reader - This will enable an auto-increment to the DMA Channel Memory Address.\n 1=Increment the DMA Channel Memory Address by DMA Channel Control:Transfer Size after every Data Packet transfer\n 0=Do nothing"]
pub type INC_MEM_ADDR_R = crate::BitReader<bool>;
#[doc = "Field `INC_MEM_ADDR` writer - This will enable an auto-increment to the DMA Channel Memory Address.\n 1=Increment the DMA Channel Memory Address by DMA Channel Control:Transfer Size after every Data Packet transfer\n 0=Do nothing"]
pub type INC_MEM_ADDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `INC_DEV_ADDR` reader - This will enable an auto-increment to the DMA Channel Device Address.\n 1: Increment the DMA Channel Device Address by DMA Channel Control:Transfer Size after every Data Packet transfer\n 0: Do nothing"]
pub type INC_DEV_ADDR_R = crate::BitReader<bool>;
#[doc = "Field `INC_DEV_ADDR` writer - This will enable an auto-increment to the DMA Channel Device Address.\n 1: Increment the DMA Channel Device Address by DMA Channel Control:Transfer Size after every Data Packet transfer\n 0: Do nothing"]
pub type INC_DEV_ADDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - This is used to lock the arbitration of the Channel Arbiter on this channel once this channel is granted. Once this is locked, it will remain on the arbiter until it has completed it transfer (either the Transfer Aborted, Transfer Done or Transfer Terminated conditions)."]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - This is used to lock the arbitration of the Channel Arbiter on this channel once this channel is granted. Once this is locked, it will remain on the arbiter until it has completed it transfer (either the Transfer Aborted, Transfer Done or Transfer Terminated conditions)."]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DIS_HW_FLOW_CTRL` reader - This will Disable the Hardware Flow Control. When disabled, any DMA Master device attempting to communicate to the DMA over the DMA Flow Control Interface (Ports: dma_req, dma_term, and dma_done) will be ignored. This should be set before using the DMA channel in Firmware Flow Control mode."]
pub type DIS_HW_FLOW_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `DIS_HW_FLOW_CTRL` writer - This will Disable the Hardware Flow Control. When disabled, any DMA Master device attempting to communicate to the DMA over the DMA Flow Control Interface (Ports: dma_req, dma_term, and dma_done) will be ignored. This should be set before using the DMA channel in Firmware Flow Control mode."]
pub type DIS_HW_FLOW_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TRANS_SIZE` reader - This is the transfer size in Bytes of each Data Packet transfer.\n Note: The transfer size must be a legal AMBA transfer size. Valid sizes are 1, 2 and 4 Bytes."]
pub type TRANS_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRANS_SIZE` writer - This is the transfer size in Bytes of each Data Packet transfer.\n Note: The transfer size must be a legal AMBA transfer size. Valid sizes are 1, 2 and 4 Bytes."]
pub type TRANS_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `TRANS_GO` reader - This is used for the Firmware Flow Control DMA transfer."]
pub type TRANS_GO_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_GO` writer - This is used for the Firmware Flow Control DMA transfer."]
pub type TRANS_GO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TRANS_ABORT` reader - This is used to abort the current transfer on this DMA Channel. The aborted transfer will be forced to terminate immediately."]
pub type TRANS_ABORT_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_ABORT` writer - This is used to abort the current transfer on this DMA Channel. The aborted transfer will be forced to terminate immediately."]
pub type TRANS_ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This is a control field. Note: This bit only applies to Hardware Flow Control mode.\n 1= This channel is enabled and will service transfer requests\n 0=This channel is disabled. All transfer requests are ignored."]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is a status field.\n 1= There is a transfer request from the Master Device\n 0= There is no transfer request from the Master Device"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is a status signal. It is only valid while DMA Channel Control: Run is Enabled. \n This is the inverse of the DMA Channel Control: Busy field, except this is qualified with the DMA Channel Control:Run field.\n 1=Channel is done\n 0=Channel is not done or it is OFF"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - This is a status signal. The status decode is listed in priority order with the highest priority first.\n 3: Error detected by the DMA\n 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term\n 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address\n 0: DMA Channel Control:Run is Disabled (0x0)"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - This is a status signal.\n 1=The DMA Channel is busy (FSM is not IDLE)\n 0=The DMA Channel is not busy (FSM is IDLE)"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - This determines the direction of the DMA Transfer.\n 1=Data Packet Read from Memory Start Address followed by Data Packet Write to Device Address\n 0=Data Packet Read from Device Address followed by Data Packet Write to Memory Start Address"]
    #[inline(always)]
    pub fn tx_dir(&self) -> TX_DIR_R {
        TX_DIR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - This is the device that is connected to this channel as its Hardware Flow Control master.\n The Flow Control Interface is a bus with each master concatenated onto it.\n This selects which bus index of the concatenated Flow Control Interface bus is targeted towards this channel.\n The Flow Control Interface Port list is dma_req, dma_term, and dma_done."]
    #[inline(always)]
    pub fn hw_flow_ctrl_dev(&self) -> HW_FLOW_CTRL_DEV_R {
        HW_FLOW_CTRL_DEV_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - This will enable an auto-increment to the DMA Channel Memory Address.\n 1=Increment the DMA Channel Memory Address by DMA Channel Control:Transfer Size after every Data Packet transfer\n 0=Do nothing"]
    #[inline(always)]
    pub fn inc_mem_addr(&self) -> INC_MEM_ADDR_R {
        INC_MEM_ADDR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This will enable an auto-increment to the DMA Channel Device Address.\n 1: Increment the DMA Channel Device Address by DMA Channel Control:Transfer Size after every Data Packet transfer\n 0: Do nothing"]
    #[inline(always)]
    pub fn inc_dev_addr(&self) -> INC_DEV_ADDR_R {
        INC_DEV_ADDR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This is used to lock the arbitration of the Channel Arbiter on this channel once this channel is granted. Once this is locked, it will remain on the arbiter until it has completed it transfer (either the Transfer Aborted, Transfer Done or Transfer Terminated conditions)."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This will Disable the Hardware Flow Control. When disabled, any DMA Master device attempting to communicate to the DMA over the DMA Flow Control Interface (Ports: dma_req, dma_term, and dma_done) will be ignored. This should be set before using the DMA channel in Firmware Flow Control mode."]
    #[inline(always)]
    pub fn dis_hw_flow_ctrl(&self) -> DIS_HW_FLOW_CTRL_R {
        DIS_HW_FLOW_CTRL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - This is the transfer size in Bytes of each Data Packet transfer.\n Note: The transfer size must be a legal AMBA transfer size. Valid sizes are 1, 2 and 4 Bytes."]
    #[inline(always)]
    pub fn trans_size(&self) -> TRANS_SIZE_R {
        TRANS_SIZE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - This is used for the Firmware Flow Control DMA transfer."]
    #[inline(always)]
    pub fn trans_go(&self) -> TRANS_GO_R {
        TRANS_GO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This is used to abort the current transfer on this DMA Channel. The aborted transfer will be forced to terminate immediately."]
    #[inline(always)]
    pub fn trans_abort(&self) -> TRANS_ABORT_R {
        TRANS_ABORT_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This is a control field. Note: This bit only applies to Hardware Flow Control mode.\n 1= This channel is enabled and will service transfer requests\n 0=This channel is disabled. All transfer requests are ignored."]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W<0> {
        RUN_W::new(self)
    }
    #[doc = "Bit 1 - This is a status field.\n 1= There is a transfer request from the Master Device\n 0= There is no transfer request from the Master Device"]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W<1> {
        REQ_W::new(self)
    }
    #[doc = "Bit 2 - This is a status signal. It is only valid while DMA Channel Control: Run is Enabled. \n This is the inverse of the DMA Channel Control: Busy field, except this is qualified with the DMA Channel Control:Run field.\n 1=Channel is done\n 0=Channel is not done or it is OFF"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<2> {
        DONE_W::new(self)
    }
    #[doc = "Bits 3:4 - This is a status signal. The status decode is listed in priority order with the highest priority first.\n 3: Error detected by the DMA\n 2: The DMA Channel is externally done, in that the Device has terminated the transfer over the Hardware Flow Control through the Port dma_term\n 1: The DMA Channel is locally done, in that Memory Start Address equals Memory End Address\n 0: DMA Channel Control:Run is Disabled (0x0)"]
    #[inline(always)]
    pub fn sts(&mut self) -> STS_W<3> {
        STS_W::new(self)
    }
    #[doc = "Bit 5 - This is a status signal.\n 1=The DMA Channel is busy (FSM is not IDLE)\n 0=The DMA Channel is not busy (FSM is IDLE)"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<5> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 8 - This determines the direction of the DMA Transfer.\n 1=Data Packet Read from Memory Start Address followed by Data Packet Write to Device Address\n 0=Data Packet Read from Device Address followed by Data Packet Write to Memory Start Address"]
    #[inline(always)]
    pub fn tx_dir(&mut self) -> TX_DIR_W<8> {
        TX_DIR_W::new(self)
    }
    #[doc = "Bits 9:15 - This is the device that is connected to this channel as its Hardware Flow Control master.\n The Flow Control Interface is a bus with each master concatenated onto it.\n This selects which bus index of the concatenated Flow Control Interface bus is targeted towards this channel.\n The Flow Control Interface Port list is dma_req, dma_term, and dma_done."]
    #[inline(always)]
    pub fn hw_flow_ctrl_dev(&mut self) -> HW_FLOW_CTRL_DEV_W<9> {
        HW_FLOW_CTRL_DEV_W::new(self)
    }
    #[doc = "Bit 16 - This will enable an auto-increment to the DMA Channel Memory Address.\n 1=Increment the DMA Channel Memory Address by DMA Channel Control:Transfer Size after every Data Packet transfer\n 0=Do nothing"]
    #[inline(always)]
    pub fn inc_mem_addr(&mut self) -> INC_MEM_ADDR_W<16> {
        INC_MEM_ADDR_W::new(self)
    }
    #[doc = "Bit 17 - This will enable an auto-increment to the DMA Channel Device Address.\n 1: Increment the DMA Channel Device Address by DMA Channel Control:Transfer Size after every Data Packet transfer\n 0: Do nothing"]
    #[inline(always)]
    pub fn inc_dev_addr(&mut self) -> INC_DEV_ADDR_W<17> {
        INC_DEV_ADDR_W::new(self)
    }
    #[doc = "Bit 18 - This is used to lock the arbitration of the Channel Arbiter on this channel once this channel is granted. Once this is locked, it will remain on the arbiter until it has completed it transfer (either the Transfer Aborted, Transfer Done or Transfer Terminated conditions)."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<18> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 19 - This will Disable the Hardware Flow Control. When disabled, any DMA Master device attempting to communicate to the DMA over the DMA Flow Control Interface (Ports: dma_req, dma_term, and dma_done) will be ignored. This should be set before using the DMA channel in Firmware Flow Control mode."]
    #[inline(always)]
    pub fn dis_hw_flow_ctrl(&mut self) -> DIS_HW_FLOW_CTRL_W<19> {
        DIS_HW_FLOW_CTRL_W::new(self)
    }
    #[doc = "Bits 20:22 - This is the transfer size in Bytes of each Data Packet transfer.\n Note: The transfer size must be a legal AMBA transfer size. Valid sizes are 1, 2 and 4 Bytes."]
    #[inline(always)]
    pub fn trans_size(&mut self) -> TRANS_SIZE_W<20> {
        TRANS_SIZE_W::new(self)
    }
    #[doc = "Bit 24 - This is used for the Firmware Flow Control DMA transfer."]
    #[inline(always)]
    pub fn trans_go(&mut self) -> TRANS_GO_W<24> {
        TRANS_GO_W::new(self)
    }
    #[doc = "Bit 25 - This is used to abort the current transfer on this DMA Channel. The aborted transfer will be forced to terminate immediately."]
    #[inline(always)]
    pub fn trans_abort(&mut self) -> TRANS_ABORT_W<25> {
        TRANS_ABORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel N Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
