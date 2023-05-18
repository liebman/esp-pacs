#[doc = "Register `APP_VECBASE_CTRL` reader"]
pub struct R(crate::R<APP_VECBASE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_VECBASE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_VECBASE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_VECBASE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_VECBASE_CTRL` writer"]
pub struct W(crate::W<APP_VECBASE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_VECBASE_CTRL_SPEC>;
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
impl From<crate::W<APP_VECBASE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_VECBASE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_OUT_VECBASE_SEL` reader - "]
pub type APP_OUT_VECBASE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APP_OUT_VECBASE_SEL` writer - "]
pub type APP_OUT_VECBASE_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, APP_VECBASE_CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn app_out_vecbase_sel(&self) -> APP_OUT_VECBASE_SEL_R {
        APP_OUT_VECBASE_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPORT::APP_VECBASE_CTRL")
            .field(
                "app_out_vecbase_sel",
                &format_args!("{}", self.app_out_vecbase_sel().bits()),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn app_out_vecbase_sel(&mut self) -> APP_OUT_VECBASE_SEL_W<0> {
        APP_OUT_VECBASE_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_vecbase_ctrl](index.html) module"]
pub struct APP_VECBASE_CTRL_SPEC;
impl crate::RegisterSpec for APP_VECBASE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_vecbase_ctrl::R](R) reader structure"]
impl crate::Readable for APP_VECBASE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_vecbase_ctrl::W](W) writer structure"]
impl crate::Writable for APP_VECBASE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APP_VECBASE_CTRL to value 0"]
impl crate::Resettable for APP_VECBASE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
