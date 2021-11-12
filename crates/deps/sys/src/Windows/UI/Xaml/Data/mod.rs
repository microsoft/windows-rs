#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Binding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BindingBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BindingExpression(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BindingExpressionBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BindingMode(pub i32);
impl BindingMode {
    pub const OneWay: Self = Self(1i32);
    pub const OneTime: Self = Self(2i32);
    pub const TwoWay: Self = Self(3i32);
}
#[repr(transparent)]
pub struct BindingOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CollectionViewSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CurrentChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CurrentChangingEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBinding(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBinding2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindingBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindingBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindingExpression(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindingExpressionBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindingExpressionBaseFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindingExpressionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindingFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindingOperations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindingOperationsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICollectionView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICollectionViewFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICollectionViewGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICollectionViewSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICollectionViewSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentChangingEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentChangingEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomPropertyProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemIndexRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemIndexRangeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemsRangeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotifyPropertyChanged(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyChangedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRelativeSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRelativeSourceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISelectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISupportIncrementalLoading(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IValueConverter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ItemIndexRange(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LoadMoreItemsResult(i32);
#[repr(transparent)]
pub struct PropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PropertyChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RelativeSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RelativeSourceMode(pub i32);
impl RelativeSourceMode {
    pub const None: Self = Self(0i32);
    pub const TemplatedParent: Self = Self(1i32);
    pub const Self_: Self = Self(2i32);
}
#[repr(transparent)]
pub struct UpdateSourceTrigger(pub i32);
impl UpdateSourceTrigger {
    pub const Default: Self = Self(0i32);
    pub const PropertyChanged: Self = Self(1i32);
    pub const Explicit: Self = Self(2i32);
    pub const LostFocus: Self = Self(3i32);
}
