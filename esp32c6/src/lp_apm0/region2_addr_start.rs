#[doc = "Register `REGION2_ADDR_START` reader"]
pub struct R(crate::R<REGION2_ADDR_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION2_ADDR_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION2_ADDR_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION2_ADDR_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION2_ADDR_START` writer"]
pub struct W(crate::W<REGION2_ADDR_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION2_ADDR_START_SPEC>;
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
impl From<crate::W<REGION2_ADDR_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION2_ADDR_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION2_ADDR_START` reader - Start address of region2"]
pub type REGION2_ADDR_START_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REGION2_ADDR_START` writer - Start address of region2"]
pub type REGION2_ADDR_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REGION2_ADDR_START_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start address of region2"]
    #[inline(always)]
    pub fn region2_addr_start(&self) -> REGION2_ADDR_START_R {
        REGION2_ADDR_START_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_APM0::REGION2_ADDR_START")
            .field(
                "region2_addr_start",
                &format_args!("{}", self.region2_addr_start().bits()),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of region2"]
    #[inline(always)]
    #[must_use]
    pub fn region2_addr_start(&mut self) -> REGION2_ADDR_START_W<0> {
        REGION2_ADDR_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region2_addr_start](index.html) module"]
pub struct REGION2_ADDR_START_SPEC;
impl crate::RegisterSpec for REGION2_ADDR_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region2_addr_start::R](R) reader structure"]
impl crate::Readable for REGION2_ADDR_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region2_addr_start::W](W) writer structure"]
impl crate::Writable for REGION2_ADDR_START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION2_ADDR_START to value 0"]
impl crate::Resettable for REGION2_ADDR_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
