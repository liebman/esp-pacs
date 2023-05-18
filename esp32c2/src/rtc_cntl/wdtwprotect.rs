#[doc = "Register `WDTWPROTECT` reader"]
pub struct R(crate::R<WDTWPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTWPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTWPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTWPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTWPROTECT` writer"]
pub struct W(crate::W<WDTWPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTWPROTECT_SPEC>;
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
impl From<crate::W<WDTWPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTWPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_WKEY` reader - Need add desc"]
pub type WDT_WKEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WDT_WKEY` writer - Need add desc"]
pub type WDT_WKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WDTWPROTECT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    pub fn wdt_wkey(&self) -> WDT_WKEY_R {
        WDT_WKEY_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_CNTL::WDTWPROTECT")
            .field("wdt_wkey", &format_args!("{}", self.wdt_wkey().bits()))
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_wkey(&mut self) -> WDT_WKEY_W<0> {
        WDT_WKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtwprotect](index.html) module"]
pub struct WDTWPROTECT_SPEC;
impl crate::RegisterSpec for WDTWPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtwprotect::R](R) reader structure"]
impl crate::Readable for WDTWPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtwprotect::W](W) writer structure"]
impl crate::Writable for WDTWPROTECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTWPROTECT to value 0"]
impl crate::Resettable for WDTWPROTECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
