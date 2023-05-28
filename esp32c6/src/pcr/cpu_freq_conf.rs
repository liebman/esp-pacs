#[doc = "Register `CPU_FREQ_CONF` reader"]
pub struct R(crate::R<CPU_FREQ_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_FREQ_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_FREQ_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_FREQ_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_FREQ_CONF` writer"]
pub struct W(crate::W<CPU_FREQ_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_FREQ_CONF_SPEC>;
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
impl From<crate::W<CPU_FREQ_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_FREQ_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_LS_DIV_NUM` reader - Set as one within (0,1,3) to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_AHB_LS_DIV_NUM."]
pub type CPU_LS_DIV_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_LS_DIV_NUM` writer - Set as one within (0,1,3) to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_AHB_LS_DIV_NUM."]
pub type CPU_LS_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_FREQ_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `CPU_HS_DIV_NUM` reader - Set as one within (0,1,3) to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for high-speed clock-source such as SPLL, and should be used together with PCR_AHB_HS_DIV_NUM."]
pub type CPU_HS_DIV_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_HS_DIV_NUM` writer - Set as one within (0,1,3) to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for high-speed clock-source such as SPLL, and should be used together with PCR_AHB_HS_DIV_NUM."]
pub type CPU_HS_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_FREQ_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `CPU_HS_120M_FORCE` reader - Given that PCR_CPU_HS_DIV_NUM is 0, set this field as 1 to force clk_cpu at 120MHz. Only avaliable when PCR_CPU_HS_DIV_NUM is 0 and clk_cpu is driven by SPLL."]
pub type CPU_HS_120M_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `CPU_HS_120M_FORCE` writer - Given that PCR_CPU_HS_DIV_NUM is 0, set this field as 1 to force clk_cpu at 120MHz. Only avaliable when PCR_CPU_HS_DIV_NUM is 0 and clk_cpu is driven by SPLL."]
pub type CPU_HS_120M_FORCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_FREQ_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Set as one within (0,1,3) to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_AHB_LS_DIV_NUM."]
    #[inline(always)]
    pub fn cpu_ls_div_num(&self) -> CPU_LS_DIV_NUM_R {
        CPU_LS_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Set as one within (0,1,3) to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for high-speed clock-source such as SPLL, and should be used together with PCR_AHB_HS_DIV_NUM."]
    #[inline(always)]
    pub fn cpu_hs_div_num(&self) -> CPU_HS_DIV_NUM_R {
        CPU_HS_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Given that PCR_CPU_HS_DIV_NUM is 0, set this field as 1 to force clk_cpu at 120MHz. Only avaliable when PCR_CPU_HS_DIV_NUM is 0 and clk_cpu is driven by SPLL."]
    #[inline(always)]
    pub fn cpu_hs_120m_force(&self) -> CPU_HS_120M_FORCE_R {
        CPU_HS_120M_FORCE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_FREQ_CONF")
            .field(
                "cpu_ls_div_num",
                &format_args!("{}", self.cpu_ls_div_num().bits()),
            )
            .field(
                "cpu_hs_div_num",
                &format_args!("{}", self.cpu_hs_div_num().bits()),
            )
            .field(
                "cpu_hs_120m_force",
                &format_args!("{}", self.cpu_hs_120m_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CPU_FREQ_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set as one within (0,1,3) to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_AHB_LS_DIV_NUM."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_ls_div_num(&mut self) -> CPU_LS_DIV_NUM_W<0> {
        CPU_LS_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 8:15 - Set as one within (0,1,3) to generate clk_cpu drived by clk_hproot. The clk_cpu is div1(default)/div2/div4 of clk_hproot. This field is only avaliable for high-speed clock-source such as SPLL, and should be used together with PCR_AHB_HS_DIV_NUM."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_hs_div_num(&mut self) -> CPU_HS_DIV_NUM_W<8> {
        CPU_HS_DIV_NUM_W::new(self)
    }
    #[doc = "Bit 16 - Given that PCR_CPU_HS_DIV_NUM is 0, set this field as 1 to force clk_cpu at 120MHz. Only avaliable when PCR_CPU_HS_DIV_NUM is 0 and clk_cpu is driven by SPLL."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_hs_120m_force(&mut self) -> CPU_HS_120M_FORCE_W<16> {
        CPU_HS_120M_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU_FREQ configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_freq_conf](index.html) module"]
pub struct CPU_FREQ_CONF_SPEC;
impl crate::RegisterSpec for CPU_FREQ_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_freq_conf::R](R) reader structure"]
impl crate::Readable for CPU_FREQ_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_freq_conf::W](W) writer structure"]
impl crate::Writable for CPU_FREQ_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_FREQ_CONF to value 0"]
impl crate::Resettable for CPU_FREQ_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
