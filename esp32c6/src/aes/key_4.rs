#[doc = "Register `KEY_4` reader"]
pub struct R(crate::R<KEY_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_4` writer"]
pub struct W(crate::W<KEY_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_4_SPEC>;
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
impl From<crate::W<KEY_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_4` reader - This bits stores key_4 that is a part of key material."]
pub type KEY_4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY_4` writer - This bits stores key_4 that is a part of key material."]
pub type KEY_4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEY_4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This bits stores key_4 that is a part of key material."]
    #[inline(always)]
    pub fn key_4(&self) -> KEY_4_R {
        KEY_4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES::KEY_4")
            .field("key_4", &format_args!("{}", self.key_4().bits()))
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_4 that is a part of key material."]
    #[inline(always)]
    #[must_use]
    pub fn key_4(&mut self) -> KEY_4_W<0> {
        KEY_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key material key_4 configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_4](index.html) module"]
pub struct KEY_4_SPEC;
impl crate::RegisterSpec for KEY_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_4::R](R) reader structure"]
impl crate::Readable for KEY_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_4::W](W) writer structure"]
impl crate::Writable for KEY_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY_4 to value 0"]
impl crate::Resettable for KEY_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
