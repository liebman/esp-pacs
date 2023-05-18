#[doc = "Register `EDMA_PMS_AES` reader"]
pub struct R(crate::R<EDMA_PMS_AES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDMA_PMS_AES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDMA_PMS_AES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDMA_PMS_AES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EDMA_PMS_AES` writer"]
pub struct W(crate::W<EDMA_PMS_AES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EDMA_PMS_AES_SPEC>;
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
impl From<crate::W<EDMA_PMS_AES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EDMA_PMS_AES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTR1` reader - This field is used to configure the permission of AES accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR1` writer - This field is used to configure the permission of AES accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EDMA_PMS_AES_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATTR2` reader - This field is used to configure the permission of AES accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR2` writer - This field is used to configure the permission of AES accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EDMA_PMS_AES_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - This field is used to configure the permission of AES accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr1(&self) -> ATTR1_R {
        ATTR1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - This field is used to configure the permission of AES accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr2(&self) -> ATTR2_R {
        ATTR2_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SENSITIVE::EDMA_PMS_AES")
            .field("attr1", &format_args!("{}", self.attr1().bits()))
            .field("attr2", &format_args!("{}", self.attr2().bits()))
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to configure the permission of AES accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    #[must_use]
    pub fn attr1(&mut self) -> ATTR1_W<0> {
        ATTR1_W::new(self)
    }
    #[doc = "Bits 2:3 - This field is used to configure the permission of AES accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    #[must_use]
    pub fn attr2(&mut self) -> ATTR2_W<2> {
        ATTR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EDMA-AES permission control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edma_pms_aes](index.html) module"]
pub struct EDMA_PMS_AES_SPEC;
impl crate::RegisterSpec for EDMA_PMS_AES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edma_pms_aes::R](R) reader structure"]
impl crate::Readable for EDMA_PMS_AES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [edma_pms_aes::W](W) writer structure"]
impl crate::Writable for EDMA_PMS_AES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDMA_PMS_AES to value 0x0f"]
impl crate::Resettable for EDMA_PMS_AES_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
