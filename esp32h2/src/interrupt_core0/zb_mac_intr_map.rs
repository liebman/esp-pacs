#[doc = "Register `ZB_MAC_INTR_MAP` reader"]
pub struct R(crate::R<ZB_MAC_INTR_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ZB_MAC_INTR_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ZB_MAC_INTR_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ZB_MAC_INTR_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ZB_MAC_INTR_MAP` writer"]
pub struct W(crate::W<ZB_MAC_INTR_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ZB_MAC_INTR_MAP_SPEC>;
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
impl From<crate::W<ZB_MAC_INTR_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ZB_MAC_INTR_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZB_MAC_INTR_MAP` reader - CORE0_ZB_MAC_INTR mapping register"]
pub type ZB_MAC_INTR_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ZB_MAC_INTR_MAP` writer - CORE0_ZB_MAC_INTR mapping register"]
pub type ZB_MAC_INTR_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ZB_MAC_INTR_MAP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - CORE0_ZB_MAC_INTR mapping register"]
    #[inline(always)]
    pub fn zb_mac_intr_map(&self) -> ZB_MAC_INTR_MAP_R {
        ZB_MAC_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_CORE0::ZB_MAC_INTR_MAP")
            .field(
                "zb_mac_intr_map",
                &format_args!("{}", self.zb_mac_intr_map().bits()),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_ZB_MAC_INTR mapping register"]
    #[inline(always)]
    #[must_use]
    pub fn zb_mac_intr_map(&mut self) -> ZB_MAC_INTR_MAP_W<0> {
        ZB_MAC_INTR_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [zb_mac_intr_map](index.html) module"]
pub struct ZB_MAC_INTR_MAP_SPEC;
impl crate::RegisterSpec for ZB_MAC_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [zb_mac_intr_map::R](R) reader structure"]
impl crate::Readable for ZB_MAC_INTR_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [zb_mac_intr_map::W](W) writer structure"]
impl crate::Writable for ZB_MAC_INTR_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ZB_MAC_INTR_MAP to value 0"]
impl crate::Resettable for ZB_MAC_INTR_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
