#[doc = "Register `TIMER_INT1_MAP` reader"]
pub struct R(crate::R<TIMER_INT1_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_INT1_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_INT1_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_INT1_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_INT1_MAP` writer"]
pub struct W(crate::W<TIMER_INT1_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_INT1_MAP_SPEC>;
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
impl From<crate::W<TIMER_INT1_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_INT1_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_INT1_MAP` reader - this register used to map timer_int1 interrupt to one of core1's external interrupt"]
pub type TIMER_INT1_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_INT1_MAP` writer - this register used to map timer_int1 interrupt to one of core1's external interrupt"]
pub type TIMER_INT1_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER_INT1_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map timer_int1 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn timer_int1_map(&self) -> TIMER_INT1_MAP_R {
        TIMER_INT1_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_CORE1::TIMER_INT1_MAP")
            .field(
                "timer_int1_map",
                &format_args!("{}", self.timer_int1_map().bits()),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map timer_int1 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timer_int1_map(&mut self) -> TIMER_INT1_MAP_W<0> {
        TIMER_INT1_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "timer_int1 interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_int1_map](index.html) module"]
pub struct TIMER_INT1_MAP_SPEC;
impl crate::RegisterSpec for TIMER_INT1_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_int1_map::R](R) reader structure"]
impl crate::Readable for TIMER_INT1_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_int1_map::W](W) writer structure"]
impl crate::Writable for TIMER_INT1_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_INT1_MAP to value 0x10"]
impl crate::Resettable for TIMER_INT1_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
