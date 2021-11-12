#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct NotifyCollectionChangedAction(pub i32);
impl NotifyCollectionChangedAction {
    pub const Add: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
    pub const Replace: Self = Self(2i32);
    pub const Move: Self = Self(3i32);
    pub const Reset: Self = Self(4i32);
}
#[repr(transparent)]
pub struct NotifyCollectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotifyCollectionChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TypeKind(pub i32);
impl TypeKind {
    pub const Primitive: Self = Self(0i32);
    pub const Metadata: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
}
#[repr(C)]
pub struct TypeName(i32);
