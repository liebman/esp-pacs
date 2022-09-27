#[doc = "Register `BRIDGE_CONF` reader"]
pub struct R(crate::R<BRIDGE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRIDGE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRIDGE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRIDGE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRIDGE_CONF` writer"]
pub struct W(crate::W<BRIDGE_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRIDGE_CONF_SPEC>;
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
impl From<crate::W<BRIDGE_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRIDGE_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXEOF_ENA` reader - "]
pub type TXEOF_ENA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXEOF_ENA` writer - "]
pub type TXEOF_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRIDGE_CONF_SPEC, u8, u8, 6, O>;
#[doc = "Field `FIFO_MAP_ENA` reader - "]
pub type FIFO_MAP_ENA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFO_MAP_ENA` writer - "]
pub type FIFO_MAP_ENA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BRIDGE_CONF_SPEC, u8, u8, 4, O>;
#[doc = "Field `SLC0_TX_DUMMY_MODE` reader - "]
pub type SLC0_TX_DUMMY_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SLC0_TX_DUMMY_MODE` writer - "]
pub type SLC0_TX_DUMMY_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BRIDGE_CONF_SPEC, bool, O>;
#[doc = "Field `HDA_MAP_128K` reader - "]
pub type HDA_MAP_128K_R = crate::BitReader<bool>;
#[doc = "Field `HDA_MAP_128K` writer - "]
pub type HDA_MAP_128K_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRIDGE_CONF_SPEC, bool, O>;
#[doc = "Field `SLC1_TX_DUMMY_MODE` reader - "]
pub type SLC1_TX_DUMMY_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SLC1_TX_DUMMY_MODE` writer - "]
pub type SLC1_TX_DUMMY_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BRIDGE_CONF_SPEC, bool, O>;
#[doc = "Field `TX_PUSH_IDLE_NUM` reader - "]
pub type TX_PUSH_IDLE_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX_PUSH_IDLE_NUM` writer - "]
pub type TX_PUSH_IDLE_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BRIDGE_CONF_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn txeof_ena(&self) -> TXEOF_ENA_R {
        TXEOF_ENA_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn fifo_map_ena(&self) -> FIFO_MAP_ENA_R {
        FIFO_MAP_ENA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_tx_dummy_mode(&self) -> SLC0_TX_DUMMY_MODE_R {
        SLC0_TX_DUMMY_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hda_map_128k(&self) -> HDA_MAP_128K_R {
        HDA_MAP_128K_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_tx_dummy_mode(&self) -> SLC1_TX_DUMMY_MODE_R {
        SLC1_TX_DUMMY_MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tx_push_idle_num(&self) -> TX_PUSH_IDLE_NUM_R {
        TX_PUSH_IDLE_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn txeof_ena(&mut self) -> TXEOF_ENA_W<0> {
        TXEOF_ENA_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn fifo_map_ena(&mut self) -> FIFO_MAP_ENA_W<8> {
        FIFO_MAP_ENA_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_tx_dummy_mode(&mut self) -> SLC0_TX_DUMMY_MODE_W<12> {
        SLC0_TX_DUMMY_MODE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hda_map_128k(&mut self) -> HDA_MAP_128K_W<13> {
        HDA_MAP_128K_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_tx_dummy_mode(&mut self) -> SLC1_TX_DUMMY_MODE_W<14> {
        SLC1_TX_DUMMY_MODE_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tx_push_idle_num(&mut self) -> TX_PUSH_IDLE_NUM_W<16> {
        TX_PUSH_IDLE_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bridge_conf](index.html) module"]
pub struct BRIDGE_CONF_SPEC;
impl crate::RegisterSpec for BRIDGE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bridge_conf::R](R) reader structure"]
impl crate::Readable for BRIDGE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bridge_conf::W](W) writer structure"]
impl crate::Writable for BRIDGE_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRIDGE_CONF to value 0x000a_7720"]
impl crate::Resettable for BRIDGE_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000a_7720
    }
}