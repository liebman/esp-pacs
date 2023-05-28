#[doc = "Register `APPCPU_CTRL_B` reader"]
pub struct R(crate::R<APPCPU_CTRL_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APPCPU_CTRL_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APPCPU_CTRL_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APPCPU_CTRL_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APPCPU_CTRL_B` writer"]
pub struct W(crate::W<APPCPU_CTRL_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APPCPU_CTRL_B_SPEC>;
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
impl From<crate::W<APPCPU_CTRL_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APPCPU_CTRL_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APPCPU_CLKGATE_EN` reader - "]
pub type APPCPU_CLKGATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `APPCPU_CLKGATE_EN` writer - "]
pub type APPCPU_CLKGATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APPCPU_CTRL_B_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn appcpu_clkgate_en(&self) -> APPCPU_CLKGATE_EN_R {
        APPCPU_CLKGATE_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPCPU_CTRL_B")
            .field(
                "appcpu_clkgate_en",
                &format_args!("{}", self.appcpu_clkgate_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APPCPU_CTRL_B_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn appcpu_clkgate_en(&mut self) -> APPCPU_CLKGATE_EN_W<0> {
        APPCPU_CLKGATE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [appcpu_ctrl_b](index.html) module"]
pub struct APPCPU_CTRL_B_SPEC;
impl crate::RegisterSpec for APPCPU_CTRL_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [appcpu_ctrl_b::R](R) reader structure"]
impl crate::Readable for APPCPU_CTRL_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [appcpu_ctrl_b::W](W) writer structure"]
impl crate::Writable for APPCPU_CTRL_B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APPCPU_CTRL_B to value 0"]
impl crate::Resettable for APPCPU_CTRL_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
