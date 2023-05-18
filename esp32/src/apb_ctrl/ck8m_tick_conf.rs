#[doc = "Register `CK8M_TICK_CONF` reader"]
pub struct R(crate::R<CK8M_TICK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CK8M_TICK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CK8M_TICK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CK8M_TICK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CK8M_TICK_CONF` writer"]
pub struct W(crate::W<CK8M_TICK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CK8M_TICK_CONF_SPEC>;
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
impl From<crate::W<CK8M_TICK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CK8M_TICK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CK8M_TICK_NUM` reader - "]
pub type CK8M_TICK_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CK8M_TICK_NUM` writer - "]
pub type CK8M_TICK_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CK8M_TICK_CONF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ck8m_tick_num(&self) -> CK8M_TICK_NUM_R {
        CK8M_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_CTRL::CK8M_TICK_CONF")
            .field(
                "ck8m_tick_num",
                &format_args!("{}", self.ck8m_tick_num().bits()),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_tick_num(&mut self) -> CK8M_TICK_NUM_W<0> {
        CK8M_TICK_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ck8m_tick_conf](index.html) module"]
pub struct CK8M_TICK_CONF_SPEC;
impl crate::RegisterSpec for CK8M_TICK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ck8m_tick_conf::R](R) reader structure"]
impl crate::Readable for CK8M_TICK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ck8m_tick_conf::W](W) writer structure"]
impl crate::Writable for CK8M_TICK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CK8M_TICK_CONF to value 0x0b"]
impl crate::Resettable for CK8M_TICK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
