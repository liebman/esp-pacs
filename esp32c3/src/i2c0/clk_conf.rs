#[doc = "Register `CLK_CONF` reader"]
pub struct R(crate::R<CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONF` writer"]
pub struct W(crate::W<CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONF_SPEC>;
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
impl From<crate::W<CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLK_DIV_NUM` reader - reg_sclk_div_num"]
pub type SCLK_DIV_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLK_DIV_NUM` writer - reg_sclk_div_num"]
pub type SCLK_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `SCLK_DIV_A` reader - reg_sclk_div_a"]
pub type SCLK_DIV_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLK_DIV_A` writer - reg_sclk_div_a"]
pub type SCLK_DIV_A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CONF_SPEC, u8, u8, 6, O>;
#[doc = "Field `SCLK_DIV_B` reader - reg_sclk_div_b"]
pub type SCLK_DIV_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLK_DIV_B` writer - reg_sclk_div_b"]
pub type SCLK_DIV_B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CONF_SPEC, u8, u8, 6, O>;
#[doc = "Field `SCLK_SEL` reader - reg_sclk_sel"]
pub type SCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_SEL` writer - reg_sclk_sel"]
pub type SCLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CONF_SPEC, bool, O>;
#[doc = "Field `SCLK_ACTIVE` reader - reg_sclk_active"]
pub type SCLK_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_ACTIVE` writer - reg_sclk_active"]
pub type SCLK_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - reg_sclk_div_num"]
    #[inline(always)]
    pub fn sclk_div_num(&self) -> SCLK_DIV_NUM_R {
        SCLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - reg_sclk_div_a"]
    #[inline(always)]
    pub fn sclk_div_a(&self) -> SCLK_DIV_A_R {
        SCLK_DIV_A_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - reg_sclk_div_b"]
    #[inline(always)]
    pub fn sclk_div_b(&self) -> SCLK_DIV_B_R {
        SCLK_DIV_B_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - reg_sclk_sel"]
    #[inline(always)]
    pub fn sclk_sel(&self) -> SCLK_SEL_R {
        SCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - reg_sclk_active"]
    #[inline(always)]
    pub fn sclk_active(&self) -> SCLK_ACTIVE_R {
        SCLK_ACTIVE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0::CLK_CONF")
            .field(
                "sclk_div_num",
                &format_args!("{}", self.sclk_div_num().bits()),
            )
            .field("sclk_div_a", &format_args!("{}", self.sclk_div_a().bits()))
            .field("sclk_div_b", &format_args!("{}", self.sclk_div_b().bits()))
            .field("sclk_sel", &format_args!("{}", self.sclk_sel().bit()))
            .field("sclk_active", &format_args!("{}", self.sclk_active().bit()))
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - reg_sclk_div_num"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W<0> {
        SCLK_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 8:13 - reg_sclk_div_a"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_a(&mut self) -> SCLK_DIV_A_W<8> {
        SCLK_DIV_A_W::new(self)
    }
    #[doc = "Bits 14:19 - reg_sclk_div_b"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_b(&mut self) -> SCLK_DIV_B_W<14> {
        SCLK_DIV_B_W::new(self)
    }
    #[doc = "Bit 20 - reg_sclk_sel"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_sel(&mut self) -> SCLK_SEL_W<20> {
        SCLK_SEL_W::new(self)
    }
    #[doc = "Bit 21 - reg_sclk_active"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_active(&mut self) -> SCLK_ACTIVE_W<21> {
        SCLK_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_CLK_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf](index.html) module"]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf::R](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf::W](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x0020_0000"]
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0000;
}
