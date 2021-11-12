#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BindableVectorChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindableIterable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindableIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindableObservableVector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindableVector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindableVectorView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotifyCollectionChanged(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotifyCollectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotifyCollectionChangedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct NotifyCollectionChangedAction(i32);
#[repr(transparent)]
pub struct NotifyCollectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotifyCollectionChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TypeKind(i32);
#[repr(C)]
pub struct TypeName(i32);
