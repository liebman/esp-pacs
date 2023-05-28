#[doc = "Register `CACHE_PRELOAD_INT_CTRL` reader"]
pub struct R(crate::R<CACHE_PRELOAD_INT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_PRELOAD_INT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_PRELOAD_INT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_PRELOAD_INT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_PRELOAD_INT_CTRL` writer"]
pub struct W(crate::W<CACHE_PRELOAD_INT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_PRELOAD_INT_CTRL_SPEC>;
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
impl From<crate::W<CACHE_PRELOAD_INT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_PRELOAD_INT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ICACHE_PRELOAD_INT_ST` reader - The bit is used to indicate the interrupt by icache pre-load done."]
pub type PRO_ICACHE_PRELOAD_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `PRO_ICACHE_PRELOAD_INT_ENA` reader - The bit is used to enable the interrupt by icache pre-load done."]
pub type PRO_ICACHE_PRELOAD_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PRO_ICACHE_PRELOAD_INT_ENA` writer - The bit is used to enable the interrupt by icache pre-load done."]
pub type PRO_ICACHE_PRELOAD_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_PRELOAD_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `PRO_ICACHE_PRELOAD_INT_CLR` writer - The bit is used to clear the interrupt by icache pre-load done."]
pub type PRO_ICACHE_PRELOAD_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_PRELOAD_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `PRO_DCACHE_PRELOAD_INT_ST` reader - The bit is used to indicate the interrupt by dcache pre-load done."]
pub type PRO_DCACHE_PRELOAD_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `PRO_DCACHE_PRELOAD_INT_ENA` reader - The bit is used to enable the interrupt by dcache pre-load done."]
pub type PRO_DCACHE_PRELOAD_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PRO_DCACHE_PRELOAD_INT_ENA` writer - The bit is used to enable the interrupt by dcache pre-load done."]
pub type PRO_DCACHE_PRELOAD_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_PRELOAD_INT_CTRL_SPEC, bool, O>;
#[doc = "Field `PRO_DCACHE_PRELOAD_INT_CLR` writer - The bit is used to clear the interrupt by dcache pre-load done."]
pub type PRO_DCACHE_PRELOAD_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_PRELOAD_INT_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to indicate the interrupt by icache pre-load done."]
    #[inline(always)]
    pub fn pro_icache_preload_int_st(&self) -> PRO_ICACHE_PRELOAD_INT_ST_R {
        PRO_ICACHE_PRELOAD_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the interrupt by icache pre-load done."]
    #[inline(always)]
    pub fn pro_icache_preload_int_ena(&self) -> PRO_ICACHE_PRELOAD_INT_ENA_R {
        PRO_ICACHE_PRELOAD_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to indicate the interrupt by dcache pre-load done."]
    #[inline(always)]
    pub fn pro_dcache_preload_int_st(&self) -> PRO_DCACHE_PRELOAD_INT_ST_R {
        PRO_DCACHE_PRELOAD_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt by dcache pre-load done."]
    #[inline(always)]
    pub fn pro_dcache_preload_int_ena(&self) -> PRO_DCACHE_PRELOAD_INT_ENA_R {
        PRO_DCACHE_PRELOAD_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_PRELOAD_INT_CTRL")
            .field(
                "pro_icache_preload_int_st",
                &format_args!("{}", self.pro_icache_preload_int_st().bit()),
            )
            .field(
                "pro_icache_preload_int_ena",
                &format_args!("{}", self.pro_icache_preload_int_ena().bit()),
            )
            .field(
                "pro_dcache_preload_int_st",
                &format_args!("{}", self.pro_dcache_preload_int_st().bit()),
            )
            .field(
                "pro_dcache_preload_int_ena",
                &format_args!("{}", self.pro_dcache_preload_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_PRELOAD_INT_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - The bit is used to enable the interrupt by icache pre-load done."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_preload_int_ena(&mut self) -> PRO_ICACHE_PRELOAD_INT_ENA_W<1> {
        PRO_ICACHE_PRELOAD_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The bit is used to clear the interrupt by icache pre-load done."]
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_preload_int_clr(&mut self) -> PRO_ICACHE_PRELOAD_INT_CLR_W<2> {
        PRO_ICACHE_PRELOAD_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt by dcache pre-load done."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_preload_int_ena(&mut self) -> PRO_DCACHE_PRELOAD_INT_ENA_W<4> {
        PRO_DCACHE_PRELOAD_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The bit is used to clear the interrupt by dcache pre-load done."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_preload_int_clr(&mut self) -> PRO_DCACHE_PRELOAD_INT_CLR_W<5> {
        PRO_DCACHE_PRELOAD_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_preload_int_ctrl](index.html) module"]
pub struct CACHE_PRELOAD_INT_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_PRELOAD_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_preload_int_ctrl::R](R) reader structure"]
impl crate::Readable for CACHE_PRELOAD_INT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_preload_int_ctrl::W](W) writer structure"]
impl crate::Writable for CACHE_PRELOAD_INT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_PRELOAD_INT_CTRL to value 0"]
impl crate::Resettable for CACHE_PRELOAD_INT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
