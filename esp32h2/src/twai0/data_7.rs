#[doc = "Register `DATA_7` reader"]
pub struct R(crate::R<DATA_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_7` writer"]
pub struct W(crate::W<DATA_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_7_SPEC>;
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
impl From<crate::W<DATA_7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_7` reader - In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 7 and when software initiate read operation, it is rx data register 7."]
pub type DATA_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_7` writer - In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 7 and when software initiate read operation, it is rx data register 7."]
pub type DATA_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA_7_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 7 and when software initiate read operation, it is rx data register 7."]
    #[inline(always)]
    pub fn data_7(&self) -> DATA_7_R {
        DATA_7_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWAI0::DATA_7")
            .field("data_7", &format_args!("{}", self.data_7().bits()))
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 7 and when software initiate read operation, it is rx data register 7."]
    #[inline(always)]
    #[must_use]
    pub fn data_7(&mut self) -> DATA_7_W<0> {
        DATA_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_7](index.html) module"]
pub struct DATA_7_SPEC;
impl crate::RegisterSpec for DATA_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_7::R](R) reader structure"]
impl crate::Readable for DATA_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_7::W](W) writer structure"]
impl crate::Writable for DATA_7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_7 to value 0"]
impl crate::Resettable for DATA_7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
