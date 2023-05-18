#[doc = "Register `CPU_INT_PRI_0` reader"]
pub struct R(crate::R<CPU_INT_PRI_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_INT_PRI_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_INT_PRI_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_INT_PRI_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_INT_PRI_0` writer"]
pub struct W(crate::W<CPU_INT_PRI_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_INT_PRI_0_SPEC>;
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
impl From<crate::W<CPU_INT_PRI_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_INT_PRI_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_PRI_0_MAP` reader - Need add description"]
pub type CPU_PRI_0_MAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_PRI_0_MAP` writer - Need add description"]
pub type CPU_PRI_0_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_INT_PRI_0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Need add description"]
    #[inline(always)]
    pub fn cpu_pri_0_map(&self) -> CPU_PRI_0_MAP_R {
        CPU_PRI_0_MAP_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTPRI::CPU_INT_PRI_0")
            .field(
                "cpu_pri_0_map",
                &format_args!("{}", self.cpu_pri_0_map().bits()),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_pri_0_map(&mut self) -> CPU_PRI_0_MAP_W<0> {
        CPU_PRI_0_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_int_pri_0](index.html) module"]
pub struct CPU_INT_PRI_0_SPEC;
impl crate::RegisterSpec for CPU_INT_PRI_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_int_pri_0::R](R) reader structure"]
impl crate::Readable for CPU_INT_PRI_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_int_pri_0::W](W) writer structure"]
impl crate::Writable for CPU_INT_PRI_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_INT_PRI_0 to value 0"]
impl crate::Resettable for CPU_INT_PRI_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
