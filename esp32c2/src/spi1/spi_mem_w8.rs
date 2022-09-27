#[doc = "Register `SPI_MEM_W8` reader"]
pub struct R(crate::R<SPI_MEM_W8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_W8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_W8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_W8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_W8` writer"]
pub struct W(crate::W<SPI_MEM_W8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_W8_SPEC>;
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
impl From<crate::W<SPI_MEM_W8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_W8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_BUF8` reader - data buffer"]
pub type SPI_MEM_BUF8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPI_MEM_BUF8` writer - data buffer"]
pub type SPI_MEM_BUF8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_MEM_W8_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_mem_buf8(&self) -> SPI_MEM_BUF8_R {
        SPI_MEM_BUF8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_mem_buf8(&mut self) -> SPI_MEM_BUF8_W<0> {
        SPI_MEM_BUF8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 memory data buffer8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_w8](index.html) module"]
pub struct SPI_MEM_W8_SPEC;
impl crate::RegisterSpec for SPI_MEM_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_w8::R](R) reader structure"]
impl crate::Readable for SPI_MEM_W8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_w8::W](W) writer structure"]
impl crate::Writable for SPI_MEM_W8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_W8 to value 0"]
impl crate::Resettable for SPI_MEM_W8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}