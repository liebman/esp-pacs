#[doc = "Register `CNTL_DBG_SEL` reader"]
pub struct R(crate::R<CNTL_DBG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTL_DBG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTL_DBG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTL_DBG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTL_DBG_SEL` writer"]
pub struct W(crate::W<CNTL_DBG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTL_DBG_SEL_SPEC>;
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
impl From<crate::W<CNTL_DBG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTL_DBG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEBUG_12M_NO_GATING` reader - Need add desc"]
pub type DEBUG_12M_NO_GATING_R = crate::BitReader<bool>;
#[doc = "Field `DEBUG_12M_NO_GATING` writer - Need add desc"]
pub type DEBUG_12M_NO_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CNTL_DBG_SEL_SPEC, bool, O>;
#[doc = "Field `DEBUG_BIT_SEL` reader - Need add desc"]
pub type DEBUG_BIT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBUG_BIT_SEL` writer - Need add desc"]
pub type DEBUG_BIT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_DBG_SEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DEBUG_SEL0` reader - Need add desc"]
pub type DEBUG_SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBUG_SEL0` writer - Need add desc"]
pub type DEBUG_SEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_DBG_SEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DEBUG_SEL1` reader - Need add desc"]
pub type DEBUG_SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBUG_SEL1` writer - Need add desc"]
pub type DEBUG_SEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_DBG_SEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DEBUG_SEL2` reader - Need add desc"]
pub type DEBUG_SEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBUG_SEL2` writer - Need add desc"]
pub type DEBUG_SEL2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_DBG_SEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DEBUG_SEL3` reader - Need add desc"]
pub type DEBUG_SEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBUG_SEL3` writer - Need add desc"]
pub type DEBUG_SEL3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_DBG_SEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DEBUG_SEL4` reader - Need add desc"]
pub type DEBUG_SEL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBUG_SEL4` writer - Need add desc"]
pub type DEBUG_SEL4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CNTL_DBG_SEL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    pub fn debug_12m_no_gating(&self) -> DEBUG_12M_NO_GATING_R {
        DEBUG_12M_NO_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Need add desc"]
    #[inline(always)]
    pub fn debug_bit_sel(&self) -> DEBUG_BIT_SEL_R {
        DEBUG_BIT_SEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel0(&self) -> DEBUG_SEL0_R {
        DEBUG_SEL0_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel1(&self) -> DEBUG_SEL1_R {
        DEBUG_SEL1_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel2(&self) -> DEBUG_SEL2_R {
        DEBUG_SEL2_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel3(&self) -> DEBUG_SEL3_R {
        DEBUG_SEL3_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    pub fn debug_sel4(&self) -> DEBUG_SEL4_R {
        DEBUG_SEL4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL_DBG_SEL")
            .field(
                "debug_12m_no_gating",
                &format_args!("{}", self.debug_12m_no_gating().bit()),
            )
            .field(
                "debug_bit_sel",
                &format_args!("{}", self.debug_bit_sel().bits()),
            )
            .field("debug_sel0", &format_args!("{}", self.debug_sel0().bits()))
            .field("debug_sel1", &format_args!("{}", self.debug_sel1().bits()))
            .field("debug_sel2", &format_args!("{}", self.debug_sel2().bits()))
            .field("debug_sel3", &format_args!("{}", self.debug_sel3().bits()))
            .field("debug_sel4", &format_args!("{}", self.debug_sel4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CNTL_DBG_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn debug_12m_no_gating(&mut self) -> DEBUG_12M_NO_GATING_W<1> {
        DEBUG_12M_NO_GATING_W::new(self)
    }
    #[doc = "Bits 2:6 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn debug_bit_sel(&mut self) -> DEBUG_BIT_SEL_W<2> {
        DEBUG_BIT_SEL_W::new(self)
    }
    #[doc = "Bits 7:11 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel0(&mut self) -> DEBUG_SEL0_W<7> {
        DEBUG_SEL0_W::new(self)
    }
    #[doc = "Bits 12:16 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel1(&mut self) -> DEBUG_SEL1_W<12> {
        DEBUG_SEL1_W::new(self)
    }
    #[doc = "Bits 17:21 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel2(&mut self) -> DEBUG_SEL2_W<17> {
        DEBUG_SEL2_W::new(self)
    }
    #[doc = "Bits 22:26 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel3(&mut self) -> DEBUG_SEL3_W<22> {
        DEBUG_SEL3_W::new(self)
    }
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn debug_sel4(&mut self) -> DEBUG_SEL4_W<27> {
        DEBUG_SEL4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl_dbg_sel](index.html) module"]
pub struct CNTL_DBG_SEL_SPEC;
impl crate::RegisterSpec for CNTL_DBG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntl_dbg_sel::R](R) reader structure"]
impl crate::Readable for CNTL_DBG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntl_dbg_sel::W](W) writer structure"]
impl crate::Writable for CNTL_DBG_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTL_DBG_SEL to value 0"]
impl crate::Resettable for CNTL_DBG_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
