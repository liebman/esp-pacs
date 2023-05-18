#[doc = "Register `MAIN_BUF0_LOW` reader"]
pub struct R(crate::R<MAIN_BUF0_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAIN_BUF0_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAIN_BUF0_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAIN_BUF0_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAIN_TIMER_BUF0_LOW` reader - need_des"]
pub type MAIN_TIMER_BUF0_LOW_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn main_timer_buf0_low(&self) -> MAIN_TIMER_BUF0_LOW_R {
        MAIN_TIMER_BUF0_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_TIMER::MAIN_BUF0_LOW")
            .field(
                "main_timer_buf0_low",
                &format_args!("{}", self.main_timer_buf0_low().bits()),
            )
            .finish()
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [main_buf0_low](index.html) module"]
pub struct MAIN_BUF0_LOW_SPEC;
impl crate::RegisterSpec for MAIN_BUF0_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [main_buf0_low::R](R) reader structure"]
impl crate::Readable for MAIN_BUF0_LOW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAIN_BUF0_LOW to value 0"]
impl crate::Resettable for MAIN_BUF0_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
