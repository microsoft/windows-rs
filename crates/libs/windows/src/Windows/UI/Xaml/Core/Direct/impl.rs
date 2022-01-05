#[cfg(feature = "implement_exclusive")]
pub trait IXamlDirectImpl: Sized {
    fn GetObject(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetXamlDirectObject(&self, object: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IXamlDirectObject>;
    fn CreateInstance(&self, typeindex: XamlTypeIndex) -> ::windows::core::Result<IXamlDirectObject>;
    fn SetObjectProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SetXamlDirectObjectProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &::core::option::Option<IXamlDirectObject>) -> ::windows::core::Result<()>;
    fn SetBooleanProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: bool) -> ::windows::core::Result<()>;
    fn SetDoubleProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: f64) -> ::windows::core::Result<()>;
    fn SetInt32Property(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: i32) -> ::windows::core::Result<()>;
    fn SetStringProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetDateTimeProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetPointProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetRectProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetSizeProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn SetTimeSpanProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetColorProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn SetCornerRadiusProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::CornerRadius) -> ::windows::core::Result<()>;
    fn SetDurationProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::Duration) -> ::windows::core::Result<()>;
    fn SetGridLengthProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::GridLength) -> ::windows::core::Result<()>;
    fn SetThicknessProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn SetMatrixProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::Media::Matrix) -> ::windows::core::Result<()>;
    fn SetMatrix3DProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: &super::super::Media::Media3D::Matrix3D) -> ::windows::core::Result<()>;
    fn SetEnumProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex, value: u32) -> ::windows::core::Result<()>;
    fn GetObjectProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetXamlDirectObjectProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<IXamlDirectObject>;
    fn GetBooleanProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<bool>;
    fn GetDoubleProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<f64>;
    fn GetInt32Property(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<i32>;
    fn GetStringProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDateTimeProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime>;
    fn GetPointProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn GetRectProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn GetSizeProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::super::Foundation::Size>;
    fn GetTimeSpanProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan>;
    fn GetColorProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::super::Color>;
    fn GetCornerRadiusProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::CornerRadius>;
    fn GetDurationProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::Duration>;
    fn GetGridLengthProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::GridLength>;
    fn GetThicknessProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::Thickness>;
    fn GetMatrixProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::Media::Matrix>;
    fn GetMatrix3DProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<super::super::Media::Media3D::Matrix3D>;
    fn GetEnumProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<u32>;
    fn ClearProperty(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, propertyindex: XamlPropertyIndex) -> ::windows::core::Result<()>;
    fn GetCollectionCount(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>) -> ::windows::core::Result<u32>;
    fn GetXamlDirectObjectFromCollectionAt(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, index: u32) -> ::windows::core::Result<IXamlDirectObject>;
    fn AddToCollection(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, value: &::core::option::Option<IXamlDirectObject>) -> ::windows::core::Result<()>;
    fn InsertIntoCollectionAt(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, index: u32, value: &::core::option::Option<IXamlDirectObject>) -> ::windows::core::Result<()>;
    fn RemoveFromCollection(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, value: &::core::option::Option<IXamlDirectObject>) -> ::windows::core::Result<bool>;
    fn RemoveFromCollectionAt(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, index: u32) -> ::windows::core::Result<()>;
    fn ClearCollection(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>) -> ::windows::core::Result<()>;
    fn AddEventHandler(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, eventindex: XamlEventIndex, handler: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AddEventHandler_HandledEventsToo(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, eventindex: XamlEventIndex, handler: &::core::option::Option<::windows::core::IInspectable>, handledeventstoo: bool) -> ::windows::core::Result<()>;
    fn RemoveEventHandler(&self, xamldirectobject: &::core::option::Option<IXamlDirectObject>, eventindex: XamlEventIndex, handler: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
pub trait IXamlDirectObjectImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlDirectStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<XamlDirect>;
}
