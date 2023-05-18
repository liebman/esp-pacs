#[doc = "Register `CORE_1_DRAM0_EXCEPTION_MONITOR_1` reader"]
pub struct R(crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_DRAM0_RECORDING_BYTEEN_0` reader - The first dram0's byteen status when trigger DRAM busy interrupt"]
pub type CORE_1_DRAM0_RECORDING_BYTEEN_0_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - The first dram0's byteen status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_1_dram0_recording_byteen_0(&self) -> CORE_1_DRAM0_RECORDING_BYTEEN_0_R {
        CORE_1_DRAM0_RECORDING_BYTEEN_0_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASSIST_DEBUG::CORE_1_DRAM0_EXCEPTION_MONITOR_1")
            .field(
                "core_1_dram0_recording_byteen_0",
                &format_args!("{}", self.core_1_dram0_recording_byteen_0().bits()),
            )
            .finish()
    }
}
#[doc = "Core1 bus busy status regsiter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_dram0_exception_monitor_1](index.html) module"]
pub struct CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_dram0_exception_monitor_1::R](R) reader structure"]
impl crate::Readable for CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_1 to value 0"]
impl crate::Resettable for CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
