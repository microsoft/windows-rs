#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct BindingMode(i32);
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
#[repr(C)]
pub struct RelativeSourceMode(i32);
#[repr(C)]
pub struct UpdateSourceTrigger(i32);
