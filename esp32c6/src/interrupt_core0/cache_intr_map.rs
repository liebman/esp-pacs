#[doc = "Register `CACHE_INTR_MAP` reader"]
pub struct R(crate::R<CACHE_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_INTR_MAP` writer"]
pub struct W(crate::W<CACHE_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_INTR_MAP_SPEC>;
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
impl From<crate::W<CACHE_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_INTR_MAP` reader - Need add description"]
pub type CACHE_INTR_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CACHE_INTR_MAP` writer - Need add description"]
pub type CACHE_INTR_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CACHE_INTR_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    pub fn cache_intr_map(&self) -> CACHE_INTR_MAP_R {
        CACHE_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_INTR_MAP")
            .field(
                "cache_intr_map",
                &format_args!("{}", self.cache_intr_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn cache_intr_map(&mut self) -> CACHE_INTR_MAP_W<0> {
        CACHE_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_intr_map](index.html) module"]
pub struct CACHE_INTR_MAP_SPEC;
impl crate::RegisterSpec for CACHE_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_intr_map::R](R) reader structure"]
impl crate::Readable for CACHE_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_intr_map::W](W) writer structure"]
impl crate::Writable for CACHE_INTR_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_INTR_MAP to value 0"]
impl crate::Resettable for CACHE_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
