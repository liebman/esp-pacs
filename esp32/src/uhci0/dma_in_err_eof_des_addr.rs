#[doc = "Register `DMA_IN_ERR_EOF_DES_ADDR` reader"]
pub struct R(crate::R<DMA_IN_ERR_EOF_DES_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_IN_ERR_EOF_DES_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_IN_ERR_EOF_DES_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_IN_ERR_EOF_DES_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_ERR_EOF_DES_ADDR` reader - This register stores the address of in link descriptor when there are some errors in this descriptor."]
pub type IN_ERR_EOF_DES_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of in link descriptor when there are some errors in this descriptor."]
    #[inline(always)]
    pub fn in_err_eof_des_addr(&self) -> IN_ERR_EOF_DES_ADDR_R {
        IN_ERR_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHCI0::DMA_IN_ERR_EOF_DES_ADDR")
            .field(
                "in_err_eof_des_addr",
                &format_args!("{}", self.in_err_eof_des_addr().bits()),
            )
            .finish()
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_in_err_eof_des_addr](index.html) module"]
pub struct DMA_IN_ERR_EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for DMA_IN_ERR_EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_in_err_eof_des_addr::R](R) reader structure"]
impl crate::Readable for DMA_IN_ERR_EOF_DES_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_IN_ERR_EOF_DES_ADDR to value 0"]
impl crate::Resettable for DMA_IN_ERR_EOF_DES_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
