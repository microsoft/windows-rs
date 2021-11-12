#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Payments_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPaymentAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentCanMakePaymentResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentCanMakePaymentResultFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentCurrencyAmount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentCurrencyAmountFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentDetailsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentDetailsModifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentDetailsModifierFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentMediator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentMediator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentMerchantInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentMerchantInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentMethodData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentMethodDataFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentRequest2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentRequestChangedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentRequestChangedResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentRequestChangedResultFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentRequestFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentRequestFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentRequestSubmitResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentShippingOption(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentShippingOptionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentToken(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentTokenFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentCanMakePaymentResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentCanMakePaymentResultStatus(pub i32);
impl PaymentCanMakePaymentResultStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Yes: Self = Self(1i32);
    pub const No: Self = Self(2i32);
    pub const NotAllowed: Self = Self(3i32);
    pub const UserNotSignedIn: Self = Self(4i32);
    pub const SpecifiedPaymentMethodIdsNotSupported: Self = Self(5i32);
    pub const NoQualifyingCardOnFile: Self = Self(6i32);
}
impl ::core::marker::Copy for PaymentCanMakePaymentResultStatus {}
impl ::core::clone::Clone for PaymentCanMakePaymentResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentCurrencyAmount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentDetailsModifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentMediator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentMerchantInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentMethodData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentOptionPresence(pub i32);
impl PaymentOptionPresence {
    pub const None: Self = Self(0i32);
    pub const Optional: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentOptionPresence {}
impl ::core::clone::Clone for PaymentOptionPresence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentRequestChangeKind(pub i32);
impl PaymentRequestChangeKind {
    pub const ShippingOption: Self = Self(0i32);
    pub const ShippingAddress: Self = Self(1i32);
}
impl ::core::marker::Copy for PaymentRequestChangeKind {}
impl ::core::clone::Clone for PaymentRequestChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentRequestChangedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentRequestChangedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentRequestChangedResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentRequestCompletionStatus(pub i32);
impl PaymentRequestCompletionStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentRequestCompletionStatus {}
impl ::core::clone::Clone for PaymentRequestCompletionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentRequestStatus(pub i32);
impl PaymentRequestStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentRequestStatus {}
impl ::core::clone::Clone for PaymentRequestStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentRequestSubmitResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentShippingOption(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentShippingType(pub i32);
impl PaymentShippingType {
    pub const Shipping: Self = Self(0i32);
    pub const Delivery: Self = Self(1i32);
    pub const Pickup: Self = Self(2i32);
}
impl ::core::marker::Copy for PaymentShippingType {}
impl ::core::clone::Clone for PaymentShippingType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentToken(pub *mut ::core::ffi::c_void);
