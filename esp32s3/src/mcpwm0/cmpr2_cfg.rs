#[doc = "Register `CMPR2_CFG` reader"]
pub struct R(crate::R<CMPR2_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPR2_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPR2_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPR2_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPR2_CFG` writer"]
pub struct W(crate::W<CMPR2_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPR2_CFG_SPEC>;
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
impl From<crate::W<CMPR2_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPR2_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPR2_A_UPMETHOD` reader - Update method for PWM generator 2 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub type CMPR2_A_UPMETHOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPR2_A_UPMETHOD` writer - Update method for PWM generator 2 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub type CMPR2_A_UPMETHOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMPR2_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `CMPR2_B_UPMETHOD` reader - Update method for PWM generator 2 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub type CMPR2_B_UPMETHOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPR2_B_UPMETHOD` writer - Update method for PWM generator 2 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub type CMPR2_B_UPMETHOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMPR2_CFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `CMPR2_A_SHDW_FULL` reader - Set and reset by hardware. If set, PWM generator 2 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared, A's active reg has been updated with shadow register latest value"]
pub type CMPR2_A_SHDW_FULL_R = crate::BitReader<bool>;
#[doc = "Field `CMPR2_A_SHDW_FULL` writer - Set and reset by hardware. If set, PWM generator 2 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared, A's active reg has been updated with shadow register latest value"]
pub type CMPR2_A_SHDW_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPR2_CFG_SPEC, bool, O>;
#[doc = "Field `CMPR2_B_SHDW_FULL` reader - Set and reset by hardware. If set, PWM generator 2 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared, B's active reg has been updated with shadow register latest value"]
pub type CMPR2_B_SHDW_FULL_R = crate::BitReader<bool>;
#[doc = "Field `CMPR2_B_SHDW_FULL` writer - Set and reset by hardware. If set, PWM generator 2 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared, B's active reg has been updated with shadow register latest value"]
pub type CMPR2_B_SHDW_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPR2_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Update method for PWM generator 2 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    pub fn cmpr2_a_upmethod(&self) -> CMPR2_A_UPMETHOD_R {
        CMPR2_A_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Update method for PWM generator 2 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    pub fn cmpr2_b_upmethod(&self) -> CMPR2_B_UPMETHOD_R {
        CMPR2_B_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Set and reset by hardware. If set, PWM generator 2 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared, A's active reg has been updated with shadow register latest value"]
    #[inline(always)]
    pub fn cmpr2_a_shdw_full(&self) -> CMPR2_A_SHDW_FULL_R {
        CMPR2_A_SHDW_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set and reset by hardware. If set, PWM generator 2 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared, B's active reg has been updated with shadow register latest value"]
    #[inline(always)]
    pub fn cmpr2_b_shdw_full(&self) -> CMPR2_B_SHDW_FULL_R {
        CMPR2_B_SHDW_FULL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update method for PWM generator 2 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_a_upmethod(&mut self) -> CMPR2_A_UPMETHOD_W<0> {
        CMPR2_A_UPMETHOD_W::new(self)
    }
    #[doc = "Bits 4:7 - Update method for PWM generator 2 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_b_upmethod(&mut self) -> CMPR2_B_UPMETHOD_W<4> {
        CMPR2_B_UPMETHOD_W::new(self)
    }
    #[doc = "Bit 8 - Set and reset by hardware. If set, PWM generator 2 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared, A's active reg has been updated with shadow register latest value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_a_shdw_full(&mut self) -> CMPR2_A_SHDW_FULL_W<8> {
        CMPR2_A_SHDW_FULL_W::new(self)
    }
    #[doc = "Bit 9 - Set and reset by hardware. If set, PWM generator 2 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared, B's active reg has been updated with shadow register latest value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr2_b_shdw_full(&mut self) -> CMPR2_B_SHDW_FULL_W<9> {
        CMPR2_B_SHDW_FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer status and update method for time stamp registers A and B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpr2_cfg](index.html) module"]
pub struct CMPR2_CFG_SPEC;
impl crate::RegisterSpec for CMPR2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpr2_cfg::R](R) reader structure"]
impl crate::Readable for CMPR2_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpr2_cfg::W](W) writer structure"]
impl crate::Writable for CMPR2_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPR2_CFG to value 0"]
impl crate::Resettable for CMPR2_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}