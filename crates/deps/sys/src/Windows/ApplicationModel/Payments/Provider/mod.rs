#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPaymentAppCanMakePaymentTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentAppCanMakePaymentTriggerDetails {}
impl ::core::clone::Clone for IPaymentAppCanMakePaymentTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentAppManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentAppManager {}
impl ::core::clone::Clone for IPaymentAppManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentAppManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentAppManagerStatics {}
impl ::core::clone::Clone for IPaymentAppManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentTransaction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentTransaction {}
impl ::core::clone::Clone for IPaymentTransaction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentTransactionAcceptResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentTransactionAcceptResult {}
impl ::core::clone::Clone for IPaymentTransactionAcceptResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaymentTransactionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaymentTransactionStatics {}
impl ::core::clone::Clone for IPaymentTransactionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentAppCanMakePaymentTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentAppCanMakePaymentTriggerDetails {}
impl ::core::clone::Clone for PaymentAppCanMakePaymentTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentAppManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentAppManager {}
impl ::core::clone::Clone for PaymentAppManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentTransaction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentTransaction {}
impl ::core::clone::Clone for PaymentTransaction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentTransactionAcceptResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentTransactionAcceptResult {}
impl ::core::clone::Clone for PaymentTransactionAcceptResult {
    fn clone(&self) -> Self {
        *self
    }
}
