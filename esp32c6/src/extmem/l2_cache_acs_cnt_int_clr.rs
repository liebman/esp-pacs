#[doc = "Register `L2_CACHE_ACS_CNT_INT_CLR` reader"]
pub struct R(crate::R<L2_CACHE_ACS_CNT_INT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_ACS_CNT_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_ACS_CNT_INT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_ACS_CNT_INT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_IBUS0_OVF_INT_CLR` reader - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus0 accesses L2-Cache."]
pub type L2_IBUS0_OVF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L2_IBUS1_OVF_INT_CLR` reader - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus1 accesses L2-Cache."]
pub type L2_IBUS1_OVF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L2_IBUS2_OVF_INT_CLR` reader - Reserved"]
pub type L2_IBUS2_OVF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L2_IBUS3_OVF_INT_CLR` reader - Reserved"]
pub type L2_IBUS3_OVF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L2_DBUS0_OVF_INT_CLR` reader - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus0 accesses L2-Cache."]
pub type L2_DBUS0_OVF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L2_DBUS1_OVF_INT_CLR` reader - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus1 accesses L2-Cache."]
pub type L2_DBUS1_OVF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L2_DBUS2_OVF_INT_CLR` reader - Reserved"]
pub type L2_DBUS2_OVF_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `L2_DBUS3_OVF_INT_CLR` reader - Reserved"]
pub type L2_DBUS3_OVF_INT_CLR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 8 - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus0 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_ibus0_ovf_int_clr(&self) -> L2_IBUS0_OVF_INT_CLR_R {
        L2_IBUS0_OVF_INT_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus1 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_ibus1_ovf_int_clr(&self) -> L2_IBUS1_OVF_INT_CLR_R {
        L2_IBUS1_OVF_INT_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l2_ibus2_ovf_int_clr(&self) -> L2_IBUS2_OVF_INT_CLR_R {
        L2_IBUS2_OVF_INT_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn l2_ibus3_ovf_int_clr(&self) -> L2_IBUS3_OVF_INT_CLR_R {
        L2_IBUS3_OVF_INT_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus0 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_dbus0_ovf_int_clr(&self) -> L2_DBUS0_OVF_INT_CLR_R {
        L2_DBUS0_OVF_INT_CLR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The bit is used to clear counters overflow interrupt and counters in L2-Cache due to bus1 accesses L2-Cache."]
    #[inline(always)]
    pub fn l2_dbus1_ovf_int_clr(&self) -> L2_DBUS1_OVF_INT_CLR_R {
        L2_DBUS1_OVF_INT_CLR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn l2_dbus2_ovf_int_clr(&self) -> L2_DBUS2_OVF_INT_CLR_R {
        L2_DBUS2_OVF_INT_CLR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn l2_dbus3_ovf_int_clr(&self) -> L2_DBUS3_OVF_INT_CLR_R {
        L2_DBUS3_OVF_INT_CLR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM::L2_CACHE_ACS_CNT_INT_CLR")
            .field(
                "l2_ibus0_ovf_int_clr",
                &format_args!("{}", self.l2_ibus0_ovf_int_clr().bit()),
            )
            .field(
                "l2_ibus1_ovf_int_clr",
                &format_args!("{}", self.l2_ibus1_ovf_int_clr().bit()),
            )
            .field(
                "l2_ibus2_ovf_int_clr",
                &format_args!("{}", self.l2_ibus2_ovf_int_clr().bit()),
            )
            .field(
                "l2_ibus3_ovf_int_clr",
                &format_args!("{}", self.l2_ibus3_ovf_int_clr().bit()),
            )
            .field(
                "l2_dbus0_ovf_int_clr",
                &format_args!("{}", self.l2_dbus0_ovf_int_clr().bit()),
            )
            .field(
                "l2_dbus1_ovf_int_clr",
                &format_args!("{}", self.l2_dbus1_ovf_int_clr().bit()),
            )
            .field(
                "l2_dbus2_ovf_int_clr",
                &format_args!("{}", self.l2_dbus2_ovf_int_clr().bit()),
            )
            .field(
                "l2_dbus3_ovf_int_clr",
                &format_args!("{}", self.l2_dbus3_ovf_int_clr().bit()),
            )
            .finish()
    }
}
#[doc = "Cache Access Counter Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_acs_cnt_int_clr](index.html) module"]
pub struct L2_CACHE_ACS_CNT_INT_CLR_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_CNT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_acs_cnt_int_clr::R](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_CNT_INT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_CNT_INT_CLR to value 0"]
impl crate::Resettable for L2_CACHE_ACS_CNT_INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
