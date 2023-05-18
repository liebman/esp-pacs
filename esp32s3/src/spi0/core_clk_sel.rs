#[doc = "Register `CORE_CLK_SEL` reader"]
pub struct R(crate::R<CORE_CLK_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_CLK_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_CLK_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_CLK_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_CLK_SEL` writer"]
pub struct W(crate::W<CORE_CLK_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_CLK_SEL_SPEC>;
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
impl From<crate::W<CORE_CLK_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_CLK_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_CLK_SEL` reader - When the digital system clock selects PLL clock and the frequency of PLL clock is 480MHz, the value of SPI_MEM_CORE_CLK_SEL: 0: SPI0/1 module clock (MSPI_CORE_CLK) is 80MHz. 1: MSPI_CORE_CLK is 120MHz. 2: MSPI_CORE_CLK is 160MHz. 3: MSPI_CORE_CLK is 240MHz. When the digital system clock selects PLL clock and the frequency of PLL clock is 320MHz, the value of SPI_MEM_CORE_CLK_SEL: 0: MSPI_CORE_CLK is 80MHz. 1: MSPI_CORE_CLK is 80MHz. 2: MSPI_CORE_CLK 160MHz. 3: Not used."]
pub type CORE_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_CLK_SEL` writer - When the digital system clock selects PLL clock and the frequency of PLL clock is 480MHz, the value of SPI_MEM_CORE_CLK_SEL: 0: SPI0/1 module clock (MSPI_CORE_CLK) is 80MHz. 1: MSPI_CORE_CLK is 120MHz. 2: MSPI_CORE_CLK is 160MHz. 3: MSPI_CORE_CLK is 240MHz. When the digital system clock selects PLL clock and the frequency of PLL clock is 320MHz, the value of SPI_MEM_CORE_CLK_SEL: 0: MSPI_CORE_CLK is 80MHz. 1: MSPI_CORE_CLK is 80MHz. 2: MSPI_CORE_CLK 160MHz. 3: Not used."]
pub type CORE_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_CLK_SEL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - When the digital system clock selects PLL clock and the frequency of PLL clock is 480MHz, the value of SPI_MEM_CORE_CLK_SEL: 0: SPI0/1 module clock (MSPI_CORE_CLK) is 80MHz. 1: MSPI_CORE_CLK is 120MHz. 2: MSPI_CORE_CLK is 160MHz. 3: MSPI_CORE_CLK is 240MHz. When the digital system clock selects PLL clock and the frequency of PLL clock is 320MHz, the value of SPI_MEM_CORE_CLK_SEL: 0: MSPI_CORE_CLK is 80MHz. 1: MSPI_CORE_CLK is 80MHz. 2: MSPI_CORE_CLK 160MHz. 3: Not used."]
    #[inline(always)]
    pub fn core_clk_sel(&self) -> CORE_CLK_SEL_R {
        CORE_CLK_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI0::CORE_CLK_SEL")
            .field(
                "core_clk_sel",
                &format_args!("{}", self.core_clk_sel().bits()),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - When the digital system clock selects PLL clock and the frequency of PLL clock is 480MHz, the value of SPI_MEM_CORE_CLK_SEL: 0: SPI0/1 module clock (MSPI_CORE_CLK) is 80MHz. 1: MSPI_CORE_CLK is 120MHz. 2: MSPI_CORE_CLK is 160MHz. 3: MSPI_CORE_CLK is 240MHz. When the digital system clock selects PLL clock and the frequency of PLL clock is 320MHz, the value of SPI_MEM_CORE_CLK_SEL: 0: MSPI_CORE_CLK is 80MHz. 1: MSPI_CORE_CLK is 80MHz. 2: MSPI_CORE_CLK 160MHz. 3: Not used."]
    #[inline(always)]
    #[must_use]
    pub fn core_clk_sel(&mut self) -> CORE_CLK_SEL_W<0> {
        CORE_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 module clock select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_clk_sel](index.html) module"]
pub struct CORE_CLK_SEL_SPEC;
impl crate::RegisterSpec for CORE_CLK_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_clk_sel::R](R) reader structure"]
impl crate::Readable for CORE_CLK_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_clk_sel::W](W) writer structure"]
impl crate::Writable for CORE_CLK_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_CLK_SEL to value 0"]
impl crate::Resettable for CORE_CLK_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
