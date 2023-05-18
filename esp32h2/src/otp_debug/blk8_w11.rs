#[doc = "Register `BLK8_W11` reader"]
pub struct R(crate::R<BLK8_W11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK8_W11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK8_W11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK8_W11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK8_W11` reader - Otp block8 word11 data."]
pub type BLOCK8_W11_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block8 word11 data."]
    #[inline(always)]
    pub fn block8_w11(&self) -> BLOCK8_W11_R {
        BLOCK8_W11_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_DEBUG::BLK8_W11")
            .field("block8_w11", &format_args!("{}", self.block8_w11().bits()))
            .finish()
    }
}
#[doc = "Otp debuger block8 data register11.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk8_w11](index.html) module"]
pub struct BLK8_W11_SPEC;
impl crate::RegisterSpec for BLK8_W11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk8_w11::R](R) reader structure"]
impl crate::Readable for BLK8_W11_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK8_W11 to value 0"]
impl crate::Resettable for BLK8_W11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
