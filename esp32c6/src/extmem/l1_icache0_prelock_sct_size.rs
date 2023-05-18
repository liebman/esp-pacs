#[doc = "Register `L1_ICACHE0_PRELOCK_SCT_SIZE` reader"]
pub struct R(crate::R<L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_ICACHE0_PRELOCK_SCT0_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT0_ADDR_REG"]
pub type L1_ICACHE0_PRELOCK_SCT0_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `L1_ICACHE0_PRELOCK_SCT1_SIZE` reader - Those bits are used to configure the size of the second section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT1_ADDR_REG"]
pub type L1_ICACHE0_PRELOCK_SCT1_SIZE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn l1_icache0_prelock_sct0_size(&self) -> L1_ICACHE0_PRELOCK_SCT0_SIZE_R {
        L1_ICACHE0_PRELOCK_SCT0_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Those bits are used to configure the size of the second section of prelock on L1-ICache0, which should be used together with L1_ICACHE0_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn l1_icache0_prelock_sct1_size(&self) -> L1_ICACHE0_PRELOCK_SCT1_SIZE_R {
        L1_ICACHE0_PRELOCK_SCT1_SIZE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM::L1_ICACHE0_PRELOCK_SCT_SIZE")
            .field(
                "l1_icache0_prelock_sct0_size",
                &format_args!("{}", self.l1_icache0_prelock_sct0_size().bits()),
            )
            .field(
                "l1_icache0_prelock_sct1_size",
                &format_args!("{}", self.l1_icache0_prelock_sct1_size().bits()),
            )
            .finish()
    }
}
#[doc = "L1 instruction Cache 0 prelock section size configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_icache0_prelock_sct_size](index.html) module"]
pub struct L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC;
impl crate::RegisterSpec for L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_icache0_prelock_sct_size::R](R) reader structure"]
impl crate::Readable for L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_ICACHE0_PRELOCK_SCT_SIZE to value 0x3fff_3fff"]
impl crate::Resettable for L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_3fff;
}
