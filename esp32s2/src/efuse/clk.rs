#[doc = "Register `CLK` reader"]
pub struct R(crate::R<CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK` writer"]
pub struct W(crate::W<CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SPEC>;
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
impl From<crate::W<CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EFUSE_MEM_FORCE_PD` reader - If set, forces eFuse SRAM into power-saving mode."]
pub type EFUSE_MEM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_MEM_FORCE_PD` writer - If set, forces eFuse SRAM into power-saving mode."]
pub type EFUSE_MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_SPEC, bool, O>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - If set, forces to activate clock signal of eFuse SRAM."]
pub type MEM_CLK_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - If set, forces to activate clock signal of eFuse SRAM."]
pub type MEM_CLK_FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_SPEC, bool, O>;
#[doc = "Field `EFUSE_MEM_FORCE_PU` reader - If set, forces eFuse SRAM into working mode."]
pub type EFUSE_MEM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE_MEM_FORCE_PU` writer - If set, forces eFuse SRAM into working mode."]
pub type EFUSE_MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_SPEC, bool, O>;
#[doc = "Field `EN` reader - If set, forces to enable clock signal of eFuse memory."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - If set, forces to enable clock signal of eFuse memory."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If set, forces eFuse SRAM into power-saving mode."]
    #[inline(always)]
    pub fn efuse_mem_force_pd(&self) -> EFUSE_MEM_FORCE_PD_R {
        EFUSE_MEM_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set, forces to activate clock signal of eFuse SRAM."]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MEM_CLK_FORCE_ON_R {
        MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set, forces eFuse SRAM into working mode."]
    #[inline(always)]
    pub fn efuse_mem_force_pu(&self) -> EFUSE_MEM_FORCE_PU_R {
        EFUSE_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - If set, forces to enable clock signal of eFuse memory."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE::CLK")
            .field(
                "efuse_mem_force_pd",
                &format_args!("{}", self.efuse_mem_force_pd().bit()),
            )
            .field(
                "mem_clk_force_on",
                &format_args!("{}", self.mem_clk_force_on().bit()),
            )
            .field(
                "efuse_mem_force_pu",
                &format_args!("{}", self.efuse_mem_force_pu().bit()),
            )
            .field("en", &format_args!("{}", self.en().bit()))
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - If set, forces eFuse SRAM into power-saving mode."]
    #[inline(always)]
    #[must_use]
    pub fn efuse_mem_force_pd(&mut self) -> EFUSE_MEM_FORCE_PD_W<0> {
        EFUSE_MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 1 - If set, forces to activate clock signal of eFuse SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W<1> {
        MEM_CLK_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 2 - If set, forces eFuse SRAM into working mode."]
    #[inline(always)]
    #[must_use]
    pub fn efuse_mem_force_pu(&mut self) -> EFUSE_MEM_FORCE_PU_W<2> {
        EFUSE_MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 16 - If set, forces to enable clock signal of eFuse memory."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<16> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eFuse clock configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](index.html) module"]
pub struct CLK_SPEC;
impl crate::RegisterSpec for CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk::R](R) reader structure"]
impl crate::Readable for CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk::W](W) writer structure"]
impl crate::Writable for CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK to value 0x02"]
impl crate::Resettable for CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
