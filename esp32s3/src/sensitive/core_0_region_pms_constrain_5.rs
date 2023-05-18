#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_5` reader"]
pub struct R(crate::R<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_5` writer"]
pub struct W(crate::W<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>;
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
impl From<crate::W<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_2` reader - Region 1 end address and Region 2 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_2` writer - Region 1 end address and Region 2 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_REGION_PMS_CONSTRAIN_5_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Region 1 end address and Region 2 start address for core0."]
    #[inline(always)]
    pub fn core_0_region_pms_constrain_addr_2(&self) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_R {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SENSITIVE::CORE_0_REGION_PMS_CONSTRAIN_5")
            .field(
                "core_0_region_pms_constrain_addr_2",
                &format_args!("{}", self.core_0_region_pms_constrain_addr_2().bits()),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 1 end address and Region 2 start address for core0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_region_pms_constrain_addr_2(
        &mut self,
    ) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_W<0> {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core0 region permission register 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_region_pms_constrain_5](index.html) module"]
pub struct CORE_0_REGION_PMS_CONSTRAIN_5_SPEC;
impl crate::RegisterSpec for CORE_0_REGION_PMS_CONSTRAIN_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_region_pms_constrain_5::R](R) reader structure"]
impl crate::Readable for CORE_0_REGION_PMS_CONSTRAIN_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_region_pms_constrain_5::W](W) writer structure"]
impl crate::Writable for CORE_0_REGION_PMS_CONSTRAIN_5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_REGION_PMS_CONSTRAIN_5 to value 0"]
impl crate::Resettable for CORE_0_REGION_PMS_CONSTRAIN_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
