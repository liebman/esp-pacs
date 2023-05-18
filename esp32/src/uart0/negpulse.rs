#[doc = "Register `NEGPULSE` reader"]
pub struct R(crate::R<NEGPULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NEGPULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NEGPULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NEGPULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NEGEDGE_MIN_CNT` reader - This register stores the count of rxd negedge edge. it is used in boudrate-detect process."]
pub type NEGEDGE_MIN_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - This register stores the count of rxd negedge edge. it is used in boudrate-detect process."]
    #[inline(always)]
    pub fn negedge_min_cnt(&self) -> NEGEDGE_MIN_CNT_R {
        NEGEDGE_MIN_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0::NEGPULSE")
            .field(
                "negedge_min_cnt",
                &format_args!("{}", self.negedge_min_cnt().bits()),
            )
            .finish()
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [negpulse](index.html) module"]
pub struct NEGPULSE_SPEC;
impl crate::RegisterSpec for NEGPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [negpulse::R](R) reader structure"]
impl crate::Readable for NEGPULSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NEGPULSE to value 0x000f_ffff"]
impl crate::Resettable for NEGPULSE_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_ffff;
}
