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
    pub const Add: NotifyCollectionChangedAction = NotifyCollectionChangedAction(0i32);
    pub const Remove: NotifyCollectionChangedAction = NotifyCollectionChangedAction(1i32);
    pub const Replace: NotifyCollectionChangedAction = NotifyCollectionChangedAction(2i32);
    pub const Move: NotifyCollectionChangedAction = NotifyCollectionChangedAction(3i32);
    pub const Reset: NotifyCollectionChangedAction = NotifyCollectionChangedAction(4i32);
}
#[repr(transparent)]
pub struct NotifyCollectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NotifyCollectionChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TypeKind(pub i32);
impl TypeKind {
    pub const Primitive: TypeKind = TypeKind(0i32);
    pub const Metadata: TypeKind = TypeKind(1i32);
    pub const Custom: TypeKind = TypeKind(2i32);
}
#[repr(C)]
pub struct TypeName(i32);
