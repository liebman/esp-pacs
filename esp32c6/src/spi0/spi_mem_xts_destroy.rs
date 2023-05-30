#[doc = "Register `SPI_MEM_XTS_DESTROY` writer"]
pub struct W(crate::W<SPI_MEM_XTS_DESTROY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_XTS_DESTROY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_MEM_XTS_DESTROY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_XTS_DESTROY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_XTS_DESTROY` writer - Set this bit to destroy encrypted result. This action should be asserted only when manual encryption status is 3. After this action, manual encryption status will become 0."]
pub type SPI_XTS_DESTROY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_MEM_XTS_DESTROY_SPEC, bool, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_XTS_DESTROY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to destroy encrypted result. This action should be asserted only when manual encryption status is 3. After this action, manual encryption status will become 0."]
    #[inline(always)]
    #[must_use]
    pub fn spi_xts_destroy(&mut self) -> SPI_XTS_DESTROY_W<0> {
        SPI_XTS_DESTROY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manual Encryption physical address register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_xts_destroy](index.html) module"]
pub struct SPI_MEM_XTS_DESTROY_SPEC;
impl crate::RegisterSpec for SPI_MEM_XTS_DESTROY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_xts_destroy::W](W) writer structure"]
impl crate::Writable for SPI_MEM_XTS_DESTROY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_XTS_DESTROY to value 0"]
impl crate::Resettable for SPI_MEM_XTS_DESTROY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
