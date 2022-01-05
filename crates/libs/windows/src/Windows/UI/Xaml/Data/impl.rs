#[cfg(feature = "implement_exclusive")]
pub trait IBindingImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<super::PropertyPath>;
    fn SetPath(&self, value: &::core::option::Option<super::PropertyPath>) -> ::windows::core::Result<()>;
    fn Mode(&self) -> ::windows::core::Result<BindingMode>;
    fn SetMode(&self, value: BindingMode) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn RelativeSource(&self) -> ::windows::core::Result<RelativeSource>;
    fn SetRelativeSource(&self, value: &::core::option::Option<RelativeSource>) -> ::windows::core::Result<()>;
    fn ElementName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetElementName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Converter(&self) -> ::windows::core::Result<IValueConverter>;
    fn SetConverter(&self, value: &::core::option::Option<IValueConverter>) -> ::windows::core::Result<()>;
    fn ConverterParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetConverterParameter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn ConverterLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetConverterLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBinding2Impl: Sized {
    fn FallbackValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetFallbackValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TargetNullValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTargetNullValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn UpdateSourceTrigger(&self) -> ::windows::core::Result<UpdateSourceTrigger>;
    fn SetUpdateSourceTrigger(&self, value: UpdateSourceTrigger) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BindingBase>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingExpressionImpl: Sized {
    fn DataItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ParentBinding(&self) -> ::windows::core::Result<Binding>;
    fn UpdateSource(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingExpressionBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingExpressionBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingExpressionFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Binding>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingOperationsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingOperationsStaticsImpl: Sized {
    fn SetBinding(&self, target: &::core::option::Option<super::DependencyObject>, dp: &::core::option::Option<super::DependencyProperty>, binding: &::core::option::Option<BindingBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
pub trait ICollectionViewImpl: Sized + IIterableImpl<::windows::core::IInspectable> + IObservableVectorImpl<::windows::core::IInspectable> + IVectorImpl<::windows::core::IInspectable> {
    fn CurrentItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CurrentPosition(&self) -> ::windows::core::Result<i32>;
    fn IsCurrentAfterLast(&self) -> ::windows::core::Result<bool>;
    fn IsCurrentBeforeFirst(&self) -> ::windows::core::Result<bool>;
    fn CollectionGroups(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>;
    fn HasMoreItems(&self) -> ::windows::core::Result<bool>;
    fn CurrentChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentChanging(&self, handler: &::core::option::Option<CurrentChangingEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentChanging(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MoveCurrentTo(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
    fn MoveCurrentToPosition(&self, index: i32) -> ::windows::core::Result<bool>;
    fn MoveCurrentToFirst(&self) -> ::windows::core::Result<bool>;
    fn MoveCurrentToLast(&self) -> ::windows::core::Result<bool>;
    fn MoveCurrentToNext(&self) -> ::windows::core::Result<bool>;
    fn MoveCurrentToPrevious(&self) -> ::windows::core::Result<bool>;
    fn LoadMoreItemsAsync(&self, count: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>>;
}
pub trait ICollectionViewFactoryImpl: Sized {
    fn CreateView(&self) -> ::windows::core::Result<ICollectionView>;
}
pub trait ICollectionViewGroupImpl: Sized {
    fn Group(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GroupItems(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICollectionViewSourceImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSource(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn View(&self) -> ::windows::core::Result<ICollectionView>;
    fn IsSourceGrouped(&self) -> ::windows::core::Result<bool>;
    fn SetIsSourceGrouped(&self, value: bool) -> ::windows::core::Result<()>;
    fn ItemsPath(&self) -> ::windows::core::Result<super::PropertyPath>;
    fn SetItemsPath(&self, value: &::core::option::Option<super::PropertyPath>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICollectionViewSourceStaticsImpl: Sized {
    fn SourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ViewProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsSourceGroupedProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ItemsPathProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentChangingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCancelable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICurrentChangingEventArgsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CurrentChangingEventArgs>;
    fn CreateWithCancelableParameter(&self, iscancelable: bool, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CurrentChangingEventArgs>;
}
pub trait ICustomPropertyImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetValue(&self, target: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, target: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetIndexedValue(&self, target: &::core::option::Option<::windows::core::IInspectable>, index: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetIndexedValue(&self, target: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>, index: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn CanWrite(&self) -> ::windows::core::Result<bool>;
    fn CanRead(&self) -> ::windows::core::Result<bool>;
}
pub trait ICustomPropertyProviderImpl: Sized {
    fn GetCustomProperty(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<ICustomProperty>;
    fn GetIndexedProperty(&self, name: &::windows::core::HSTRING, r#type: &super::Interop::TypeName) -> ::windows::core::Result<ICustomProperty>;
    fn GetStringRepresentation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Type(&self) -> ::windows::core::Result<super::Interop::TypeName>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemIndexRangeImpl: Sized {
    fn FirstIndex(&self) -> ::windows::core::Result<i32>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn LastIndex(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemIndexRangeFactoryImpl: Sized {
    fn CreateInstance(&self, firstindex: i32, length: u32, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemIndexRange>;
}
#[cfg(feature = "Foundation")]
pub trait IItemsRangeInfoImpl: Sized + IClosableImpl {
    fn RangesChanged(&self, visiblerange: &::core::option::Option<ItemIndexRange>, trackeditems: &::core::option::Option<super::super::super::Foundation::Collections::IVectorView<ItemIndexRange>>) -> ::windows::core::Result<()>;
}
pub trait INotifyPropertyChangedImpl: Sized {
    fn PropertyChanged(&self, handler: &::core::option::Option<PropertyChangedEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertyChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyChangedEventArgsImpl: Sized {
    fn PropertyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyChangedEventArgsFactoryImpl: Sized {
    fn CreateInstance(&self, name: &::windows::core::HSTRING, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PropertyChangedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRelativeSourceImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<RelativeSourceMode>;
    fn SetMode(&self, value: RelativeSourceMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRelativeSourceFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RelativeSource>;
}
pub trait ISelectionInfoImpl: Sized {
    fn SelectRange(&self, itemindexrange: &::core::option::Option<ItemIndexRange>) -> ::windows::core::Result<()>;
    fn DeselectRange(&self, itemindexrange: &::core::option::Option<ItemIndexRange>) -> ::windows::core::Result<()>;
    fn IsSelected(&self, index: i32) -> ::windows::core::Result<bool>;
    fn GetSelectedRanges(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ItemIndexRange>>;
}
pub trait ISupportIncrementalLoadingImpl: Sized {
    fn LoadMoreItemsAsync(&self, count: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>>;
    fn HasMoreItems(&self) -> ::windows::core::Result<bool>;
}
pub trait IValueConverterImpl: Sized {
    fn Convert(&self, value: &::core::option::Option<::windows::core::IInspectable>, targettype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>, language: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ConvertBack(&self, value: &::core::option::Option<::windows::core::IInspectable>, targettype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>, language: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
