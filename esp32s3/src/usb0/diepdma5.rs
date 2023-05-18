#[doc = "Register `DIEPDMA5` reader"]
pub struct R(crate::R<DIEPDMA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPDMA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPDMA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPDMA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPDMA5` writer"]
pub struct W(crate::W<DIEPDMA5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPDMA5_SPEC>;
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
impl From<crate::W<DIEPDMA5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPDMA5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_DMAADDR5` reader - "]
pub type D_DMAADDR5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `D_DMAADDR5` writer - "]
pub type D_DMAADDR5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIEPDMA5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmaaddr5(&self) -> D_DMAADDR5_R {
        D_DMAADDR5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB0::DIEPDMA5")
            .field("d_dmaaddr5", &format_args!("{}", self.d_dmaaddr5().bits()))
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn d_dmaaddr5(&mut self) -> D_DMAADDR5_W<0> {
        D_DMAADDR5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdma5](index.html) module"]
pub struct DIEPDMA5_SPEC;
impl crate::RegisterSpec for DIEPDMA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepdma5::R](R) reader structure"]
impl crate::Readable for DIEPDMA5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepdma5::W](W) writer structure"]
impl crate::Writable for DIEPDMA5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPDMA5 to value 0"]
impl crate::Resettable for DIEPDMA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
