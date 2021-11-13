#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BindableVectorChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BindableVectorChangedEventHandler {}
impl ::core::clone::Clone for BindableVectorChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindableIterable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindableIterable {}
impl ::core::clone::Clone for IBindableIterable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindableIterator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindableIterator {}
impl ::core::clone::Clone for IBindableIterator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindableObservableVector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindableObservableVector {}
impl ::core::clone::Clone for IBindableObservableVector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindableVector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindableVector {}
impl ::core::clone::Clone for IBindableVector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindableVectorView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindableVectorView {}
impl ::core::clone::Clone for IBindableVectorView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotifyCollectionChanged(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotifyCollectionChanged {}
impl ::core::clone::Clone for INotifyCollectionChanged {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotifyCollectionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotifyCollectionChangedEventArgs {}
impl ::core::clone::Clone for INotifyCollectionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotifyCollectionChangedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotifyCollectionChangedEventArgsFactory {}
impl ::core::clone::Clone for INotifyCollectionChangedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotifyCollectionChangedAction(pub i32);
impl NotifyCollectionChangedAction {
    pub const Add: Self = Self(0i32);
    pub const Remove: Self = Self(1i32);
    pub const Replace: Self = Self(2i32);
    pub const Move: Self = Self(3i32);
    pub const Reset: Self = Self(4i32);
}
impl ::core::marker::Copy for NotifyCollectionChangedAction {}
impl ::core::clone::Clone for NotifyCollectionChangedAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotifyCollectionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NotifyCollectionChangedEventArgs {}
impl ::core::clone::Clone for NotifyCollectionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NotifyCollectionChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NotifyCollectionChangedEventHandler {}
impl ::core::clone::Clone for NotifyCollectionChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TypeKind(pub i32);
impl TypeKind {
    pub const Primitive: Self = Self(0i32);
    pub const Metadata: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
}
impl ::core::marker::Copy for TypeKind {}
impl ::core::clone::Clone for TypeKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TypeName {
    pub Name: ::windows_sys::core::HSTRING,
    pub Kind: TypeKind,
}
impl ::core::marker::Copy for TypeName {}
impl ::core::clone::Clone for TypeName {
    fn clone(&self) -> Self {
        *self
    }
}
