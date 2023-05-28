#[doc = "Register `T_STRING` reader"]
pub struct R(crate::R<T_STRING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T_STRING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T_STRING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T_STRING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T_STRING` writer"]
pub struct W(crate::W<T_STRING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T_STRING_SPEC>;
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
impl From<crate::W<T_STRING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T_STRING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_STRING` reader - Defines t_string for calculating the initial Hash value for SHA-512/t."]
pub type T_STRING_R = crate::FieldReader<u32, u32>;
#[doc = "Field `T_STRING` writer - Defines t_string for calculating the initial Hash value for SHA-512/t."]
pub type T_STRING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T_STRING_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Defines t_string for calculating the initial Hash value for SHA-512/t."]
    #[inline(always)]
    pub fn t_string(&self) -> T_STRING_R {
        T_STRING_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T_STRING")
            .field("t_string", &format_args!("{}", self.t_string().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T_STRING_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines t_string for calculating the initial Hash value for SHA-512/t."]
    #[inline(always)]
    #[must_use]
    pub fn t_string(&mut self) -> T_STRING_W<0> {
        T_STRING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "String content register for calculating initial Hash Value (only effective for SHA-512/t)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t_string](index.html) module"]
pub struct T_STRING_SPEC;
impl crate::RegisterSpec for T_STRING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t_string::R](R) reader structure"]
impl crate::Readable for T_STRING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t_string::W](W) writer structure"]
impl crate::Writable for T_STRING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T_STRING to value 0"]
impl crate::Resettable for T_STRING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
