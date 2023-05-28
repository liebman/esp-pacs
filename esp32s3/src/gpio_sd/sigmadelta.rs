#[doc = "Register `SIGMADELTA%s` reader"]
pub struct R(crate::R<SIGMADELTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA%s` writer"]
pub struct W(crate::W<SIGMADELTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA_SPEC>;
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
impl From<crate::W<SIGMADELTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD_IN` reader - This field is used to configure the duty cycle of sigma delta modulation output."]
pub type SD_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SD_IN` writer - This field is used to configure the duty cycle of sigma delta modulation output."]
pub type SD_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIGMADELTA_SPEC, u8, u8, 8, O>;
#[doc = "Field `SD_PRESCALE` reader - This field is used to set a divider value to divide APB clock."]
pub type SD_PRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SD_PRESCALE` writer - This field is used to set a divider value to divide APB clock."]
pub type SD_PRESCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGMADELTA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This field is used to configure the duty cycle of sigma delta modulation output."]
    #[inline(always)]
    pub fn sd_in(&self) -> SD_IN_R {
        SD_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This field is used to set a divider value to divide APB clock."]
    #[inline(always)]
    pub fn sd_prescale(&self) -> SD_PRESCALE_R {
        SD_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA")
            .field("sd_in", &format_args!("{}", self.sd_in().bits()))
            .field(
                "sd_prescale",
                &format_args!("{}", self.sd_prescale().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This field is used to configure the duty cycle of sigma delta modulation output."]
    #[inline(always)]
    #[must_use]
    pub fn sd_in(&mut self) -> SD_IN_W<0> {
        SD_IN_W::new(self)
    }
    #[doc = "Bits 8:15 - This field is used to set a divider value to divide APB clock."]
    #[inline(always)]
    #[must_use]
    pub fn sd_prescale(&mut self) -> SD_PRESCALE_W<8> {
        SD_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Duty Cycle Configure Register of SDM%s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta](index.html) module"]
pub struct SIGMADELTA_SPEC;
impl crate::RegisterSpec for SIGMADELTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta::R](R) reader structure"]
impl crate::Readable for SIGMADELTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta::W](W) writer structure"]
impl crate::Writable for SIGMADELTA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA%s to value 0xff00"]
impl crate::Resettable for SIGMADELTA_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
