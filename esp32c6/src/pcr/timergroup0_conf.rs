#[doc = "Register `TIMERGROUP0_CONF` reader"]
pub struct R(crate::R<TIMERGROUP0_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMERGROUP0_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMERGROUP0_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMERGROUP0_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMERGROUP0_CONF` writer"]
pub struct W(crate::W<TIMERGROUP0_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMERGROUP0_CONF_SPEC>;
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
impl From<crate::W<TIMERGROUP0_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMERGROUP0_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TG0_CLK_EN` reader - Set 1 to enable timer_group0 apb clock"]
pub type TG0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TG0_CLK_EN` writer - Set 1 to enable timer_group0 apb clock"]
pub type TG0_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERGROUP0_CONF_SPEC, bool, O>;
#[doc = "Field `TG0_RST_EN` reader - Set 0 to reset timer_group0 module"]
pub type TG0_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `TG0_RST_EN` writer - Set 0 to reset timer_group0 module"]
pub type TG0_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMERGROUP0_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable timer_group0 apb clock"]
    #[inline(always)]
    pub fn tg0_clk_en(&self) -> TG0_CLK_EN_R {
        TG0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group0 module"]
    #[inline(always)]
    pub fn tg0_rst_en(&self) -> TG0_RST_EN_R {
        TG0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCR::TIMERGROUP0_CONF")
            .field("tg0_clk_en", &format_args!("{}", self.tg0_clk_en().bit()))
            .field("tg0_rst_en", &format_args!("{}", self.tg0_rst_en().bit()))
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable timer_group0 apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn tg0_clk_en(&mut self) -> TG0_CLK_EN_W<0> {
        TG0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group0 module"]
    #[inline(always)]
    #[must_use]
    pub fn tg0_rst_en(&mut self) -> TG0_RST_EN_W<1> {
        TG0_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMERGROUP0 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timergroup0_conf](index.html) module"]
pub struct TIMERGROUP0_CONF_SPEC;
impl crate::RegisterSpec for TIMERGROUP0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timergroup0_conf::R](R) reader structure"]
impl crate::Readable for TIMERGROUP0_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timergroup0_conf::W](W) writer structure"]
impl crate::Writable for TIMERGROUP0_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMERGROUP0_CONF to value 0x01"]
impl crate::Resettable for TIMERGROUP0_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
