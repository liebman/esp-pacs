#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_TX_END_INT_CLR[0-3]` writer - Set this bit to clear theCH%s_TX_END_INT interrupt."]
pub type CH_TX_END_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CH_TX_ERR_INT_CLR[0-3]` writer - Set this bit to clear theCH%s_ERR_INT interrupt."]
pub type CH_TX_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CH_TX_THR_EVENT_INT_CLR[0-3]` writer - Set this bit to clear theCH%s_TX_THR_EVENT_INT interrupt."]
pub type CH_TX_THR_EVENT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CH_TX_LOOP_INT_CLR[0-3]` writer - Set this bit to clear theCH%s_TX_LOOP_INT interrupt."]
pub type CH_TX_LOOP_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CH_RX_END_INT_CLR[4-7]` writer - Set this bit to clear theCH4_RX_END_INT interrupt."]
pub type CH_RX_END_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CH_RX_ERR_INT_CLR[RX_CH4,RX_CH5,RX_CH6,RX_CH7]` writer - Set this bit to clear theCH4_ERR_INT interrupt."]
pub type CH_RX_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CH4_RX_THR_EVENT_INT_CLR` writer - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
pub type CH4_RX_THR_EVENT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CH5_RX_THR_EVENT_INT_CLR` writer - Set this bit to clear theCH5_RX_THR_EVENT_INT interrupt."]
pub type CH5_RX_THR_EVENT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CH6_RX_THR_EVENT_INT_CLR` writer - Set this bit to clear theCH6_RX_THR_EVENT_INT interrupt."]
pub type CH6_RX_THR_EVENT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CH7_RX_THR_EVENT_INT_CLR` writer - Set this bit to clear theCH7_RX_THR_EVENT_INT interrupt."]
pub type CH7_RX_THR_EVENT_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL_INT_CLR` writer - Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt."]
pub type TX_CH3_DMA_ACCESS_FAIL_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL_INT_CLR` writer - Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt."]
pub type RX_CH7_DMA_ACCESS_FAIL_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Set this bit to clear theCH[0-3]_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_end_int_clr<const O: u8>(&mut self) -> CH_TX_END_INT_CLR_W<O> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 0 - Set this bit to clear theCH0_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<0> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear theCH1_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<1> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear theCH2_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<2> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear theCH3_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<3> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Set this bit to clear theCH[0-3]_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_err_int_clr<const O: u8>(&mut self) -> CH_TX_ERR_INT_CLR_W<O> {
        CH_TX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear theCH0_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err_int_clr(&mut self) -> CH_TX_ERR_INT_CLR_W<4> {
        CH_TX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear theCH1_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err_int_clr(&mut self) -> CH_TX_ERR_INT_CLR_W<5> {
        CH_TX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear theCH2_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_err_int_clr(&mut self) -> CH_TX_ERR_INT_CLR_W<6> {
        CH_TX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear theCH3_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_err_int_clr(&mut self) -> CH_TX_ERR_INT_CLR_W<7> {
        CH_TX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Set this bit to clear theCH[0-3]_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_thr_event_int_clr<const O: u8>(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<O> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear theCH0_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<8> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear theCH1_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<9> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear theCH2_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<10> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear theCH3_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<11> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Set this bit to clear theCH[0-3]_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_loop_int_clr<const O: u8>(&mut self) -> CH_TX_LOOP_INT_CLR_W<O> {
        CH_TX_LOOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear theCH0_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop_int_clr(&mut self) -> CH_TX_LOOP_INT_CLR_W<12> {
        CH_TX_LOOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear theCH1_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop_int_clr(&mut self) -> CH_TX_LOOP_INT_CLR_W<13> {
        CH_TX_LOOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear theCH2_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop_int_clr(&mut self) -> CH_TX_LOOP_INT_CLR_W<14> {
        CH_TX_LOOP_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to clear theCH3_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop_int_clr(&mut self) -> CH_TX_LOOP_INT_CLR_W<15> {
        CH_TX_LOOP_INT_CLR_W::new(self)
    }
    #[doc = "Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_end_int_clr<const O: u8>(&mut self) -> CH_RX_END_INT_CLR_W<O> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<16> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<17> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<18> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<19> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_err_int_clr<const O: u8>(&mut self) -> CH_RX_ERR_INT_CLR_W<O> {
        CH_RX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ch4_rx_err_int_clr(&mut self) -> CH_RX_ERR_INT_CLR_W<20> {
        CH_RX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ch5_rx_err_int_clr(&mut self) -> CH_RX_ERR_INT_CLR_W<21> {
        CH_RX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ch6_rx_err_int_clr(&mut self) -> CH_RX_ERR_INT_CLR_W<22> {
        CH_RX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ch7_rx_err_int_clr(&mut self) -> CH_RX_ERR_INT_CLR_W<23> {
        CH_RX_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24 - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_thr_event_int_clr(&mut self) -> CH4_RX_THR_EVENT_INT_CLR_W<24> {
        CH4_RX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 25 - Set this bit to clear theCH5_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_thr_event_int_clr(&mut self) -> CH5_RX_THR_EVENT_INT_CLR_W<25> {
        CH5_RX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to clear theCH6_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_thr_event_int_clr(&mut self) -> CH6_RX_THR_EVENT_INT_CLR_W<26> {
        CH6_RX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 27 - Set this bit to clear theCH7_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_thr_event_int_clr(&mut self) -> CH7_RX_THR_EVENT_INT_CLR_W<27> {
        CH7_RX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 28 - Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_dma_access_fail_int_clr(&mut self) -> TX_CH3_DMA_ACCESS_FAIL_INT_CLR_W<28> {
        TX_CH3_DMA_ACCESS_FAIL_INT_CLR_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_dma_access_fail_int_clr(&mut self) -> RX_CH7_DMA_ACCESS_FAIL_INT_CLR_W<29> {
        RX_CH7_DMA_ACCESS_FAIL_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
