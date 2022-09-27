#[doc = "Register `TEST_MUX` reader"]
pub struct R(crate::R<TEST_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_MUX` writer"]
pub struct W(crate::W<TEST_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_MUX_SPEC>;
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
impl From<crate::W<TEST_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENT_RTC` reader - ENT_RTC"]
pub type ENT_RTC_R = crate::BitReader<bool>;
#[doc = "Field `ENT_RTC` writer - ENT_RTC"]
pub type ENT_RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_MUX_SPEC, bool, O>;
#[doc = "Field `DTEST_RTC` reader - DTEST_RTC"]
pub type DTEST_RTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTEST_RTC` writer - DTEST_RTC"]
pub type DTEST_RTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEST_MUX_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 29 - ENT_RTC"]
    #[inline(always)]
    pub fn ent_rtc(&self) -> ENT_RTC_R {
        ENT_RTC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - DTEST_RTC"]
    #[inline(always)]
    pub fn dtest_rtc(&self) -> DTEST_RTC_R {
        DTEST_RTC_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 29 - ENT_RTC"]
    #[inline(always)]
    pub fn ent_rtc(&mut self) -> ENT_RTC_W<29> {
        ENT_RTC_W::new(self)
    }
    #[doc = "Bits 30:31 - DTEST_RTC"]
    #[inline(always)]
    pub fn dtest_rtc(&mut self) -> DTEST_RTC_W<30> {
        DTEST_RTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_mux](index.html) module"]
pub struct TEST_MUX_SPEC;
impl crate::RegisterSpec for TEST_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test_mux::R](R) reader structure"]
impl crate::Readable for TEST_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_mux::W](W) writer structure"]
impl crate::Writable for TEST_MUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST_MUX to value 0"]
impl crate::Resettable for TEST_MUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}