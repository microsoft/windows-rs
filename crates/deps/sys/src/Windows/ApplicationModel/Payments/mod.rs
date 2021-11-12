#![allow(non_snake_case, non_camel_case_types)]
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
pub struct PaymentCanMakePaymentResultStatus(i32);
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
pub struct PaymentOptionPresence(i32);
#[repr(transparent)]
pub struct PaymentOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentRequest(pub *mut ::core::ffi::c_void);
pub struct PaymentRequestChangeKind(i32);
#[repr(transparent)]
pub struct PaymentRequestChangedArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentRequestChangedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentRequestChangedResult(pub *mut ::core::ffi::c_void);
pub struct PaymentRequestCompletionStatus(i32);
pub struct PaymentRequestStatus(i32);
#[repr(transparent)]
pub struct PaymentRequestSubmitResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentShippingOption(pub *mut ::core::ffi::c_void);
pub struct PaymentShippingType(i32);
#[repr(transparent)]
pub struct PaymentToken(pub *mut ::core::ffi::c_void);
