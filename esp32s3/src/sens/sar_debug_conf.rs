#[doc = "Register `SAR_DEBUG_CONF` reader"]
pub struct R(crate::R<SAR_DEBUG_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_DEBUG_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_DEBUG_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_DEBUG_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_DEBUG_CONF` writer"]
pub struct W(crate::W<SAR_DEBUG_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_DEBUG_CONF_SPEC>;
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
impl From<crate::W<SAR_DEBUG_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_DEBUG_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_DEBUG_BIT_SEL` reader - no public"]
pub type SAR_DEBUG_BIT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_DEBUG_BIT_SEL` writer - no public"]
pub type SAR_DEBUG_BIT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_DEBUG_CONF_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - no public"]
    #[inline(always)]
    pub fn sar_debug_bit_sel(&self) -> SAR_DEBUG_BIT_SEL_R {
        SAR_DEBUG_BIT_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar_debug_bit_sel(&mut self) -> SAR_DEBUG_BIT_SEL_W<0> {
        SAR_DEBUG_BIT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc peri debug configure\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_debug_conf](index.html) module"]
pub struct SAR_DEBUG_CONF_SPEC;
impl crate::RegisterSpec for SAR_DEBUG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_debug_conf::R](R) reader structure"]
impl crate::Readable for SAR_DEBUG_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_debug_conf::W](W) writer structure"]
impl crate::Writable for SAR_DEBUG_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_DEBUG_CONF to value 0"]
impl crate::Resettable for SAR_DEBUG_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}