#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DONE_INT_ST` reader - x"]
pub type DONE_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `ERR_INT_ST` reader - x"]
pub type ERR_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - x"]
    #[inline(always)]
    pub fn done_int_st(&self) -> DONE_INT_ST_R {
        DONE_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - x"]
    #[inline(always)]
    pub fn err_int_st(&self) -> ERR_INT_ST_R {
        ERR_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_BACKUP::INT_ST")
            .field("done_int_st", &format_args!("{}", self.done_int_st().bit()))
            .field("err_int_st", &format_args!("{}", self.err_int_st().bit()))
            .finish()
    }
}
#[doc = "x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
