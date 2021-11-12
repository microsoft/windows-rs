#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPaymentAppCanMakePaymentTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentAppManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentAppManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentTransactionAcceptResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaymentTransactionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentAppCanMakePaymentTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentAppManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentTransactionAcceptResult(pub *mut ::core::ffi::c_void);
