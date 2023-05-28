#[doc = "Register `USER` reader"]
pub struct R(crate::R<USER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER` writer"]
pub struct W(crate::W<USER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER_SPEC>;
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
impl From<crate::W<USER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUTDIN` reader - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
pub type DOUTDIN_R = crate::BitReader<bool>;
#[doc = "Field `DOUTDIN` writer - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
pub type DOUTDIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `QPI_MODE` reader - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
pub type QPI_MODE_R = crate::BitReader<bool>;
#[doc = "Field `QPI_MODE` writer - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
pub type QPI_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `OPI_MODE` reader - Just for master mode. 1: spi controller is in OPI mode (all in 8-bit mode). 0: others. Can be configured in CONF state."]
pub type OPI_MODE_R = crate::BitReader<bool>;
#[doc = "Field `OPI_MODE` writer - Just for master mode. 1: spi controller is in OPI mode (all in 8-bit mode). 0: others. Can be configured in CONF state."]
pub type OPI_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `TSCK_I_EDGE` reader - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
pub type TSCK_I_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `TSCK_I_EDGE` writer - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
pub type TSCK_I_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `CS_HOLD` reader - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type CS_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `CS_HOLD` writer - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type CS_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `CS_SETUP` reader - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type CS_SETUP_R = crate::BitReader<bool>;
#[doc = "Field `CS_SETUP` writer - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
pub type CS_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `RSCK_I_EDGE` reader - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
pub type RSCK_I_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `RSCK_I_EDGE` writer - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
pub type RSCK_I_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `CK_OUT_EDGE` reader - the bit combined with SPI_DOUT_MODE register to set mosi signal delay mode. Can be configured in CONF state."]
pub type CK_OUT_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `CK_OUT_EDGE` writer - the bit combined with SPI_DOUT_MODE register to set mosi signal delay mode. Can be configured in CONF state."]
pub type CK_OUT_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `RD_BYTE_ORDER` reader - In read-data (MISO) phase 1: big-endian 0: little_endian. Can be configured in CONF state."]
pub type RD_BYTE_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `RD_BYTE_ORDER` writer - In read-data (MISO) phase 1: big-endian 0: little_endian. Can be configured in CONF state."]
pub type RD_BYTE_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `WR_BYTE_ORDER` reader - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian. Can be configured in CONF state."]
pub type WR_BYTE_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `WR_BYTE_ORDER` writer - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian. Can be configured in CONF state."]
pub type WR_BYTE_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `FWRITE_DUAL` reader - In the write operations read-data phase is in 2-bit mode. Can be configured in CONF state."]
pub type FWRITE_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `FWRITE_DUAL` writer - In the write operations read-data phase is in 2-bit mode. Can be configured in CONF state."]
pub type FWRITE_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `FWRITE_QUAD` reader - In the write operations read-data phase is in 4-bit mode. Can be configured in CONF state."]
pub type FWRITE_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `FWRITE_QUAD` writer - In the write operations read-data phase is in 4-bit mode. Can be configured in CONF state."]
pub type FWRITE_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `FWRITE_OCT` reader - In the write operations read-data phase is in 8-bit mode. Can be configured in CONF state."]
pub type FWRITE_OCT_R = crate::BitReader<bool>;
#[doc = "Field `FWRITE_OCT` writer - In the write operations read-data phase is in 8-bit mode. Can be configured in CONF state."]
pub type FWRITE_OCT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_CONF_NXT` reader - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
pub type USR_CONF_NXT_R = crate::BitReader<bool>;
#[doc = "Field `USR_CONF_NXT` writer - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
pub type USR_CONF_NXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `SIO` reader - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
pub type SIO_R = crate::BitReader<bool>;
#[doc = "Field `SIO` writer - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
pub type SIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_HOLD_POL` reader - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low. Can be configured in CONF state."]
pub type USR_HOLD_POL_R = crate::BitReader<bool>;
#[doc = "Field `USR_HOLD_POL` writer - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low. Can be configured in CONF state."]
pub type USR_HOLD_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_DOUT_HOLD` reader - spi is hold at data out state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_DOUT_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `USR_DOUT_HOLD` writer - spi is hold at data out state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_DOUT_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_DIN_HOLD` reader - spi is hold at data in state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_DIN_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `USR_DIN_HOLD` writer - spi is hold at data in state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_DIN_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_DUMMY_HOLD` reader - spi is hold at dummy state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_DUMMY_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `USR_DUMMY_HOLD` writer - spi is hold at dummy state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_DUMMY_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_ADDR_HOLD` reader - spi is hold at address state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_ADDR_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `USR_ADDR_HOLD` writer - spi is hold at address state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_ADDR_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_CMD_HOLD` reader - spi is hold at command state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_CMD_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `USR_CMD_HOLD` writer - spi is hold at command state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_CMD_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_PREP_HOLD` reader - spi is hold at prepare state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_PREP_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `USR_PREP_HOLD` writer - spi is hold at prepare state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
pub type USR_PREP_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_MISO_HIGHPART` reader - read-data phase only access to high-part of the buffer SPI_BUF8~SPI_BUF17. 1: enable 0: disable. Can be configured in CONF state."]
pub type USR_MISO_HIGHPART_R = crate::BitReader<bool>;
#[doc = "Field `USR_MISO_HIGHPART` writer - read-data phase only access to high-part of the buffer SPI_BUF8~SPI_BUF17. 1: enable 0: disable. Can be configured in CONF state."]
pub type USR_MISO_HIGHPART_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_MOSI_HIGHPART` reader - write-data phase only access to high-part of the buffer SPI_BUF8~SPI_BUF17. 1: enable 0: disable. Can be configured in CONF state."]
pub type USR_MOSI_HIGHPART_R = crate::BitReader<bool>;
#[doc = "Field `USR_MOSI_HIGHPART` writer - write-data phase only access to high-part of the buffer SPI_BUF8~SPI_BUF17. 1: enable 0: disable. Can be configured in CONF state."]
pub type USR_MOSI_HIGHPART_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_DUMMY_IDLE` reader - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
pub type USR_DUMMY_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `USR_DUMMY_IDLE` writer - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
pub type USR_DUMMY_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_MOSI` reader - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
pub type USR_MOSI_R = crate::BitReader<bool>;
#[doc = "Field `USR_MOSI` writer - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
pub type USR_MOSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_MISO` reader - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
pub type USR_MISO_R = crate::BitReader<bool>;
#[doc = "Field `USR_MISO` writer - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
pub type USR_MISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_DUMMY` reader - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
pub type USR_DUMMY_R = crate::BitReader<bool>;
#[doc = "Field `USR_DUMMY` writer - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
pub type USR_DUMMY_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_ADDR` reader - This bit enable the address phase of an operation. Can be configured in CONF state."]
pub type USR_ADDR_R = crate::BitReader<bool>;
#[doc = "Field `USR_ADDR` writer - This bit enable the address phase of an operation. Can be configured in CONF state."]
pub type USR_ADDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
#[doc = "Field `USR_COMMAND` reader - This bit enable the command phase of an operation. Can be configured in CONF state."]
pub type USR_COMMAND_R = crate::BitReader<bool>;
#[doc = "Field `USR_COMMAND` writer - This bit enable the command phase of an operation. Can be configured in CONF state."]
pub type USR_COMMAND_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn doutdin(&self) -> DOUTDIN_R {
        DOUTDIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn qpi_mode(&self) -> QPI_MODE_R {
        QPI_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Just for master mode. 1: spi controller is in OPI mode (all in 8-bit mode). 0: others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn opi_mode(&self) -> OPI_MODE_R {
        OPI_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
    #[inline(always)]
    pub fn tsck_i_edge(&self) -> TSCK_I_EDGE_R {
        TSCK_I_EDGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold(&self) -> CS_HOLD_R {
        CS_HOLD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup(&self) -> CS_SETUP_R {
        CS_SETUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
    #[inline(always)]
    pub fn rsck_i_edge(&self) -> RSCK_I_EDGE_R {
        RSCK_I_EDGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - the bit combined with SPI_DOUT_MODE register to set mosi signal delay mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CK_OUT_EDGE_R {
        CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - In read-data (MISO) phase 1: big-endian 0: little_endian. Can be configured in CONF state."]
    #[inline(always)]
    pub fn rd_byte_order(&self) -> RD_BYTE_ORDER_R {
        RD_BYTE_ORDER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian. Can be configured in CONF state."]
    #[inline(always)]
    pub fn wr_byte_order(&self) -> WR_BYTE_ORDER_R {
        WR_BYTE_ORDER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - In the write operations read-data phase is in 2-bit mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fwrite_dual(&self) -> FWRITE_DUAL_R {
        FWRITE_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In the write operations read-data phase is in 4-bit mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fwrite_quad(&self) -> FWRITE_QUAD_R {
        FWRITE_QUAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the write operations read-data phase is in 8-bit mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fwrite_oct(&self) -> FWRITE_OCT_R {
        FWRITE_OCT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_conf_nxt(&self) -> USR_CONF_NXT_R {
        USR_CONF_NXT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn sio(&self) -> SIO_R {
        SIO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_hold_pol(&self) -> USR_HOLD_POL_R {
        USR_HOLD_POL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - spi is hold at data out state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dout_hold(&self) -> USR_DOUT_HOLD_R {
        USR_DOUT_HOLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - spi is hold at data in state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_din_hold(&self) -> USR_DIN_HOLD_R {
        USR_DIN_HOLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - spi is hold at dummy state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy_hold(&self) -> USR_DUMMY_HOLD_R {
        USR_DUMMY_HOLD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - spi is hold at address state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_addr_hold(&self) -> USR_ADDR_HOLD_R {
        USR_ADDR_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - spi is hold at command state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_cmd_hold(&self) -> USR_CMD_HOLD_R {
        USR_CMD_HOLD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - spi is hold at prepare state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_prep_hold(&self) -> USR_PREP_HOLD_R {
        USR_PREP_HOLD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer SPI_BUF8~SPI_BUF17. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_miso_highpart(&self) -> USR_MISO_HIGHPART_R {
        USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer SPI_BUF8~SPI_BUF17. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_mosi_highpart(&self) -> USR_MOSI_HIGHPART_R {
        USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> USR_DUMMY_IDLE_R {
        USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_mosi(&self) -> USR_MOSI_R {
        USR_MOSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_miso(&self) -> USR_MISO_R {
        USR_MISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy(&self) -> USR_DUMMY_R {
        USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_addr(&self) -> USR_ADDR_R {
        USR_ADDR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_command(&self) -> USR_COMMAND_R {
        USR_COMMAND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER")
            .field("doutdin", &format_args!("{}", self.doutdin().bit()))
            .field("qpi_mode", &format_args!("{}", self.qpi_mode().bit()))
            .field("opi_mode", &format_args!("{}", self.opi_mode().bit()))
            .field("tsck_i_edge", &format_args!("{}", self.tsck_i_edge().bit()))
            .field("cs_hold", &format_args!("{}", self.cs_hold().bit()))
            .field("cs_setup", &format_args!("{}", self.cs_setup().bit()))
            .field("rsck_i_edge", &format_args!("{}", self.rsck_i_edge().bit()))
            .field("ck_out_edge", &format_args!("{}", self.ck_out_edge().bit()))
            .field(
                "rd_byte_order",
                &format_args!("{}", self.rd_byte_order().bit()),
            )
            .field(
                "wr_byte_order",
                &format_args!("{}", self.wr_byte_order().bit()),
            )
            .field("fwrite_dual", &format_args!("{}", self.fwrite_dual().bit()))
            .field("fwrite_quad", &format_args!("{}", self.fwrite_quad().bit()))
            .field("fwrite_oct", &format_args!("{}", self.fwrite_oct().bit()))
            .field(
                "usr_conf_nxt",
                &format_args!("{}", self.usr_conf_nxt().bit()),
            )
            .field("sio", &format_args!("{}", self.sio().bit()))
            .field(
                "usr_hold_pol",
                &format_args!("{}", self.usr_hold_pol().bit()),
            )
            .field(
                "usr_dout_hold",
                &format_args!("{}", self.usr_dout_hold().bit()),
            )
            .field(
                "usr_din_hold",
                &format_args!("{}", self.usr_din_hold().bit()),
            )
            .field(
                "usr_dummy_hold",
                &format_args!("{}", self.usr_dummy_hold().bit()),
            )
            .field(
                "usr_addr_hold",
                &format_args!("{}", self.usr_addr_hold().bit()),
            )
            .field(
                "usr_cmd_hold",
                &format_args!("{}", self.usr_cmd_hold().bit()),
            )
            .field(
                "usr_prep_hold",
                &format_args!("{}", self.usr_prep_hold().bit()),
            )
            .field(
                "usr_miso_highpart",
                &format_args!("{}", self.usr_miso_highpart().bit()),
            )
            .field(
                "usr_mosi_highpart",
                &format_args!("{}", self.usr_mosi_highpart().bit()),
            )
            .field(
                "usr_dummy_idle",
                &format_args!("{}", self.usr_dummy_idle().bit()),
            )
            .field("usr_mosi", &format_args!("{}", self.usr_mosi().bit()))
            .field("usr_miso", &format_args!("{}", self.usr_miso().bit()))
            .field("usr_dummy", &format_args!("{}", self.usr_dummy().bit()))
            .field("usr_addr", &format_args!("{}", self.usr_addr().bit()))
            .field("usr_command", &format_args!("{}", self.usr_command().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set the bit to enable full duplex communication. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn doutdin(&mut self) -> DOUTDIN_W<0> {
        DOUTDIN_W::new(self)
    }
    #[doc = "Bit 3 - Both for master mode and slave mode. 1: spi controller is in QPI mode. 0: others. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn qpi_mode(&mut self) -> QPI_MODE_W<3> {
        QPI_MODE_W::new(self)
    }
    #[doc = "Bit 4 - Just for master mode. 1: spi controller is in OPI mode (all in 8-bit mode). 0: others. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn opi_mode(&mut self) -> OPI_MODE_W<4> {
        OPI_MODE_W::new(self)
    }
    #[doc = "Bit 5 - In the slave mode, this bit can be used to change the polarity of tsck. 0: tsck = spi_ck_i. 1:tsck = !spi_ck_i."]
    #[inline(always)]
    #[must_use]
    pub fn tsck_i_edge(&mut self) -> TSCK_I_EDGE_W<5> {
        TSCK_I_EDGE_W::new(self)
    }
    #[doc = "Bit 6 - spi cs keep low when spi is in done phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_hold(&mut self) -> CS_HOLD_W<6> {
        CS_HOLD_W::new(self)
    }
    #[doc = "Bit 7 - spi cs is enable when spi is in prepare phase. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_setup(&mut self) -> CS_SETUP_W<7> {
        CS_SETUP_W::new(self)
    }
    #[doc = "Bit 8 - In the slave mode, this bit can be used to change the polarity of rsck. 0: rsck = !spi_ck_i. 1:rsck = spi_ck_i."]
    #[inline(always)]
    #[must_use]
    pub fn rsck_i_edge(&mut self) -> RSCK_I_EDGE_W<8> {
        RSCK_I_EDGE_W::new(self)
    }
    #[doc = "Bit 9 - the bit combined with SPI_DOUT_MODE register to set mosi signal delay mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn ck_out_edge(&mut self) -> CK_OUT_EDGE_W<9> {
        CK_OUT_EDGE_W::new(self)
    }
    #[doc = "Bit 10 - In read-data (MISO) phase 1: big-endian 0: little_endian. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn rd_byte_order(&mut self) -> RD_BYTE_ORDER_W<10> {
        RD_BYTE_ORDER_W::new(self)
    }
    #[doc = "Bit 11 - In command address write-data (MOSI) phases 1: big-endian 0: litte_endian. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn wr_byte_order(&mut self) -> WR_BYTE_ORDER_W<11> {
        WR_BYTE_ORDER_W::new(self)
    }
    #[doc = "Bit 12 - In the write operations read-data phase is in 2-bit mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn fwrite_dual(&mut self) -> FWRITE_DUAL_W<12> {
        FWRITE_DUAL_W::new(self)
    }
    #[doc = "Bit 13 - In the write operations read-data phase is in 4-bit mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn fwrite_quad(&mut self) -> FWRITE_QUAD_W<13> {
        FWRITE_QUAD_W::new(self)
    }
    #[doc = "Bit 14 - In the write operations read-data phase is in 8-bit mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn fwrite_oct(&mut self) -> FWRITE_OCT_W<14> {
        FWRITE_OCT_W::new(self)
    }
    #[doc = "Bit 15 - 1: Enable the DMA CONF phase of next seg-trans operation, which means seg-trans will continue. 0: The seg-trans will end after the current SPI seg-trans or this is not seg-trans mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_conf_nxt(&mut self) -> USR_CONF_NXT_W<15> {
        USR_CONF_NXT_W::new(self)
    }
    #[doc = "Bit 16 - Set the bit to enable 3-line half duplex communication mosi and miso signals share the same pin. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn sio(&mut self) -> SIO_W<16> {
        SIO_W::new(self)
    }
    #[doc = "Bit 17 - It is combined with hold bits to set the polarity of spi hold line 1: spi will be held when spi hold line is high 0: spi will be held when spi hold line is low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_hold_pol(&mut self) -> USR_HOLD_POL_W<17> {
        USR_HOLD_POL_W::new(self)
    }
    #[doc = "Bit 18 - spi is hold at data out state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_dout_hold(&mut self) -> USR_DOUT_HOLD_W<18> {
        USR_DOUT_HOLD_W::new(self)
    }
    #[doc = "Bit 19 - spi is hold at data in state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_din_hold(&mut self) -> USR_DIN_HOLD_W<19> {
        USR_DIN_HOLD_W::new(self)
    }
    #[doc = "Bit 20 - spi is hold at dummy state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_dummy_hold(&mut self) -> USR_DUMMY_HOLD_W<20> {
        USR_DUMMY_HOLD_W::new(self)
    }
    #[doc = "Bit 21 - spi is hold at address state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_addr_hold(&mut self) -> USR_ADDR_HOLD_W<21> {
        USR_ADDR_HOLD_W::new(self)
    }
    #[doc = "Bit 22 - spi is hold at command state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_cmd_hold(&mut self) -> USR_CMD_HOLD_W<22> {
        USR_CMD_HOLD_W::new(self)
    }
    #[doc = "Bit 23 - spi is hold at prepare state the bit are combined with SPI_USR_HOLD_POL bit. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_prep_hold(&mut self) -> USR_PREP_HOLD_W<23> {
        USR_PREP_HOLD_W::new(self)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer SPI_BUF8~SPI_BUF17. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_miso_highpart(&mut self) -> USR_MISO_HIGHPART_W<24> {
        USR_MISO_HIGHPART_W::new(self)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer SPI_BUF8~SPI_BUF17. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_mosi_highpart(&mut self) -> USR_MOSI_HIGHPART_W<25> {
        USR_MOSI_HIGHPART_W::new(self)
    }
    #[doc = "Bit 26 - spi clock is disable in dummy phase when the bit is enable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_dummy_idle(&mut self) -> USR_DUMMY_IDLE_W<26> {
        USR_DUMMY_IDLE_W::new(self)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_mosi(&mut self) -> USR_MOSI_W<27> {
        USR_MOSI_W::new(self)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_miso(&mut self) -> USR_MISO_W<28> {
        USR_MISO_W::new(self)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_dummy(&mut self) -> USR_DUMMY_W<29> {
        USR_DUMMY_W::new(self)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_addr(&mut self) -> USR_ADDR_W<30> {
        USR_ADDR_W::new(self)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn usr_command(&mut self) -> USR_COMMAND_W<31> {
        USR_COMMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI USER control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user](index.html) module"]
pub struct USER_SPEC;
impl crate::RegisterSpec for USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user::R](R) reader structure"]
impl crate::Readable for USER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user::W](W) writer structure"]
impl crate::Writable for USER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USER to value 0x8000_00c0"]
impl crate::Resettable for USER_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_00c0;
}
