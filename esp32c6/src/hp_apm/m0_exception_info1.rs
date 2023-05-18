#[doc = "Register `M0_EXCEPTION_INFO1` reader"]
pub struct R(crate::R<M0_EXCEPTION_INFO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M0_EXCEPTION_INFO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M0_EXCEPTION_INFO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M0_EXCEPTION_INFO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M0_EXCEPTION_ADDR` reader - Exception addr"]
pub type M0_EXCEPTION_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Exception addr"]
    #[inline(always)]
    pub fn m0_exception_addr(&self) -> M0_EXCEPTION_ADDR_R {
        M0_EXCEPTION_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_APM::M0_EXCEPTION_INFO1")
            .field(
                "m0_exception_addr",
                &format_args!("{}", self.m0_exception_addr().bits()),
            )
            .finish()
    }
}
#[doc = "M0 exception_info1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m0_exception_info1](index.html) module"]
pub struct M0_EXCEPTION_INFO1_SPEC;
impl crate::RegisterSpec for M0_EXCEPTION_INFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m0_exception_info1::R](R) reader structure"]
impl crate::Readable for M0_EXCEPTION_INFO1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M0_EXCEPTION_INFO1 to value 0"]
impl crate::Resettable for M0_EXCEPTION_INFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
