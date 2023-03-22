#[doc = "Register `SRAM_ACE2_SIZE` reader"]
pub struct R(crate::R<SRAM_ACE2_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_ACE2_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_ACE2_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_ACE2_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_ACE2_SIZE` writer"]
pub struct W(crate::W<SRAM_ACE2_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_ACE2_SIZE_SPEC>;
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
impl From<crate::W<SRAM_ACE2_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_ACE2_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_ACE2_SIZE` reader - "]
pub type SRAM_ACE2_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SRAM_ACE2_SIZE` writer - "]
pub type SRAM_ACE2_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRAM_ACE2_SIZE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sram_ace2_size(&self) -> SRAM_ACE2_SIZE_R {
        SRAM_ACE2_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ace2_size(&mut self) -> SRAM_ACE2_SIZE_W<0> {
        SRAM_ACE2_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_ace2_size](index.html) module"]
pub struct SRAM_ACE2_SIZE_SPEC;
impl crate::RegisterSpec for SRAM_ACE2_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_ace2_size::R](R) reader structure"]
impl crate::Readable for SRAM_ACE2_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_ace2_size::W](W) writer structure"]
impl crate::Writable for SRAM_ACE2_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_ACE2_SIZE to value 0x1000"]
impl crate::Resettable for SRAM_ACE2_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
