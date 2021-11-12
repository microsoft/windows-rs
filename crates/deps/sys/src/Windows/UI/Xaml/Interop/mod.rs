#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct BindableVectorChangedEventHandler(pub *mut ::core::ffi::c_void);
pub struct IBindableIterable(pub *mut ::core::ffi::c_void);
pub struct IBindableIterator(pub *mut ::core::ffi::c_void);
pub struct IBindableObservableVector(pub *mut ::core::ffi::c_void);
pub struct IBindableVector(pub *mut ::core::ffi::c_void);
pub struct IBindableVectorView(pub *mut ::core::ffi::c_void);
pub struct INotifyCollectionChanged(pub *mut ::core::ffi::c_void);
pub struct INotifyCollectionChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct INotifyCollectionChangedEventArgsFactory(pub *mut ::core::ffi::c_void);
pub struct NotifyCollectionChangedAction(i32);
pub struct NotifyCollectionChangedEventArgs(i32);
pub struct NotifyCollectionChangedEventHandler(pub *mut ::core::ffi::c_void);
pub struct TypeKind(i32);
pub struct TypeName(i32);
