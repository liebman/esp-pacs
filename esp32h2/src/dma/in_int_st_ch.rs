#[doc = "Register `IN_INT_ST_CH%s` reader"]
pub struct R(crate::R<IN_INT_ST_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_INT_ST_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_INT_ST_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_INT_ST_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_DONE` reader - The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_R = crate::BitReader<bool>;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_R = crate::BitReader<bool>;
#[doc = "Field `IN_ERR_EOF` reader - The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_ERR` reader - The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_EMPTY` reader - The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_OVF` reader - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_UDF` reader - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf(&self) -> INFIFO_OVF_R {
        INFIFO_OVF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf(&self) -> INFIFO_UDF_R {
        INFIFO_UDF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Masked interrupt of channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_int_st_ch](index.html) module"]
pub struct IN_INT_ST_CH_SPEC;
impl crate::RegisterSpec for IN_INT_ST_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_int_st_ch::R](R) reader structure"]
impl crate::Readable for IN_INT_ST_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_INT_ST_CH%s to value 0"]
impl crate::Resettable for IN_INT_ST_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}