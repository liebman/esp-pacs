#[doc = "Register `M0_STATUS` reader"]
pub struct R(crate::R<M0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M0_EXCEPTION_STATUS` reader - Exception status"]
pub type M0_EXCEPTION_STATUS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Exception status"]
    #[inline(always)]
    pub fn m0_exception_status(&self) -> M0_EXCEPTION_STATUS_R {
        M0_EXCEPTION_STATUS_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_APM::M0_STATUS")
            .field(
                "m0_exception_status",
                &format_args!("{}", self.m0_exception_status().bits()),
            )
            .finish()
    }
}
#[doc = "M0 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m0_status](index.html) module"]
pub struct M0_STATUS_SPEC;
impl crate::RegisterSpec for M0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m0_status::R](R) reader structure"]
impl crate::Readable for M0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M0_STATUS to value 0"]
impl crate::Resettable for M0_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
