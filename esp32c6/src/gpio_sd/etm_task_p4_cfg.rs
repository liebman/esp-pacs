#[doc = "Register `ETM_TASK_P4_CFG` reader"]
pub struct R(crate::R<ETM_TASK_P4_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETM_TASK_P4_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETM_TASK_P4_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETM_TASK_P4_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETM_TASK_P4_CFG` writer"]
pub struct W(crate::W<ETM_TASK_P4_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETM_TASK_P4_CFG_SPEC>;
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
impl From<crate::W<ETM_TASK_P4_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETM_TASK_P4_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETM_TASK_GPIO16_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO16_EN_R = crate::BitReader<bool>;
#[doc = "Field `ETM_TASK_GPIO16_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO16_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETM_TASK_P4_CFG_SPEC, bool, O>;
#[doc = "Field `ETM_TASK_GPIO16_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO16_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETM_TASK_GPIO16_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO16_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETM_TASK_P4_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETM_TASK_GPIO17_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO17_EN_R = crate::BitReader<bool>;
#[doc = "Field `ETM_TASK_GPIO17_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO17_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETM_TASK_P4_CFG_SPEC, bool, O>;
#[doc = "Field `ETM_TASK_GPIO17_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO17_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETM_TASK_GPIO17_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO17_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETM_TASK_P4_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETM_TASK_GPIO18_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO18_EN_R = crate::BitReader<bool>;
#[doc = "Field `ETM_TASK_GPIO18_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO18_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETM_TASK_P4_CFG_SPEC, bool, O>;
#[doc = "Field `ETM_TASK_GPIO18_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO18_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETM_TASK_GPIO18_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO18_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETM_TASK_P4_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETM_TASK_GPIO19_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO19_EN_R = crate::BitReader<bool>;
#[doc = "Field `ETM_TASK_GPIO19_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO19_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETM_TASK_P4_CFG_SPEC, bool, O>;
#[doc = "Field `ETM_TASK_GPIO19_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO19_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETM_TASK_GPIO19_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO19_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETM_TASK_P4_CFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio16_en(&self) -> ETM_TASK_GPIO16_EN_R {
        ETM_TASK_GPIO16_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio16_sel(&self) -> ETM_TASK_GPIO16_SEL_R {
        ETM_TASK_GPIO16_SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio17_en(&self) -> ETM_TASK_GPIO17_EN_R {
        ETM_TASK_GPIO17_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio17_sel(&self) -> ETM_TASK_GPIO17_SEL_R {
        ETM_TASK_GPIO17_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio18_en(&self) -> ETM_TASK_GPIO18_EN_R {
        ETM_TASK_GPIO18_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio18_sel(&self) -> ETM_TASK_GPIO18_SEL_R {
        ETM_TASK_GPIO18_SEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio19_en(&self) -> ETM_TASK_GPIO19_EN_R {
        ETM_TASK_GPIO19_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio19_sel(&self) -> ETM_TASK_GPIO19_SEL_R {
        ETM_TASK_GPIO19_SEL_R::new(((self.bits >> 25) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P4_CFG")
            .field(
                "etm_task_gpio16_en",
                &format_args!("{}", self.etm_task_gpio16_en().bit()),
            )
            .field(
                "etm_task_gpio16_sel",
                &format_args!("{}", self.etm_task_gpio16_sel().bits()),
            )
            .field(
                "etm_task_gpio17_en",
                &format_args!("{}", self.etm_task_gpio17_en().bit()),
            )
            .field(
                "etm_task_gpio17_sel",
                &format_args!("{}", self.etm_task_gpio17_sel().bits()),
            )
            .field(
                "etm_task_gpio18_en",
                &format_args!("{}", self.etm_task_gpio18_en().bit()),
            )
            .field(
                "etm_task_gpio18_sel",
                &format_args!("{}", self.etm_task_gpio18_sel().bits()),
            )
            .field(
                "etm_task_gpio19_en",
                &format_args!("{}", self.etm_task_gpio19_en().bit()),
            )
            .field(
                "etm_task_gpio19_sel",
                &format_args!("{}", self.etm_task_gpio19_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ETM_TASK_P4_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio16_en(&mut self) -> ETM_TASK_GPIO16_EN_W<0> {
        ETM_TASK_GPIO16_EN_W::new(self)
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio16_sel(&mut self) -> ETM_TASK_GPIO16_SEL_W<1> {
        ETM_TASK_GPIO16_SEL_W::new(self)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio17_en(&mut self) -> ETM_TASK_GPIO17_EN_W<8> {
        ETM_TASK_GPIO17_EN_W::new(self)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio17_sel(&mut self) -> ETM_TASK_GPIO17_SEL_W<9> {
        ETM_TASK_GPIO17_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio18_en(&mut self) -> ETM_TASK_GPIO18_EN_W<16> {
        ETM_TASK_GPIO18_EN_W::new(self)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio18_sel(&mut self) -> ETM_TASK_GPIO18_SEL_W<17> {
        ETM_TASK_GPIO18_SEL_W::new(self)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio19_en(&mut self) -> ETM_TASK_GPIO19_EN_W<24> {
        ETM_TASK_GPIO19_EN_W::new(self)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio19_sel(&mut self) -> ETM_TASK_GPIO19_SEL_W<25> {
        ETM_TASK_GPIO19_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etm_task_p4_cfg](index.html) module"]
pub struct ETM_TASK_P4_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P4_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etm_task_p4_cfg::R](R) reader structure"]
impl crate::Readable for ETM_TASK_P4_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etm_task_p4_cfg::W](W) writer structure"]
impl crate::Writable for ETM_TASK_P4_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETM_TASK_P4_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P4_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
