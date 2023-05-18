#[doc = "Register `CORE_1_RCD_SP` reader"]
pub struct R(crate::R<CORE_1_RCD_SP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_RCD_SP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_RCD_SP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_RCD_SP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_RCD_SP` reader - Core1_stack pointer"]
pub type CORE_1_RCD_SP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core1_stack pointer"]
    #[inline(always)]
    pub fn core_1_rcd_sp(&self) -> CORE_1_RCD_SP_R {
        CORE_1_RCD_SP_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASSIST_DEBUG::CORE_1_RCD_SP")
            .field(
                "core_1_rcd_sp",
                &format_args!("{}", self.core_1_rcd_sp().bits()),
            )
            .finish()
    }
}
#[doc = "Core1 pdebug status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_rcd_sp](index.html) module"]
pub struct CORE_1_RCD_SP_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_SP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_rcd_sp::R](R) reader structure"]
impl crate::Readable for CORE_1_RCD_SP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_RCD_SP to value 0"]
impl crate::Resettable for CORE_1_RCD_SP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
