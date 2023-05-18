#[doc = "Register `ENABLE1` reader"]
pub struct R(crate::R<ENABLE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLE1` writer"]
pub struct W(crate::W<ENABLE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE1_SPEC>;
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
impl From<crate::W<ENABLE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - GPIO32~53 output enable register."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - GPIO32~53 output enable register."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENABLE1_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - GPIO32~53 output enable register."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO::ENABLE1")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - GPIO32~53 output enable register."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO32 ~ 53 output enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable1](index.html) module"]
pub struct ENABLE1_SPEC;
impl crate::RegisterSpec for ENABLE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable1::R](R) reader structure"]
impl crate::Readable for ENABLE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable1::W](W) writer structure"]
impl crate::Writable for ENABLE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENABLE1 to value 0"]
impl crate::Resettable for ENABLE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
