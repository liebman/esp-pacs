#[doc = "Register `OUTFIFO_PUSH` reader"]
pub struct R(crate::R<OUTFIFO_PUSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTFIFO_PUSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTFIFO_PUSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTFIFO_PUSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTFIFO_PUSH` writer"]
pub struct W(crate::W<OUTFIFO_PUSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTFIFO_PUSH_SPEC>;
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
impl From<crate::W<OUTFIFO_PUSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTFIFO_PUSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTFIFO_WDATA` reader - "]
pub type OUTFIFO_WDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUTFIFO_WDATA` writer - "]
pub type OUTFIFO_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTFIFO_PUSH_SPEC, u16, u16, 9, O>;
#[doc = "Field `OUTFIFO_PUSH` reader - "]
pub type OUTFIFO_PUSH_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_PUSH` writer - "]
pub type OUTFIFO_PUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTFIFO_PUSH_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn outfifo_wdata(&self) -> OUTFIFO_WDATA_R {
        OUTFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn outfifo_push(&self) -> OUTFIFO_PUSH_R {
        OUTFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S0::OUTFIFO_PUSH")
            .field(
                "outfifo_wdata",
                &format_args!("{}", self.outfifo_wdata().bits()),
            )
            .field(
                "outfifo_push",
                &format_args!("{}", self.outfifo_push().bit()),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_wdata(&mut self) -> OUTFIFO_WDATA_W<0> {
        OUTFIFO_WDATA_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_push(&mut self) -> OUTFIFO_PUSH_W<16> {
        OUTFIFO_PUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outfifo_push](index.html) module"]
pub struct OUTFIFO_PUSH_SPEC;
impl crate::RegisterSpec for OUTFIFO_PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outfifo_push::R](R) reader structure"]
impl crate::Readable for OUTFIFO_PUSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outfifo_push::W](W) writer structure"]
impl crate::Writable for OUTFIFO_PUSH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTFIFO_PUSH to value 0"]
impl crate::Resettable for OUTFIFO_PUSH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
