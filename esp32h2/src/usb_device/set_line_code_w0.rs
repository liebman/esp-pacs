#[doc = "Register `SET_LINE_CODE_W0` reader"]
pub struct R(crate::R<SET_LINE_CODE_W0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SET_LINE_CODE_W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SET_LINE_CODE_W0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SET_LINE_CODE_W0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_DW_DTE_RATE` reader - The value of dwDTERate set by host through SET_LINE_CODING command."]
pub type USB_SERIAL_JTAG_DW_DTE_RATE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The value of dwDTERate set by host through SET_LINE_CODING command."]
    #[inline(always)]
    pub fn usb_serial_jtag_dw_dte_rate(&self) -> USB_SERIAL_JTAG_DW_DTE_RATE_R {
        USB_SERIAL_JTAG_DW_DTE_RATE_R::new(self.bits)
    }
}
#[doc = "W0 of SET_LINE_CODING command.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_line_code_w0](index.html) module"]
pub struct SET_LINE_CODE_W0_SPEC;
impl crate::RegisterSpec for SET_LINE_CODE_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [set_line_code_w0::R](R) reader structure"]
impl crate::Readable for SET_LINE_CODE_W0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SET_LINE_CODE_W0 to value 0"]
impl crate::Resettable for SET_LINE_CODE_W0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
