#[doc = "Register `CONTINUE_` writer"]
pub struct W(crate::W<CONTINUE__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTINUE__SPEC>;
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
impl From<crate::W<CONTINUE__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTINUE__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTINUE_OP` writer - Write 1 to continue Typical SHA calculation."]
pub type CONTINUE_OP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTINUE__SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Write 1 to continue Typical SHA calculation."]
    #[inline(always)]
    #[must_use]
    pub fn continue_op(&mut self) -> CONTINUE_OP_W<0> {
        CONTINUE_OP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Continues SHA operation (only effective in Typical SHA mode)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [continue_](index.html) module"]
pub struct CONTINUE__SPEC;
impl crate::RegisterSpec for CONTINUE__SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [continue_::W](W) writer structure"]
impl crate::Writable for CONTINUE__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTINUE_ to value 0"]
impl crate::Resettable for CONTINUE__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
