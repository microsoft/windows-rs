#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPaymentAppCanMakePaymentTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IPaymentAppManager(pub *mut ::core::ffi::c_void);
pub struct IPaymentAppManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IPaymentTransaction(pub *mut ::core::ffi::c_void);
pub struct IPaymentTransactionAcceptResult(pub *mut ::core::ffi::c_void);
pub struct IPaymentTransactionStatics(pub *mut ::core::ffi::c_void);
pub struct PaymentAppCanMakePaymentTriggerDetails(i32);
pub struct PaymentAppManager(i32);
pub struct PaymentTransaction(i32);
pub struct PaymentTransactionAcceptResult(i32);
