#[doc = "Register `PLL` reader"]
pub struct R(crate::R<PLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL` writer"]
pub struct W(crate::W<PLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_SPEC>;
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
impl From<crate::W<PLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCK` reader - Block"]
pub type BLOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLOCK` writer - Block"]
pub type BLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_SPEC, u8, u8, 8, O>;
#[doc = "Field `ADDR` reader - Address"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - Address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA` reader - Data"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRITE` reader - Write"]
pub type WRITE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE` writer - Write"]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - Ready"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - Ready"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Block"]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Write"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Ready"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Block"]
    #[inline(always)]
    pub fn block(&mut self) -> BLOCK_W<0> {
        BLOCK_W::new(self)
    }
    #[doc = "Bits 8:15 - Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<8> {
        ADDR_W::new(self)
    }
    #[doc = "Bits 16:23 - Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<16> {
        DATA_W::new(self)
    }
    #[doc = "Bit 24 - Write"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W<24> {
        WRITE_W::new(self)
    }
    #[doc = "Bit 25 - Ready"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<25> {
        BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL I2C Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll](index.html) module"]
pub struct PLL_SPEC;
impl crate::RegisterSpec for PLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll::R](R) reader structure"]
impl crate::Readable for PLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll::W](W) writer structure"]
impl crate::Writable for PLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL to value 0"]
impl crate::Resettable for PLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
