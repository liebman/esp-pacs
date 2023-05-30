#[doc = "Register `CFG_UPDATE` writer"]
pub struct W(crate::W<CFG_UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_UPDATE_SPEC>;
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
impl From<crate::W<CFG_UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONF_UPDATE` writer - update the timing configurations"]
pub type CONF_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_UPDATE_SPEC, bool, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CFG_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - update the timing configurations"]
    #[inline(always)]
    #[must_use]
    pub fn conf_update(&mut self) -> CONF_UPDATE_W<0> {
        CONF_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "update sdio configurations\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_update](index.html) module"]
pub struct CFG_UPDATE_SPEC;
impl crate::RegisterSpec for CFG_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cfg_update::W](W) writer structure"]
impl crate::Writable for CFG_UPDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_UPDATE to value 0"]
impl crate::Resettable for CFG_UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
