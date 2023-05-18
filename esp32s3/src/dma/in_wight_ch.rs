#[doc = "Register `IN_WIGHT_CH%s` reader"]
pub struct R(crate::R<IN_WIGHT_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_WIGHT_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_WIGHT_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_WIGHT_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_WIGHT_CH%s` writer"]
pub struct W(crate::W<IN_WIGHT_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_WIGHT_CH_SPEC>;
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
impl From<crate::W<IN_WIGHT_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_WIGHT_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_WEIGHT` reader - The weight of Rx channel 0."]
pub type RX_WEIGHT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_WEIGHT` writer - The weight of Rx channel 0."]
pub type RX_WEIGHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IN_WIGHT_CH_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 8:11 - The weight of Rx channel 0."]
    #[inline(always)]
    pub fn rx_weight(&self) -> RX_WEIGHT_R {
        RX_WEIGHT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA::IN_WIGHT_CH")
            .field("rx_weight", &format_args!("{}", self.rx_weight().bits()))
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:11 - The weight of Rx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn rx_weight(&mut self) -> RX_WEIGHT_W<8> {
        RX_WEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Weight register of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_wight_ch](index.html) module"]
pub struct IN_WIGHT_CH_SPEC;
impl crate::RegisterSpec for IN_WIGHT_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_wight_ch::R](R) reader structure"]
impl crate::Readable for IN_WIGHT_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_wight_ch::W](W) writer structure"]
impl crate::Writable for IN_WIGHT_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_WIGHT_CH%s to value 0x0f00"]
impl crate::Resettable for IN_WIGHT_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00;
}
