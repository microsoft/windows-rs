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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlDirect {
    const NAME: &'static str = "Windows.UI.Xaml.Core.Direct.IXamlDirect";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlDirectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlDirectImpl, const OFFSET: isize>() -> IXamlDirectVtbl {
        unsafe extern "system" fn GetObject<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXamlDirectObject<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXamlDirectObject(&*(&object as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeindex: XamlTypeIndex, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(typeindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetObjectProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetObjectProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetXamlDirectObjectProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXamlDirectObjectProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetBooleanProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBooleanProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, value).into()
        }
        unsafe extern "system" fn SetDoubleProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDoubleProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, value).into()
        }
        unsafe extern "system" fn SetInt32Property<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInt32Property(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, value).into()
        }
        unsafe extern "system" fn SetStringProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStringProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDateTimeProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDateTimeProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetPointProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetRectProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRectProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSizeProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSizeProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetTimeSpanProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeSpanProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetColorProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetCornerRadiusProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCornerRadiusProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::CornerRadius as ::windows::core::Abi>::Abi as *const <super::super::CornerRadius as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDurationProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDurationProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::Duration as ::windows::core::Abi>::Abi as *const <super::super::Duration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetGridLengthProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGridLengthProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::GridLength as ::windows::core::Abi>::Abi as *const <super::super::GridLength as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetThicknessProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThicknessProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMatrixProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Media::Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::Media::Matrix as ::windows::core::Abi>::Abi as *const <super::super::Media::Matrix as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetMatrix3DProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Media::Media3D::Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrix3DProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, &*(&value as *const <super::super::Media::Media3D::Matrix3D as ::windows::core::Abi>::Abi as *const <super::super::Media::Media3D::Matrix3D as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetEnumProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnumProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex, value).into()
        }
        unsafe extern "system" fn GetObjectProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObjectProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXamlDirectObjectProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXamlDirectObjectProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBooleanProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBooleanProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDoubleProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDoubleProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt32Property<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInt32Property(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStringProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStringProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDateTimeProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDateTimeProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPointProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPointProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRectProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRectProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSizeProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSizeProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeSpanProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimeSpanProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCornerRadiusProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCornerRadiusProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDurationProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDurationProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGridLengthProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGridLengthProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThicknessProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThicknessProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatrixProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Media::Matrix) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatrixProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatrix3DProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Media::Media3D::Matrix3D) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatrix3DProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnumProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnumProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearProperty<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, propertyindex: XamlPropertyIndex) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearProperty(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), propertyindex).into()
        }
        unsafe extern "system" fn GetCollectionCount<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollectionCount(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXamlDirectObjectFromCollectionAt<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, index: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXamlDirectObjectFromCollectionAt(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), index) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToCollection<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToCollection(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InsertIntoCollectionAt<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, index: u32, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InsertIntoCollectionAt(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), index, &*(&value as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveFromCollection<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveFromCollection(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFromCollectionAt<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, index: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveFromCollectionAt(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), index).into()
        }
        unsafe extern "system" fn ClearCollection<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearCollection(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddEventHandler<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, eventindex: XamlEventIndex, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEventHandler(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), eventindex, &*(&handler as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddEventHandler_HandledEventsToo<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, eventindex: XamlEventIndex, handler: *mut ::core::ffi::c_void, handledeventstoo: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddEventHandler_HandledEventsToo(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), eventindex, &*(&handler as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), handledeventstoo).into()
        }
        unsafe extern "system" fn RemoveEventHandler<Impl: IXamlDirectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamldirectobject: ::windows::core::RawPtr, eventindex: XamlEventIndex, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEventHandler(&*(&xamldirectobject as *const <IXamlDirectObject as ::windows::core::Abi>::Abi as *const <IXamlDirectObject as ::windows::core::DefaultType>::DefaultType), eventindex, &*(&handler as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXamlDirect>,
            ::windows::core::GetTrustLevel,
            GetObject::<Impl, OFFSET>,
            GetXamlDirectObject::<Impl, OFFSET>,
            CreateInstance::<Impl, OFFSET>,
            SetObjectProperty::<Impl, OFFSET>,
            SetXamlDirectObjectProperty::<Impl, OFFSET>,
            SetBooleanProperty::<Impl, OFFSET>,
            SetDoubleProperty::<Impl, OFFSET>,
            SetInt32Property::<Impl, OFFSET>,
            SetStringProperty::<Impl, OFFSET>,
            SetDateTimeProperty::<Impl, OFFSET>,
            SetPointProperty::<Impl, OFFSET>,
            SetRectProperty::<Impl, OFFSET>,
            SetSizeProperty::<Impl, OFFSET>,
            SetTimeSpanProperty::<Impl, OFFSET>,
            SetColorProperty::<Impl, OFFSET>,
            SetCornerRadiusProperty::<Impl, OFFSET>,
            SetDurationProperty::<Impl, OFFSET>,
            SetGridLengthProperty::<Impl, OFFSET>,
            SetThicknessProperty::<Impl, OFFSET>,
            SetMatrixProperty::<Impl, OFFSET>,
            SetMatrix3DProperty::<Impl, OFFSET>,
            SetEnumProperty::<Impl, OFFSET>,
            GetObjectProperty::<Impl, OFFSET>,
            GetXamlDirectObjectProperty::<Impl, OFFSET>,
            GetBooleanProperty::<Impl, OFFSET>,
            GetDoubleProperty::<Impl, OFFSET>,
            GetInt32Property::<Impl, OFFSET>,
            GetStringProperty::<Impl, OFFSET>,
            GetDateTimeProperty::<Impl, OFFSET>,
            GetPointProperty::<Impl, OFFSET>,
            GetRectProperty::<Impl, OFFSET>,
            GetSizeProperty::<Impl, OFFSET>,
            GetTimeSpanProperty::<Impl, OFFSET>,
            GetColorProperty::<Impl, OFFSET>,
            GetCornerRadiusProperty::<Impl, OFFSET>,
            GetDurationProperty::<Impl, OFFSET>,
            GetGridLengthProperty::<Impl, OFFSET>,
            GetThicknessProperty::<Impl, OFFSET>,
            GetMatrixProperty::<Impl, OFFSET>,
            GetMatrix3DProperty::<Impl, OFFSET>,
            GetEnumProperty::<Impl, OFFSET>,
            ClearProperty::<Impl, OFFSET>,
            GetCollectionCount::<Impl, OFFSET>,
            GetXamlDirectObjectFromCollectionAt::<Impl, OFFSET>,
            AddToCollection::<Impl, OFFSET>,
            InsertIntoCollectionAt::<Impl, OFFSET>,
            RemoveFromCollection::<Impl, OFFSET>,
            RemoveFromCollectionAt::<Impl, OFFSET>,
            ClearCollection::<Impl, OFFSET>,
            AddEventHandler::<Impl, OFFSET>,
            AddEventHandler_HandledEventsToo::<Impl, OFFSET>,
            RemoveEventHandler::<Impl, OFFSET>,
        )
    }
}
pub trait IXamlDirectObjectImpl: Sized {}
impl ::windows::core::RuntimeName for IXamlDirectObject {
    const NAME: &'static str = "Windows.UI.Xaml.Core.Direct.IXamlDirectObject";
}
impl IXamlDirectObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlDirectObjectImpl, const OFFSET: isize>() -> IXamlDirectObjectVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlDirectObject>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlDirectStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<XamlDirect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlDirectStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Core.Direct.IXamlDirectStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlDirectStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlDirectStaticsImpl, const OFFSET: isize>() -> IXamlDirectStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IXamlDirectStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlDirectStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, OFFSET>)
    }
}
