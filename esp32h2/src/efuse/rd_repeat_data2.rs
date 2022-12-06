#[doc = "Register `RD_REPEAT_DATA2` reader"]
pub struct R(crate::R<RD_REPEAT_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY_PURPOSE_2` reader - Represents the purpose of Key2."]
pub type KEY_PURPOSE_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_PURPOSE_3` reader - Represents the purpose of Key3."]
pub type KEY_PURPOSE_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_PURPOSE_4` reader - Represents the purpose of Key4."]
pub type KEY_PURPOSE_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_PURPOSE_5` reader - Represents the purpose of Key5."]
pub type KEY_PURPOSE_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC_DPA_LEVEL` reader - Represents the spa secure level by configuring the clock random divide mode."]
pub type SEC_DPA_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECDSA_FORCE_USE_HARDWARE_K` reader - Represents whether hardware random number k is forced used in ESDCA. 1: force used. 0: not force used."]
pub type ECDSA_FORCE_USE_HARDWARE_K_R = crate::BitReader<bool>;
#[doc = "Field `CRYPT_DPA_ENABLE` reader - Represents whether anti-dpa attack is enabled. 1:enabled. 0: disabled."]
pub type CRYPT_DPA_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SECURE_BOOT_EN` reader - Represents whether secure boot is enabled or disabled. 1: enabled. 0: disabled."]
pub type SECURE_BOOT_EN_R = crate::BitReader<bool>;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE` reader - Represents whether revoking aggressive secure boot is enabled or disabled. 1: enabled. 0: disabled."]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::BitReader<bool>;
#[doc = "Field `RPT4_RESERVED2_0` reader - Reserved."]
pub type RPT4_RESERVED2_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_TPUW` reader - Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value."]
pub type FLASH_TPUW_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Represents the purpose of Key2."]
    #[inline(always)]
    pub fn key_purpose_2(&self) -> KEY_PURPOSE_2_R {
        KEY_PURPOSE_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Represents the purpose of Key3."]
    #[inline(always)]
    pub fn key_purpose_3(&self) -> KEY_PURPOSE_3_R {
        KEY_PURPOSE_3_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Represents the purpose of Key4."]
    #[inline(always)]
    pub fn key_purpose_4(&self) -> KEY_PURPOSE_4_R {
        KEY_PURPOSE_4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Represents the purpose of Key5."]
    #[inline(always)]
    pub fn key_purpose_5(&self) -> KEY_PURPOSE_5_R {
        KEY_PURPOSE_5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Represents the spa secure level by configuring the clock random divide mode."]
    #[inline(always)]
    pub fn sec_dpa_level(&self) -> SEC_DPA_LEVEL_R {
        SEC_DPA_LEVEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Represents whether hardware random number k is forced used in ESDCA. 1: force used. 0: not force used."]
    #[inline(always)]
    pub fn ecdsa_force_use_hardware_k(&self) -> ECDSA_FORCE_USE_HARDWARE_K_R {
        ECDSA_FORCE_USE_HARDWARE_K_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents whether anti-dpa attack is enabled. 1:enabled. 0: disabled."]
    #[inline(always)]
    pub fn crypt_dpa_enable(&self) -> CRYPT_DPA_ENABLE_R {
        CRYPT_DPA_ENABLE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents whether secure boot is enabled or disabled. 1: enabled. 0: disabled."]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents whether revoking aggressive secure boot is enabled or disabled. 1: enabled. 0: disabled."]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:27 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved2_0(&self) -> RPT4_RESERVED2_0_R {
        RPT4_RESERVED2_0_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31 - Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value."]
    #[inline(always)]
    pub fn flash_tpuw(&self) -> FLASH_TPUW_R {
        FLASH_TPUW_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "BLOCK0 data register 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data2](index.html) module"]
pub struct RD_REPEAT_DATA2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_data2::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA2 to value 0x000c_0000"]
impl crate::Resettable for RD_REPEAT_DATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0x000c_0000;
}