#[doc = "Register `AESH_BSWAP_CTRL` reader"]
pub struct R(crate::R<AESH_BSWAP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESH_BSWAP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESH_BSWAP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESH_BSWAP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESH_BSWAP_CTRL` writer"]
pub struct W(crate::W<AESH_BSWAP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESH_BSWAP_CTRL_SPEC>;
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
impl From<crate::W<AESH_BSWAP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESH_BSWAP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP_BYTE_SWAP_EN` reader - Used to enable byte swap on a DWORD during AHB read from AES / HASH block: 1=Enable; 0=Disable."]
pub type IP_BYTE_SWAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `IP_BYTE_SWAP_EN` writer - Used to enable byte swap on a DWORD during AHB read from AES / HASH block: 1=Enable; 0=Disable."]
pub type IP_BYTE_SWAP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AESH_BSWAP_CTRL_SPEC, bool, O>;
#[doc = "Field `OP_BYTE_SWAP_EN` reader - Used to enable byte swap on a DWORD during AHB write from AES / HASH block: 1=Enable; 0=Disable."]
pub type OP_BYTE_SWAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `OP_BYTE_SWAP_EN` writer - Used to enable byte swap on a DWORD during AHB write from AES / HASH block: 1=Enable; 0=Disable."]
pub type OP_BYTE_SWAP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AESH_BSWAP_CTRL_SPEC, bool, O>;
#[doc = "Field `IP_BLK_SWAP_EN` reader - Used to enable word swap on a DWORD during AHB read from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
pub type IP_BLK_SWAP_EN_R = crate::FieldReader<u8, IP_BLK_SWAP_ENSELECT_A>;
#[doc = "Used to enable word swap on a DWORD during AHB read from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IP_BLK_SWAP_ENSELECT_A {
    #[doc = "4: 4=Swap 32-bit doublewords in 128-byte blocks"]
    SWAP_128_BYTE_BLOCK = 4,
    #[doc = "3: 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C,\n 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,..."]
    SWAP_64_BYTE_BLOCK = 3,
    #[doc = "2: 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,..."]
    SWAP_16_BYTE_BLOCK = 2,
    #[doc = "1: 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4,\n 0x0, 0xC, 0x8, ...\n"]
    SWAP_8_BYTE_BLOCK = 1,
    #[doc = "0: 0=Disable."]
    DIS = 0,
}
impl From<IP_BLK_SWAP_ENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IP_BLK_SWAP_ENSELECT_A) -> Self {
        variant as _
    }
}
impl IP_BLK_SWAP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IP_BLK_SWAP_ENSELECT_A> {
        match self.bits {
            4 => Some(IP_BLK_SWAP_ENSELECT_A::SWAP_128_BYTE_BLOCK),
            3 => Some(IP_BLK_SWAP_ENSELECT_A::SWAP_64_BYTE_BLOCK),
            2 => Some(IP_BLK_SWAP_ENSELECT_A::SWAP_16_BYTE_BLOCK),
            1 => Some(IP_BLK_SWAP_ENSELECT_A::SWAP_8_BYTE_BLOCK),
            0 => Some(IP_BLK_SWAP_ENSELECT_A::DIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SWAP_128_BYTE_BLOCK`"]
    #[inline(always)]
    pub fn is_swap_128_byte_block(&self) -> bool {
        *self == IP_BLK_SWAP_ENSELECT_A::SWAP_128_BYTE_BLOCK
    }
    #[doc = "Checks if the value of the field is `SWAP_64_BYTE_BLOCK`"]
    #[inline(always)]
    pub fn is_swap_64_byte_block(&self) -> bool {
        *self == IP_BLK_SWAP_ENSELECT_A::SWAP_64_BYTE_BLOCK
    }
    #[doc = "Checks if the value of the field is `SWAP_16_BYTE_BLOCK`"]
    #[inline(always)]
    pub fn is_swap_16_byte_block(&self) -> bool {
        *self == IP_BLK_SWAP_ENSELECT_A::SWAP_16_BYTE_BLOCK
    }
    #[doc = "Checks if the value of the field is `SWAP_8_BYTE_BLOCK`"]
    #[inline(always)]
    pub fn is_swap_8_byte_block(&self) -> bool {
        *self == IP_BLK_SWAP_ENSELECT_A::SWAP_8_BYTE_BLOCK
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IP_BLK_SWAP_ENSELECT_A::DIS
    }
}
#[doc = "Field `IP_BLK_SWAP_EN` writer - Used to enable word swap on a DWORD during AHB read from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
pub type IP_BLK_SWAP_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AESH_BSWAP_CTRL_SPEC, u8, IP_BLK_SWAP_ENSELECT_A, 3, O>;
impl<'a, const O: u8> IP_BLK_SWAP_EN_W<'a, O> {
    #[doc = "4=Swap 32-bit doublewords in 128-byte blocks"]
    #[inline(always)]
    pub fn swap_128_byte_block(self) -> &'a mut W {
        self.variant(IP_BLK_SWAP_ENSELECT_A::SWAP_128_BYTE_BLOCK)
    }
    #[doc = "3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C,\n 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,..."]
    #[inline(always)]
    pub fn swap_64_byte_block(self) -> &'a mut W {
        self.variant(IP_BLK_SWAP_ENSELECT_A::SWAP_64_BYTE_BLOCK)
    }
    #[doc = "2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,..."]
    #[inline(always)]
    pub fn swap_16_byte_block(self) -> &'a mut W {
        self.variant(IP_BLK_SWAP_ENSELECT_A::SWAP_16_BYTE_BLOCK)
    }
    #[doc = "1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4,\n 0x0, 0xC, 0x8, ...\n"]
    #[inline(always)]
    pub fn swap_8_byte_block(self) -> &'a mut W {
        self.variant(IP_BLK_SWAP_ENSELECT_A::SWAP_8_BYTE_BLOCK)
    }
    #[doc = "0=Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IP_BLK_SWAP_ENSELECT_A::DIS)
    }
}
#[doc = "Field `OP_BLK_SWAP_EN` reader - Used to enable word swap on a DWORD during AHB write from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
pub type OP_BLK_SWAP_EN_R = crate::FieldReader<u8, OP_BLK_SWAP_ENSELECT_A>;
#[doc = "Used to enable word swap on a DWORD during AHB write from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OP_BLK_SWAP_ENSELECT_A {
    #[doc = "4: 4=Swap 32-bit doublewords in 128-byte blocks"]
    SWAP_128_BYTE_BLOCK = 4,
    #[doc = "3: 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C,\n 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,..."]
    SWAP_64_BYTE_BLOCK = 3,
    #[doc = "2: 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,..."]
    SWAP_16_BYTE_BLOCK = 2,
    #[doc = "1: 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4,\n 0x0, 0xC, 0x8, ...\n"]
    SWAP_8_BYTE_BLOCK = 1,
    #[doc = "0: 0=Disable."]
    DIS = 0,
}
impl From<OP_BLK_SWAP_ENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: OP_BLK_SWAP_ENSELECT_A) -> Self {
        variant as _
    }
}
impl OP_BLK_SWAP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OP_BLK_SWAP_ENSELECT_A> {
        match self.bits {
            4 => Some(OP_BLK_SWAP_ENSELECT_A::SWAP_128_BYTE_BLOCK),
            3 => Some(OP_BLK_SWAP_ENSELECT_A::SWAP_64_BYTE_BLOCK),
            2 => Some(OP_BLK_SWAP_ENSELECT_A::SWAP_16_BYTE_BLOCK),
            1 => Some(OP_BLK_SWAP_ENSELECT_A::SWAP_8_BYTE_BLOCK),
            0 => Some(OP_BLK_SWAP_ENSELECT_A::DIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SWAP_128_BYTE_BLOCK`"]
    #[inline(always)]
    pub fn is_swap_128_byte_block(&self) -> bool {
        *self == OP_BLK_SWAP_ENSELECT_A::SWAP_128_BYTE_BLOCK
    }
    #[doc = "Checks if the value of the field is `SWAP_64_BYTE_BLOCK`"]
    #[inline(always)]
    pub fn is_swap_64_byte_block(&self) -> bool {
        *self == OP_BLK_SWAP_ENSELECT_A::SWAP_64_BYTE_BLOCK
    }
    #[doc = "Checks if the value of the field is `SWAP_16_BYTE_BLOCK`"]
    #[inline(always)]
    pub fn is_swap_16_byte_block(&self) -> bool {
        *self == OP_BLK_SWAP_ENSELECT_A::SWAP_16_BYTE_BLOCK
    }
    #[doc = "Checks if the value of the field is `SWAP_8_BYTE_BLOCK`"]
    #[inline(always)]
    pub fn is_swap_8_byte_block(&self) -> bool {
        *self == OP_BLK_SWAP_ENSELECT_A::SWAP_8_BYTE_BLOCK
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == OP_BLK_SWAP_ENSELECT_A::DIS
    }
}
#[doc = "Field `OP_BLK_SWAP_EN` writer - Used to enable word swap on a DWORD during AHB write from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
pub type OP_BLK_SWAP_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AESH_BSWAP_CTRL_SPEC, u8, OP_BLK_SWAP_ENSELECT_A, 3, O>;
impl<'a, const O: u8> OP_BLK_SWAP_EN_W<'a, O> {
    #[doc = "4=Swap 32-bit doublewords in 128-byte blocks"]
    #[inline(always)]
    pub fn swap_128_byte_block(self) -> &'a mut W {
        self.variant(OP_BLK_SWAP_ENSELECT_A::SWAP_128_BYTE_BLOCK)
    }
    #[doc = "3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C,\n 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,..."]
    #[inline(always)]
    pub fn swap_64_byte_block(self) -> &'a mut W {
        self.variant(OP_BLK_SWAP_ENSELECT_A::SWAP_64_BYTE_BLOCK)
    }
    #[doc = "2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,..."]
    #[inline(always)]
    pub fn swap_16_byte_block(self) -> &'a mut W {
        self.variant(OP_BLK_SWAP_ENSELECT_A::SWAP_16_BYTE_BLOCK)
    }
    #[doc = "1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4,\n 0x0, 0xC, 0x8, ...\n"]
    #[inline(always)]
    pub fn swap_8_byte_block(self) -> &'a mut W {
        self.variant(OP_BLK_SWAP_ENSELECT_A::SWAP_8_BYTE_BLOCK)
    }
    #[doc = "0=Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(OP_BLK_SWAP_ENSELECT_A::DIS)
    }
}
impl R {
    #[doc = "Bit 0 - Used to enable byte swap on a DWORD during AHB read from AES / HASH block: 1=Enable; 0=Disable."]
    #[inline(always)]
    pub fn ip_byte_swap_en(&self) -> IP_BYTE_SWAP_EN_R {
        IP_BYTE_SWAP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Used to enable byte swap on a DWORD during AHB write from AES / HASH block: 1=Enable; 0=Disable."]
    #[inline(always)]
    pub fn op_byte_swap_en(&self) -> OP_BYTE_SWAP_EN_R {
        OP_BYTE_SWAP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Used to enable word swap on a DWORD during AHB read from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
    #[inline(always)]
    pub fn ip_blk_swap_en(&self) -> IP_BLK_SWAP_EN_R {
        IP_BLK_SWAP_EN_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Used to enable word swap on a DWORD during AHB write from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
    #[inline(always)]
    pub fn op_blk_swap_en(&self) -> OP_BLK_SWAP_EN_R {
        OP_BLK_SWAP_EN_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Used to enable byte swap on a DWORD during AHB read from AES / HASH block: 1=Enable; 0=Disable."]
    #[inline(always)]
    pub fn ip_byte_swap_en(&mut self) -> IP_BYTE_SWAP_EN_W<0> {
        IP_BYTE_SWAP_EN_W::new(self)
    }
    #[doc = "Bit 1 - Used to enable byte swap on a DWORD during AHB write from AES / HASH block: 1=Enable; 0=Disable."]
    #[inline(always)]
    pub fn op_byte_swap_en(&mut self) -> OP_BYTE_SWAP_EN_W<1> {
        OP_BYTE_SWAP_EN_W::new(self)
    }
    #[doc = "Bits 2:4 - Used to enable word swap on a DWORD during AHB read from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
    #[inline(always)]
    pub fn ip_blk_swap_en(&mut self) -> IP_BLK_SWAP_EN_W<2> {
        IP_BLK_SWAP_EN_W::new(self)
    }
    #[doc = "Bits 5:7 - Used to enable word swap on a DWORD during AHB write from AES / HASH block 4=Swap 32-bit doublewords in 128-byte blocks 3=Swap doublewords in 64-byte blocks. Useful for SHA-256. Bus references issued in the order 0x3C, 0x38, 0x34, 0x30, 0x2C, 0x28, 0x24, 0x20, 0x1C, 0x18, 0x14, 0x10, 0xC, 0x8, 0x4, 0x0,... 2=Swap doublewords in 16-byte blocks. Useful for AES. Bus references issued in the order 0xC, 0x8, 0x4, 0x0, 0x1C, 0x18,... 1=Swap doublewords in 8-byte blocks. Useful for SHA-512, which works on 64-bit words. Bus references issued in the order 0x4, 0x0, 0xC, 0x8, ... 0=Disable."]
    #[inline(always)]
    pub fn op_blk_swap_en(&mut self) -> OP_BLK_SWAP_EN_W<5> {
        OP_BLK_SWAP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES HASH Byte Swap Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesh_bswap_ctrl](index.html) module"]
pub struct AESH_BSWAP_CTRL_SPEC;
impl crate::RegisterSpec for AESH_BSWAP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesh_bswap_ctrl::R](R) reader structure"]
impl crate::Readable for AESH_BSWAP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesh_bswap_ctrl::W](W) writer structure"]
impl crate::Writable for AESH_BSWAP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESH_BSWAP_CTRL to value 0"]
impl crate::Resettable for AESH_BSWAP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
