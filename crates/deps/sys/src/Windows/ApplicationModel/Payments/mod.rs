#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Payments_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPaymentAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentAddress {}
impl ::core::clone::Clone for IPaymentAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentCanMakePaymentResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentCanMakePaymentResult {}
impl ::core::clone::Clone for IPaymentCanMakePaymentResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentCanMakePaymentResultFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentCanMakePaymentResultFactory {}
impl ::core::clone::Clone for IPaymentCanMakePaymentResultFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentCurrencyAmount(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentCurrencyAmount {}
impl ::core::clone::Clone for IPaymentCurrencyAmount {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentCurrencyAmountFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentCurrencyAmountFactory {}
impl ::core::clone::Clone for IPaymentCurrencyAmountFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentDetails {}
impl ::core::clone::Clone for IPaymentDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentDetailsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentDetailsFactory {}
impl ::core::clone::Clone for IPaymentDetailsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentDetailsModifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentDetailsModifier {}
impl ::core::clone::Clone for IPaymentDetailsModifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentDetailsModifierFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentDetailsModifierFactory {}
impl ::core::clone::Clone for IPaymentDetailsModifierFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentItem {}
impl ::core::clone::Clone for IPaymentItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentItemFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentItemFactory {}
impl ::core::clone::Clone for IPaymentItemFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentMediator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentMediator {}
impl ::core::clone::Clone for IPaymentMediator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentMediator2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentMediator2 {}
impl ::core::clone::Clone for IPaymentMediator2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentMerchantInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentMerchantInfo {}
impl ::core::clone::Clone for IPaymentMerchantInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentMerchantInfoFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentMerchantInfoFactory {}
impl ::core::clone::Clone for IPaymentMerchantInfoFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentMethodData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentMethodData {}
impl ::core::clone::Clone for IPaymentMethodData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentMethodDataFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentMethodDataFactory {}
impl ::core::clone::Clone for IPaymentMethodDataFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentOptions {}
impl ::core::clone::Clone for IPaymentOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentRequest {}
impl ::core::clone::Clone for IPaymentRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentRequest2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentRequest2 {}
impl ::core::clone::Clone for IPaymentRequest2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentRequestChangedArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentRequestChangedArgs {}
impl ::core::clone::Clone for IPaymentRequestChangedArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentRequestChangedResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentRequestChangedResult {}
impl ::core::clone::Clone for IPaymentRequestChangedResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentRequestChangedResultFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentRequestChangedResultFactory {}
impl ::core::clone::Clone for IPaymentRequestChangedResultFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentRequestFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentRequestFactory {}
impl ::core::clone::Clone for IPaymentRequestFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentRequestFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentRequestFactory2 {}
impl ::core::clone::Clone for IPaymentRequestFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentRequestSubmitResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentRequestSubmitResult {}
impl ::core::clone::Clone for IPaymentRequestSubmitResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentResponse {}
impl ::core::clone::Clone for IPaymentResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentShippingOption(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentShippingOption {}
impl ::core::clone::Clone for IPaymentShippingOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentShippingOptionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentShippingOptionFactory {}
impl ::core::clone::Clone for IPaymentShippingOptionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentToken(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentToken {}
impl ::core::clone::Clone for IPaymentToken {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentTokenFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentTokenFactory {}
impl ::core::clone::Clone for IPaymentTokenFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentAddress {}
impl ::core::clone::Clone for PaymentAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentCanMakePaymentResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentCanMakePaymentResult {}
impl ::core::clone::Clone for PaymentCanMakePaymentResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PaymentCurrencyAmount {}
impl ::core::clone::Clone for PaymentCurrencyAmount {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentDetails {}
impl ::core::clone::Clone for PaymentDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentDetailsModifier(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentDetailsModifier {}
impl ::core::clone::Clone for PaymentDetailsModifier {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentItem {}
impl ::core::clone::Clone for PaymentItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentMediator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentMediator {}
impl ::core::clone::Clone for PaymentMediator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentMerchantInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentMerchantInfo {}
impl ::core::clone::Clone for PaymentMerchantInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentMethodData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentMethodData {}
impl ::core::clone::Clone for PaymentMethodData {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PaymentOptions {}
impl ::core::clone::Clone for PaymentOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentRequest {}
impl ::core::clone::Clone for PaymentRequest {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PaymentRequestChangedArgs {}
impl ::core::clone::Clone for PaymentRequestChangedArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentRequestChangedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentRequestChangedHandler {}
impl ::core::clone::Clone for PaymentRequestChangedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentRequestChangedResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentRequestChangedResult {}
impl ::core::clone::Clone for PaymentRequestChangedResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PaymentRequestSubmitResult {}
impl ::core::clone::Clone for PaymentRequestSubmitResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentResponse {}
impl ::core::clone::Clone for PaymentResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentShippingOption(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentShippingOption {}
impl ::core::clone::Clone for PaymentShippingOption {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PaymentToken {}
impl ::core::clone::Clone for PaymentToken {
    fn clone(&self) -> Self {
        *self
    }
}
