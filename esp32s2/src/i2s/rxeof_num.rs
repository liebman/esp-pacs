#[doc = "Register `RXEOF_NUM` reader"]
pub struct R(crate::R<RXEOF_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXEOF_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXEOF_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXEOF_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXEOF_NUM` writer"]
pub struct W(crate::W<RXEOF_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXEOF_NUM_SPEC>;
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
impl From<crate::W<RXEOF_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXEOF_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_EOF_NUM` reader - The length of data to be received. It will trigger I2S_IN_SUC_EOF_INT."]
pub type RX_EOF_NUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RX_EOF_NUM` writer - The length of data to be received. It will trigger I2S_IN_SUC_EOF_INT."]
pub type RX_EOF_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXEOF_NUM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The length of data to be received. It will trigger I2S_IN_SUC_EOF_INT."]
    #[inline(always)]
    pub fn rx_eof_num(&self) -> RX_EOF_NUM_R {
        RX_EOF_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The length of data to be received. It will trigger I2S_IN_SUC_EOF_INT."]
    #[inline(always)]
    pub fn rx_eof_num(&mut self) -> RX_EOF_NUM_W<0> {
        RX_EOF_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S DMA RX EOF data length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxeof_num](index.html) module"]
pub struct RXEOF_NUM_SPEC;
impl crate::RegisterSpec for RXEOF_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxeof_num::R](R) reader structure"]
impl crate::Readable for RXEOF_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxeof_num::W](W) writer structure"]
impl crate::Writable for RXEOF_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXEOF_NUM to value 0x40"]
impl crate::Resettable for RXEOF_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}