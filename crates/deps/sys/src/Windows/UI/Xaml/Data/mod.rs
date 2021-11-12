#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Binding(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Binding {}
impl ::core::clone::Clone for Binding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BindingBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BindingBase {}
impl ::core::clone::Clone for BindingBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BindingExpression(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BindingExpression {}
impl ::core::clone::Clone for BindingExpression {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BindingExpressionBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BindingExpressionBase {}
impl ::core::clone::Clone for BindingExpressionBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BindingMode(pub i32);
impl BindingMode {
    pub const OneWay: Self = Self(1i32);
    pub const OneTime: Self = Self(2i32);
    pub const TwoWay: Self = Self(3i32);
}
impl ::core::marker::Copy for BindingMode {}
impl ::core::clone::Clone for BindingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BindingOperations(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BindingOperations {}
impl ::core::clone::Clone for BindingOperations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CollectionViewSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CollectionViewSource {}
impl ::core::clone::Clone for CollectionViewSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CurrentChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CurrentChangingEventArgs {}
impl ::core::clone::Clone for CurrentChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CurrentChangingEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CurrentChangingEventHandler {}
impl ::core::clone::Clone for CurrentChangingEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBinding(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBinding {}
impl ::core::clone::Clone for IBinding {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBinding2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBinding2 {}
impl ::core::clone::Clone for IBinding2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindingBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindingBase {}
impl ::core::clone::Clone for IBindingBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindingBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindingBaseFactory {}
impl ::core::clone::Clone for IBindingBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindingExpression(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindingExpression {}
impl ::core::clone::Clone for IBindingExpression {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindingExpressionBase(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindingExpressionBase {}
impl ::core::clone::Clone for IBindingExpressionBase {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindingExpressionBaseFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindingExpressionBaseFactory {}
impl ::core::clone::Clone for IBindingExpressionBaseFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindingExpressionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindingExpressionFactory {}
impl ::core::clone::Clone for IBindingExpressionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindingFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindingFactory {}
impl ::core::clone::Clone for IBindingFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindingOperations(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindingOperations {}
impl ::core::clone::Clone for IBindingOperations {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBindingOperationsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBindingOperationsStatics {}
impl ::core::clone::Clone for IBindingOperationsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICollectionView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICollectionView {}
impl ::core::clone::Clone for ICollectionView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICollectionViewFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICollectionViewFactory {}
impl ::core::clone::Clone for ICollectionViewFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICollectionViewGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICollectionViewGroup {}
impl ::core::clone::Clone for ICollectionViewGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICollectionViewSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICollectionViewSource {}
impl ::core::clone::Clone for ICollectionViewSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICollectionViewSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICollectionViewSourceStatics {}
impl ::core::clone::Clone for ICollectionViewSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentChangingEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentChangingEventArgs {}
impl ::core::clone::Clone for ICurrentChangingEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentChangingEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentChangingEventArgsFactory {}
impl ::core::clone::Clone for ICurrentChangingEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomProperty {}
impl ::core::clone::Clone for ICustomProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomPropertyProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomPropertyProvider {}
impl ::core::clone::Clone for ICustomPropertyProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemIndexRange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemIndexRange {}
impl ::core::clone::Clone for IItemIndexRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemIndexRangeFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemIndexRangeFactory {}
impl ::core::clone::Clone for IItemIndexRangeFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IItemsRangeInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IItemsRangeInfo {}
impl ::core::clone::Clone for IItemsRangeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INotifyPropertyChanged(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INotifyPropertyChanged {}
impl ::core::clone::Clone for INotifyPropertyChanged {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyChangedEventArgs {}
impl ::core::clone::Clone for IPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertyChangedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertyChangedEventArgsFactory {}
impl ::core::clone::Clone for IPropertyChangedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRelativeSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRelativeSource {}
impl ::core::clone::Clone for IRelativeSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRelativeSourceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRelativeSourceFactory {}
impl ::core::clone::Clone for IRelativeSourceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISelectionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISelectionInfo {}
impl ::core::clone::Clone for ISelectionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISupportIncrementalLoading(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISupportIncrementalLoading {}
impl ::core::clone::Clone for ISupportIncrementalLoading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IValueConverter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IValueConverter {}
impl ::core::clone::Clone for IValueConverter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ItemIndexRange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ItemIndexRange {}
impl ::core::clone::Clone for ItemIndexRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct LoadMoreItemsResult {
    pub Count: u32,
}
impl ::core::marker::Copy for LoadMoreItemsResult {}
impl ::core::clone::Clone for LoadMoreItemsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PropertyChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PropertyChangedEventArgs {}
impl ::core::clone::Clone for PropertyChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PropertyChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PropertyChangedEventHandler {}
impl ::core::clone::Clone for PropertyChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RelativeSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RelativeSource {}
impl ::core::clone::Clone for RelativeSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RelativeSourceMode(pub i32);
impl RelativeSourceMode {
    pub const None: Self = Self(0i32);
    pub const TemplatedParent: Self = Self(1i32);
    pub const Self_: Self = Self(2i32);
}
impl ::core::marker::Copy for RelativeSourceMode {}
impl ::core::clone::Clone for RelativeSourceMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UpdateSourceTrigger(pub i32);
impl UpdateSourceTrigger {
    pub const Default: Self = Self(0i32);
    pub const PropertyChanged: Self = Self(1i32);
    pub const Explicit: Self = Self(2i32);
    pub const LostFocus: Self = Self(3i32);
}
impl ::core::marker::Copy for UpdateSourceTrigger {}
impl ::core::clone::Clone for UpdateSourceTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
