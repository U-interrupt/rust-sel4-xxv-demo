#[doc = "Register `s2mm_length` reader"]
pub struct R(crate::R<S2MM_LENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S2MM_LENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S2MM_LENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S2MM_LENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `s2mm_length` writer"]
pub struct W(crate::W<S2MM_LENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S2MM_LENGTH_SPEC>;
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
impl From<crate::W<S2MM_LENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S2MM_LENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctl_lt_initialize_to_tx` reader - "]
pub type CTL_LT_INITIALIZE_TO_TX_R = crate::BitReader<CTL_LT_INITIALIZE_TO_TX_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTL_LT_INITIALIZE_TO_TX_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<CTL_LT_INITIALIZE_TO_TX_A> for bool {
    #[inline(always)]
    fn from(variant: CTL_LT_INITIALIZE_TO_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl CTL_LT_INITIALIZE_TO_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_LT_INITIALIZE_TO_TX_A {
        match self.bits {
            false => CTL_LT_INITIALIZE_TO_TX_A::DISABLE,
            true => CTL_LT_INITIALIZE_TO_TX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CTL_LT_INITIALIZE_TO_TX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CTL_LT_INITIALIZE_TO_TX_A::ENABLE
    }
}
#[doc = "Field `ctl_lt_initialize_to_tx` writer - "]
pub type CTL_LT_INITIALIZE_TO_TX_W<'a, const O: u8> =
    crate::BitWriter<'a, S2MM_LENGTH_SPEC, O, CTL_LT_INITIALIZE_TO_TX_A>;
impl<'a, const O: u8> CTL_LT_INITIALIZE_TO_TX_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTL_LT_INITIALIZE_TO_TX_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTL_LT_INITIALIZE_TO_TX_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ctl_lt_initialize_to_tx(&self) -> CTL_LT_INITIALIZE_TO_TX_R {
        CTL_LT_INITIALIZE_TO_TX_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ctl_lt_initialize_to_tx(&mut self) -> CTL_LT_INITIALIZE_TO_TX_W<0> {
        CTL_LT_INITIALIZE_TO_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "S2MM Buffer Length (Bytes)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2mm_length](index.html) module"]
pub struct S2MM_LENGTH_SPEC;
impl crate::RegisterSpec for S2MM_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s2mm_length::R](R) reader structure"]
impl crate::Readable for S2MM_LENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s2mm_length::W](W) writer structure"]
impl crate::Writable for S2MM_LENGTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets s2mm_length to value 0"]
impl crate::Resettable for S2MM_LENGTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
