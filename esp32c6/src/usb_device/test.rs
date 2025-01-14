#[doc = "Register `TEST` reader"]
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST` writer"]
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
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
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_TEST_ENABLE` reader - Enable test of the USB pad"]
pub type USB_SERIAL_JTAG_TEST_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_ENABLE` writer - Enable test of the USB pad"]
pub type USB_SERIAL_JTAG_TEST_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_USB_OE` reader - USB pad oen in test"]
pub type USB_SERIAL_JTAG_TEST_USB_OE_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_USB_OE` writer - USB pad oen in test"]
pub type USB_SERIAL_JTAG_TEST_USB_OE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_TX_DP` reader - USB D+ tx value in test"]
pub type USB_SERIAL_JTAG_TEST_TX_DP_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_TX_DP` writer - USB D+ tx value in test"]
pub type USB_SERIAL_JTAG_TEST_TX_DP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_TX_DM` reader - USB D- tx value in test"]
pub type USB_SERIAL_JTAG_TEST_TX_DM_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_TX_DM` writer - USB D- tx value in test"]
pub type USB_SERIAL_JTAG_TEST_TX_DM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_RX_RCV` reader - USB RCV value in test"]
pub type USB_SERIAL_JTAG_TEST_RX_RCV_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_RX_DP` reader - USB D+ rx value in test"]
pub type USB_SERIAL_JTAG_TEST_RX_DP_R = crate::BitReader<bool>;
#[doc = "Field `USB_SERIAL_JTAG_TEST_RX_DM` reader - USB D- rx value in test"]
pub type USB_SERIAL_JTAG_TEST_RX_DM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Enable test of the USB pad"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_enable(&self) -> USB_SERIAL_JTAG_TEST_ENABLE_R {
        USB_SERIAL_JTAG_TEST_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB pad oen in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_usb_oe(&self) -> USB_SERIAL_JTAG_TEST_USB_OE_R {
        USB_SERIAL_JTAG_TEST_USB_OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB D+ tx value in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_tx_dp(&self) -> USB_SERIAL_JTAG_TEST_TX_DP_R {
        USB_SERIAL_JTAG_TEST_TX_DP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB D- tx value in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_tx_dm(&self) -> USB_SERIAL_JTAG_TEST_TX_DM_R {
        USB_SERIAL_JTAG_TEST_TX_DM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB RCV value in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_rx_rcv(&self) -> USB_SERIAL_JTAG_TEST_RX_RCV_R {
        USB_SERIAL_JTAG_TEST_RX_RCV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB D+ rx value in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_rx_dp(&self) -> USB_SERIAL_JTAG_TEST_RX_DP_R {
        USB_SERIAL_JTAG_TEST_RX_DP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB D- rx value in test"]
    #[inline(always)]
    pub fn usb_serial_jtag_test_rx_dm(&self) -> USB_SERIAL_JTAG_TEST_RX_DM_R {
        USB_SERIAL_JTAG_TEST_RX_DM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable test of the USB pad"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_test_enable(&mut self) -> USB_SERIAL_JTAG_TEST_ENABLE_W<0> {
        USB_SERIAL_JTAG_TEST_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - USB pad oen in test"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_test_usb_oe(&mut self) -> USB_SERIAL_JTAG_TEST_USB_OE_W<1> {
        USB_SERIAL_JTAG_TEST_USB_OE_W::new(self)
    }
    #[doc = "Bit 2 - USB D+ tx value in test"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_test_tx_dp(&mut self) -> USB_SERIAL_JTAG_TEST_TX_DP_W<2> {
        USB_SERIAL_JTAG_TEST_TX_DP_W::new(self)
    }
    #[doc = "Bit 3 - USB D- tx value in test"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_test_tx_dm(&mut self) -> USB_SERIAL_JTAG_TEST_TX_DM_W<3> {
        USB_SERIAL_JTAG_TEST_TX_DM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Registers used for debugging the PHY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](index.html) module"]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test::R](R) reader structure"]
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test::W](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0x30"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
