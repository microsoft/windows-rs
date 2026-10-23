pub const FSRM_DISPID_CLASSIFICATION_EVENTS: i32 = 90177536;
pub const FSRM_DISPID_CLASSIFICATION_MANAGER: i32 = 89128960;
pub const FSRM_DISPID_CLASSIFICATION_MANAGER2: i32 = 89194496;
pub const FSRM_DISPID_CLASSIFICATION_RULE: i32 = 87097344;
pub const FSRM_DISPID_CLASSIFIER_MODULE_DEFINITION: i32 = 88145920;
pub const FSRM_DISPID_CLASSIFIER_MODULE_IMPLEMENTATION: i32 = 102825984;
pub const FSRM_DISPID_EXPIRATION_RULE: i32 = 87162880;
pub const FSRM_DISPID_PIPELINE_MODULE_CONNECTOR: i32 = 103809024;
pub const FSRM_DISPID_PIPELINE_MODULE_DEFINITION: i32 = 88080384;
pub const FSRM_DISPID_PIPELINE_MODULE_IMPLEMENTATION: i32 = 102760448;
pub const FSRM_DISPID_PROPERTY: i32 = 85983232;
pub const FSRM_DISPID_PROPERTY_BAG: i32 = 101711872;
pub const FSRM_DISPID_PROPERTY_BAG2: i32 = 101777408;
pub const FSRM_DISPID_PROPERTY_DEFINITION: i32 = 84934656;
pub const FSRM_DISPID_PROPERTY_DEFINITION2: i32 = 85000192;
pub const FSRM_DISPID_PROPERTY_DEFINITION_VALUE: i32 = 91226112;
pub const FSRM_DISPID_RULE: i32 = 87031808;
pub const FSRM_DISPID_STORAGE_MODULE_DEFINITION: i32 = 88211456;
pub const FSRM_DISPID_STORAGE_MODULE_IMPLEMENTATION: i32 = 102891520;
#[cfg(feature = "shared")]
pub const FsrmAlwaysModified: super::ULONGLONG = 18446744073709551615;
pub const FsrmMaxNumberPropertyDefinitions: i32 = 100;
#[cfg(feature = "shared")]
pub const FsrmNeverModified: super::ULONGLONG = 0;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmClassificationManager, IFsrmClassificationManager_Vtbl, 0xd2dc89da_ee91_48a0_85d8_cc72a56f7d04);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmClassificationManager {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmClassificationManager, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmClassificationManager {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_ClassificationReportFormats(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClassificationReportFormats)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_ClassificationReportFormats(&self, formats: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClassificationReportFormats)(windows_core::Interface::as_raw(self), formats) }
    }
    pub unsafe fn get_Logging(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Logging)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Logging(&self, logging: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Logging)(windows_core::Interface::as_raw(self), logging) }
    }
    pub unsafe fn get_ClassificationReportMailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClassificationReportMailTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ClassificationReportMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClassificationReportMailTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ClassificationReportEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClassificationReportEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_ClassificationReportEnabled(&self, reportenabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClassificationReportEnabled)(windows_core::Interface::as_raw(self), reportenabled) }
    }
    pub unsafe fn get_ClassificationLastReportPathWithoutExtension(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClassificationLastReportPathWithoutExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ClassificationLastError(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClassificationLastError)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_ClassificationRunningStatus(&self) -> windows_core::Result<super::FsrmReportRunningStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClassificationRunningStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn EnumPropertyDefinitions(&self, options: super::FsrmEnumOptions) -> windows_core::Result<super::IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumPropertyDefinitions)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "fsrm")]
    pub unsafe fn CreatePropertyDefinition(&self) -> windows_core::Result<IFsrmPropertyDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyDefinition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "fsrm")]
    pub unsafe fn GetPropertyDefinition(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<IFsrmPropertyDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn EnumRules(&self, ruletype: super::FsrmRuleType, options: super::FsrmEnumOptions) -> windows_core::Result<super::IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRules)(windows_core::Interface::as_raw(self), ruletype, options, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn CreateRule(&self, ruletype: super::FsrmRuleType) -> windows_core::Result<IFsrmRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRule)(windows_core::Interface::as_raw(self), ruletype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn GetRule(&self, rulename: &windows_core::BSTR, ruletype: super::FsrmRuleType) -> windows_core::Result<IFsrmRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRule)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(rulename), ruletype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: super::FsrmPipelineModuleType, options: super::FsrmEnumOptions) -> windows_core::Result<super::IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumModuleDefinitions)(windows_core::Interface::as_raw(self), moduletype, options, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn CreateModuleDefinition(&self, moduletype: super::FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateModuleDefinition)(windows_core::Interface::as_raw(self), moduletype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn GetModuleDefinition(&self, modulename: &windows_core::BSTR, moduletype: super::FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModuleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(modulename), moduletype, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn RunClassification(&self, context: super::FsrmReportGenerationContext, reserved: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RunClassification)(windows_core::Interface::as_raw(self), context, core::mem::transmute_copy(reserved)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitForClassificationCompletion)(windows_core::Interface::as_raw(self), waitseconds, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CancelClassification(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelClassification)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn EnumFileProperties(&self, filepath: &windows_core::BSTR, options: super::FsrmGetFilePropertyOptions) -> windows_core::Result<super::IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFileProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), options, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn GetFileProperty(&self, filepath: &windows_core::BSTR, propertyname: &windows_core::BSTR, options: super::FsrmGetFilePropertyOptions) -> windows_core::Result<IFsrmProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute_copy(propertyname), options, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFileProperty(&self, filepath: &windows_core::BSTR, propertyname: &windows_core::BSTR, propertyvalue: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute_copy(propertyname), core::mem::transmute_copy(propertyvalue)) }
    }
    pub unsafe fn ClearFileProperty(&self, filepath: &windows_core::BSTR, property: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute_copy(property)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_ClassificationReportFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_ClassificationReportFormats: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_ClassificationReportFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_ClassificationReportFormats: usize,
    pub get_Logging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Logging: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_ClassificationReportMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ClassificationReportMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_ClassificationReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ClassificationReportEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_ClassificationReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_ClassificationReportEnabled: usize,
    pub get_ClassificationLastReportPathWithoutExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ClassificationLastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub get_ClassificationRunningStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmReportRunningStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_ClassificationRunningStatus: usize,
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub EnumPropertyDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrm", feature = "fsrmenums")))]
    EnumPropertyDefinitions: usize,
    #[cfg(feature = "fsrm")]
    pub CreatePropertyDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrm"))]
    CreatePropertyDefinition: usize,
    #[cfg(feature = "fsrm")]
    pub GetPropertyDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrm"))]
    GetPropertyDefinition: usize,
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub EnumRules: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmRuleType, super::FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrm", feature = "fsrmenums")))]
    EnumRules: usize,
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub CreateRule: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmRuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrm", feature = "fsrmenums")))]
    CreateRule: usize,
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub GetRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::FsrmRuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrm", feature = "fsrmenums")))]
    GetRule: usize,
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub EnumModuleDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmPipelineModuleType, super::FsrmEnumOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrm", feature = "fsrmenums")))]
    EnumModuleDefinitions: usize,
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub CreateModuleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmPipelineModuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrm", feature = "fsrmenums")))]
    CreateModuleDefinition: usize,
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub GetModuleDefinition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::FsrmPipelineModuleType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrm", feature = "fsrmenums")))]
    GetModuleDefinition: usize,
    #[cfg(feature = "fsrmenums")]
    pub RunClassification: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmReportGenerationContext, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    RunClassification: usize,
    #[cfg(feature = "wtypes")]
    pub WaitForClassificationCompletion: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    WaitForClassificationCompletion: usize,
    pub CancelClassification: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub EnumFileProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::FsrmGetFilePropertyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrm", feature = "fsrmenums")))]
    EnumFileProperties: usize,
    #[cfg(feature = "fsrmenums")]
    pub GetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::FsrmGetFilePropertyOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    GetFileProperty: usize,
    pub SetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmClassificationManager_Impl: super::IDispatch_Impl {
    fn get_ClassificationReportFormats(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_ClassificationReportFormats(&self, formats: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_Logging(&self) -> windows_core::Result<i32>;
    fn put_Logging(&self, logging: i32) -> windows_core::Result<()>;
    fn get_ClassificationReportMailTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ClassificationReportMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ClassificationReportEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_ClassificationReportEnabled(&self, reportenabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ClassificationLastReportPathWithoutExtension(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ClassificationLastError(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ClassificationRunningStatus(&self) -> windows_core::Result<super::FsrmReportRunningStatus>;
    fn EnumPropertyDefinitions(&self, options: super::FsrmEnumOptions) -> windows_core::Result<super::IFsrmCollection>;
    fn CreatePropertyDefinition(&self) -> windows_core::Result<IFsrmPropertyDefinition>;
    fn GetPropertyDefinition(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<IFsrmPropertyDefinition>;
    fn EnumRules(&self, ruletype: super::FsrmRuleType, options: super::FsrmEnumOptions) -> windows_core::Result<super::IFsrmCollection>;
    fn CreateRule(&self, ruletype: super::FsrmRuleType) -> windows_core::Result<IFsrmRule>;
    fn GetRule(&self, rulename: &windows_core::BSTR, ruletype: super::FsrmRuleType) -> windows_core::Result<IFsrmRule>;
    fn EnumModuleDefinitions(&self, moduletype: super::FsrmPipelineModuleType, options: super::FsrmEnumOptions) -> windows_core::Result<super::IFsrmCollection>;
    fn CreateModuleDefinition(&self, moduletype: super::FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition>;
    fn GetModuleDefinition(&self, modulename: &windows_core::BSTR, moduletype: super::FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition>;
    fn RunClassification(&self, context: super::FsrmReportGenerationContext, reserved: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WaitForClassificationCompletion(&self, waitseconds: i32) -> windows_core::Result<super::VARIANT_BOOL>;
    fn CancelClassification(&self) -> windows_core::Result<()>;
    fn EnumFileProperties(&self, filepath: &windows_core::BSTR, options: super::FsrmGetFilePropertyOptions) -> windows_core::Result<super::IFsrmCollection>;
    fn GetFileProperty(&self, filepath: &windows_core::BSTR, propertyname: &windows_core::BSTR, options: super::FsrmGetFilePropertyOptions) -> windows_core::Result<IFsrmProperty>;
    fn SetFileProperty(&self, filepath: &windows_core::BSTR, propertyname: &windows_core::BSTR, propertyvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClearFileProperty(&self, filepath: &windows_core::BSTR, property: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmClassificationManager_Vtbl {
    pub const fn new<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ClassificationReportFormats<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formats: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::get_ClassificationReportFormats(this) {
                    Ok(ok__) => {
                        formats.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClassificationReportFormats<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formats: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::put_ClassificationReportFormats(this, core::mem::transmute_copy(&formats)).into()
            }
        }
        unsafe extern "system" fn get_Logging<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logging: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::get_Logging(this) {
                    Ok(ok__) => {
                        logging.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Logging<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logging: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::put_Logging(this, core::mem::transmute_copy(&logging)).into()
            }
        }
        unsafe extern "system" fn get_ClassificationReportMailTo<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::get_ClassificationReportMailTo(this) {
                    Ok(ok__) => {
                        mailto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClassificationReportMailTo<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::put_ClassificationReportMailTo(this, core::mem::transmute(&mailto)).into()
            }
        }
        unsafe extern "system" fn get_ClassificationReportEnabled<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::get_ClassificationReportEnabled(this) {
                    Ok(ok__) => {
                        reportenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClassificationReportEnabled<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportenabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::put_ClassificationReportEnabled(this, core::mem::transmute_copy(&reportenabled)).into()
            }
        }
        unsafe extern "system" fn get_ClassificationLastReportPathWithoutExtension<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastreportpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::get_ClassificationLastReportPathWithoutExtension(this) {
                    Ok(ok__) => {
                        lastreportpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ClassificationLastError<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lasterror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::get_ClassificationLastError(this) {
                    Ok(ok__) => {
                        lasterror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ClassificationRunningStatus<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runningstatus: *mut super::FsrmReportRunningStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::get_ClassificationRunningStatus(this) {
                    Ok(ok__) => {
                        runningstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumPropertyDefinitions<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: super::FsrmEnumOptions, propertydefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::EnumPropertyDefinitions(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        propertydefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePropertyDefinition<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertydefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::CreatePropertyDefinition(this) {
                    Ok(ok__) => {
                        propertydefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyDefinition<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, propertydefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::GetPropertyDefinition(this, core::mem::transmute(&propertyname)) {
                    Ok(ok__) => {
                        propertydefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRules<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruletype: super::FsrmRuleType, options: super::FsrmEnumOptions, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::EnumRules(this, core::mem::transmute_copy(&ruletype), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        rules.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRule<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruletype: super::FsrmRuleType, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::CreateRule(this, core::mem::transmute_copy(&ruletype)) {
                    Ok(ok__) => {
                        rule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRule<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulename: *mut core::ffi::c_void, ruletype: super::FsrmRuleType, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::GetRule(this, core::mem::transmute(&rulename), core::mem::transmute_copy(&ruletype)) {
                    Ok(ok__) => {
                        rule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumModuleDefinitions<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduletype: super::FsrmPipelineModuleType, options: super::FsrmEnumOptions, moduledefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::EnumModuleDefinitions(this, core::mem::transmute_copy(&moduletype), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        moduledefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateModuleDefinition<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduletype: super::FsrmPipelineModuleType, moduledefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::CreateModuleDefinition(this, core::mem::transmute_copy(&moduletype)) {
                    Ok(ok__) => {
                        moduledefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetModuleDefinition<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modulename: *mut core::ffi::c_void, moduletype: super::FsrmPipelineModuleType, moduledefinition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::GetModuleDefinition(this, core::mem::transmute(&modulename), core::mem::transmute_copy(&moduletype)) {
                    Ok(ok__) => {
                        moduledefinition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RunClassification<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: super::FsrmReportGenerationContext, reserved: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::RunClassification(this, core::mem::transmute_copy(&context), core::mem::transmute(&reserved)).into()
            }
        }
        unsafe extern "system" fn WaitForClassificationCompletion<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, waitseconds: i32, completed: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::WaitForClassificationCompletion(this, core::mem::transmute_copy(&waitseconds)) {
                    Ok(ok__) => {
                        completed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CancelClassification<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::CancelClassification(this).into()
            }
        }
        unsafe extern "system" fn EnumFileProperties<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, options: super::FsrmGetFilePropertyOptions, fileproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::EnumFileProperties(this, core::mem::transmute(&filepath), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        fileproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileProperty<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, options: super::FsrmGetFilePropertyOptions, property: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::GetFileProperty(this, core::mem::transmute(&filepath), core::mem::transmute(&propertyname), core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        property.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileProperty<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, propertyname: *mut core::ffi::c_void, propertyvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::SetFileProperty(this, core::mem::transmute(&filepath), core::mem::transmute(&propertyname), core::mem::transmute(&propertyvalue)).into()
            }
        }
        unsafe extern "system" fn ClearFileProperty<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepath: *mut core::ffi::c_void, property: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::ClearFileProperty(this, core::mem::transmute(&filepath), core::mem::transmute(&property)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ClassificationReportFormats: get_ClassificationReportFormats::<Identity, OFFSET>,
            put_ClassificationReportFormats: put_ClassificationReportFormats::<Identity, OFFSET>,
            get_Logging: get_Logging::<Identity, OFFSET>,
            put_Logging: put_Logging::<Identity, OFFSET>,
            get_ClassificationReportMailTo: get_ClassificationReportMailTo::<Identity, OFFSET>,
            put_ClassificationReportMailTo: put_ClassificationReportMailTo::<Identity, OFFSET>,
            get_ClassificationReportEnabled: get_ClassificationReportEnabled::<Identity, OFFSET>,
            put_ClassificationReportEnabled: put_ClassificationReportEnabled::<Identity, OFFSET>,
            get_ClassificationLastReportPathWithoutExtension: get_ClassificationLastReportPathWithoutExtension::<Identity, OFFSET>,
            get_ClassificationLastError: get_ClassificationLastError::<Identity, OFFSET>,
            get_ClassificationRunningStatus: get_ClassificationRunningStatus::<Identity, OFFSET>,
            EnumPropertyDefinitions: EnumPropertyDefinitions::<Identity, OFFSET>,
            CreatePropertyDefinition: CreatePropertyDefinition::<Identity, OFFSET>,
            GetPropertyDefinition: GetPropertyDefinition::<Identity, OFFSET>,
            EnumRules: EnumRules::<Identity, OFFSET>,
            CreateRule: CreateRule::<Identity, OFFSET>,
            GetRule: GetRule::<Identity, OFFSET>,
            EnumModuleDefinitions: EnumModuleDefinitions::<Identity, OFFSET>,
            CreateModuleDefinition: CreateModuleDefinition::<Identity, OFFSET>,
            GetModuleDefinition: GetModuleDefinition::<Identity, OFFSET>,
            RunClassification: RunClassification::<Identity, OFFSET>,
            WaitForClassificationCompletion: WaitForClassificationCompletion::<Identity, OFFSET>,
            CancelClassification: CancelClassification::<Identity, OFFSET>,
            EnumFileProperties: EnumFileProperties::<Identity, OFFSET>,
            GetFileProperty: GetFileProperty::<Identity, OFFSET>,
            SetFileProperty: SetFileProperty::<Identity, OFFSET>,
            ClearFileProperty: ClearFileProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassificationManager as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmClassificationManager {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmClassificationManager2, IFsrmClassificationManager2_Vtbl, 0x0004c1c9_127e_4765_ba07_6a3147bca112);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmClassificationManager2 {
    type Target = IFsrmClassificationManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmClassificationManager2, windows_core::IUnknown, super::IDispatch, IFsrmClassificationManager);
#[cfg(feature = "oaidl")]
impl IFsrmClassificationManager2 {
    #[cfg(all(feature = "fsrmenums", feature = "shared", feature = "winnt"))]
    pub unsafe fn ClassifyFiles(&self, filepaths: *const super::SAFEARRAY, propertynames: Option<*const super::SAFEARRAY>, propertyvalues: Option<*const super::SAFEARRAY>, options: super::FsrmGetFilePropertyOptions) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClassifyFiles)(windows_core::Interface::as_raw(self), filepaths, propertynames.unwrap_or(core::mem::zeroed()) as _, propertyvalues.unwrap_or(core::mem::zeroed()) as _, options) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager2_Vtbl {
    pub base__: IFsrmClassificationManager_Vtbl,
    #[cfg(all(feature = "fsrmenums", feature = "shared", feature = "winnt"))]
    pub ClassifyFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY, *const super::SAFEARRAY, *const super::SAFEARRAY, super::FsrmGetFilePropertyOptions) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrmenums", feature = "shared", feature = "winnt")))]
    ClassifyFiles: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmClassificationManager2_Impl: IFsrmClassificationManager_Impl {
    fn ClassifyFiles(&self, filepaths: *const super::SAFEARRAY, propertynames: *const super::SAFEARRAY, propertyvalues: *const super::SAFEARRAY, options: super::FsrmGetFilePropertyOptions) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmClassificationManager2_Vtbl {
    pub const fn new<Identity: IFsrmClassificationManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClassifyFiles<Identity: IFsrmClassificationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepaths: *const super::SAFEARRAY, propertynames: *const super::SAFEARRAY, propertyvalues: *const super::SAFEARRAY, options: super::FsrmGetFilePropertyOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager2_Impl::ClassifyFiles(this, core::mem::transmute_copy(&filepaths), core::mem::transmute_copy(&propertynames), core::mem::transmute_copy(&propertyvalues), core::mem::transmute_copy(&options)).into()
            }
        }
        Self { base__: IFsrmClassificationManager_Vtbl::new::<Identity, OFFSET>(), ClassifyFiles: ClassifyFiles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassificationManager2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmClassificationManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmClassificationManager2 {}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::define_interface!(IFsrmClassificationRule, IFsrmClassificationRule_Vtbl, 0xafc052c2_5315_45ab_841b_c6db0e120148);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl core::ops::Deref for IFsrmClassificationRule {
    type Target = IFsrmRule;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::interface_hierarchy!(IFsrmClassificationRule, windows_core::IUnknown, super::IDispatch, super::IFsrmObject, IFsrmRule);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl IFsrmClassificationRule {
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_ExecutionOption(&self) -> windows_core::Result<super::FsrmExecutionOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExecutionOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn put_ExecutionOption(&self, executionoption: super::FsrmExecutionOption) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExecutionOption)(windows_core::Interface::as_raw(self), executionoption) }
    }
    pub unsafe fn get_PropertyAffected(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertyAffected)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_PropertyAffected(&self, property: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PropertyAffected)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(property)) }
    }
    pub unsafe fn get_Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Value(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Value)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationRule_Vtbl {
    pub base__: IFsrmRule_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub get_ExecutionOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmExecutionOption) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_ExecutionOption: usize,
    #[cfg(feature = "fsrmenums")]
    pub put_ExecutionOption: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmExecutionOption) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    put_ExecutionOption: usize,
    pub get_PropertyAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_PropertyAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmClassificationRule_Impl: IFsrmRule_Impl {
    fn get_ExecutionOption(&self) -> windows_core::Result<super::FsrmExecutionOption>;
    fn put_ExecutionOption(&self, executionoption: super::FsrmExecutionOption) -> windows_core::Result<()>;
    fn get_PropertyAffected(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_PropertyAffected(&self, property: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Value(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmClassificationRule_Vtbl {
    pub const fn new<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ExecutionOption<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executionoption: *mut super::FsrmExecutionOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationRule_Impl::get_ExecutionOption(this) {
                    Ok(ok__) => {
                        executionoption.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExecutionOption<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executionoption: super::FsrmExecutionOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationRule_Impl::put_ExecutionOption(this, core::mem::transmute_copy(&executionoption)).into()
            }
        }
        unsafe extern "system" fn get_PropertyAffected<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationRule_Impl::get_PropertyAffected(this) {
                    Ok(ok__) => {
                        property.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PropertyAffected<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationRule_Impl::put_PropertyAffected(this, core::mem::transmute(&property)).into()
            }
        }
        unsafe extern "system" fn get_Value<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationRule_Impl::get_Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Value<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationRule_Impl::put_Value(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: IFsrmRule_Vtbl::new::<Identity, OFFSET>(),
            get_ExecutionOption: get_ExecutionOption::<Identity, OFFSET>,
            put_ExecutionOption: put_ExecutionOption::<Identity, OFFSET>,
            get_PropertyAffected: get_PropertyAffected::<Identity, OFFSET>,
            put_PropertyAffected: put_PropertyAffected::<Identity, OFFSET>,
            get_Value: get_Value::<Identity, OFFSET>,
            put_Value: put_Value::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassificationRule as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmRule as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmClassificationRule {}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::define_interface!(IFsrmClassifierModuleDefinition, IFsrmClassifierModuleDefinition_Vtbl, 0xbb36ea26_6318_4b8c_8592_f72dd602e7a5);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl core::ops::Deref for IFsrmClassifierModuleDefinition {
    type Target = IFsrmPipelineModuleDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::interface_hierarchy!(IFsrmClassifierModuleDefinition, windows_core::IUnknown, super::IDispatch, super::IFsrmObject, IFsrmPipelineModuleDefinition);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl IFsrmClassifierModuleDefinition {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_PropertiesAffected(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertiesAffected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_PropertiesAffected(&self, propertiesaffected: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PropertiesAffected)(windows_core::Interface::as_raw(self), propertiesaffected) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_PropertiesUsed(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertiesUsed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_PropertiesUsed(&self, propertiesused: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PropertiesUsed)(windows_core::Interface::as_raw(self), propertiesused) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_NeedsExplicitValue(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NeedsExplicitValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_NeedsExplicitValue(&self, needsexplicitvalue: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NeedsExplicitValue)(windows_core::Interface::as_raw(self), needsexplicitvalue) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_PropertiesAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_PropertiesAffected: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_PropertiesAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_PropertiesAffected: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_PropertiesUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_PropertiesUsed: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_PropertiesUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_PropertiesUsed: usize,
    #[cfg(feature = "wtypes")]
    pub get_NeedsExplicitValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_NeedsExplicitValue: usize,
    #[cfg(feature = "wtypes")]
    pub put_NeedsExplicitValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_NeedsExplicitValue: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmClassifierModuleDefinition_Impl: IFsrmPipelineModuleDefinition_Impl {
    fn get_PropertiesAffected(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_PropertiesAffected(&self, propertiesaffected: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_PropertiesUsed(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_PropertiesUsed(&self, propertiesused: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_NeedsExplicitValue(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_NeedsExplicitValue(&self, needsexplicitvalue: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmClassifierModuleDefinition_Vtbl {
    pub const fn new<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_PropertiesAffected<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesaffected: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleDefinition_Impl::get_PropertiesAffected(this) {
                    Ok(ok__) => {
                        propertiesaffected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PropertiesAffected<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesaffected: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleDefinition_Impl::put_PropertiesAffected(this, core::mem::transmute_copy(&propertiesaffected)).into()
            }
        }
        unsafe extern "system" fn get_PropertiesUsed<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesused: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleDefinition_Impl::get_PropertiesUsed(this) {
                    Ok(ok__) => {
                        propertiesused.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PropertiesUsed<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesused: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleDefinition_Impl::put_PropertiesUsed(this, core::mem::transmute_copy(&propertiesused)).into()
            }
        }
        unsafe extern "system" fn get_NeedsExplicitValue<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsexplicitvalue: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleDefinition_Impl::get_NeedsExplicitValue(this) {
                    Ok(ok__) => {
                        needsexplicitvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NeedsExplicitValue<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsexplicitvalue: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleDefinition_Impl::put_NeedsExplicitValue(this, core::mem::transmute_copy(&needsexplicitvalue)).into()
            }
        }
        Self {
            base__: IFsrmPipelineModuleDefinition_Vtbl::new::<Identity, OFFSET>(),
            get_PropertiesAffected: get_PropertiesAffected::<Identity, OFFSET>,
            put_PropertiesAffected: put_PropertiesAffected::<Identity, OFFSET>,
            get_PropertiesUsed: get_PropertiesUsed::<Identity, OFFSET>,
            put_PropertiesUsed: put_PropertiesUsed::<Identity, OFFSET>,
            get_NeedsExplicitValue: get_NeedsExplicitValue::<Identity, OFFSET>,
            put_NeedsExplicitValue: put_NeedsExplicitValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassifierModuleDefinition as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmPipelineModuleDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmClassifierModuleDefinition {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmClassifierModuleImplementation, IFsrmClassifierModuleImplementation_Vtbl, 0x4c968fc6_6edb_4051_9c18_73b7291ae106);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmClassifierModuleImplementation {
    type Target = IFsrmPipelineModuleImplementation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmClassifierModuleImplementation, windows_core::IUnknown, super::IDispatch, IFsrmPipelineModuleImplementation);
#[cfg(feature = "oaidl")]
impl IFsrmClassifierModuleImplementation {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_LastModified(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "fsrm")]
    pub unsafe fn UseRulesAndDefinitions<P0, P1>(&self, rules: P0, propertydefinitions: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IFsrmCollection>,
        P1: windows_core::Param<super::IFsrmCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).UseRulesAndDefinitions)(windows_core::Interface::as_raw(self), rules.param().abi(), propertydefinitions.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn OnBeginFile<P0>(&self, propertybag: P0, arrayruleids: *const super::SAFEARRAY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFsrmPropertyBag>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnBeginFile)(windows_core::Interface::as_raw(self), propertybag.param().abi(), arrayruleids) }
    }
    #[cfg(all(feature = "fsrmenums", feature = "wtypes"))]
    pub unsafe fn DoesPropertyValueApply(&self, property: &windows_core::BSTR, value: &windows_core::BSTR, applyvalue: *mut super::VARIANT_BOOL, idrule: super::FSRM_OBJECT_ID, idpropdef: super::FSRM_OBJECT_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoesPropertyValueApply)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(property), core::mem::transmute_copy(value), applyvalue as _, idrule, idpropdef) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn GetPropertyValueToApply(&self, property: &windows_core::BSTR, value: *mut windows_core::BSTR, idrule: super::FSRM_OBJECT_ID, idpropdef: super::FSRM_OBJECT_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyValueToApply)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(property), core::mem::transmute(value), idrule, idpropdef) }
    }
    pub unsafe fn OnEndFile(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnEndFile)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleImplementation_Vtbl {
    pub base__: IFsrmPipelineModuleImplementation_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_LastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_LastModified: usize,
    #[cfg(feature = "fsrm")]
    pub UseRulesAndDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrm"))]
    UseRulesAndDefinitions: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub OnBeginFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    OnBeginFile: usize,
    #[cfg(all(feature = "fsrmenums", feature = "wtypes"))]
    pub DoesPropertyValueApply: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL, super::FSRM_OBJECT_ID, super::FSRM_OBJECT_ID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrmenums", feature = "wtypes")))]
    DoesPropertyValueApply: usize,
    #[cfg(feature = "fsrmenums")]
    pub GetPropertyValueToApply: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, super::FSRM_OBJECT_ID, super::FSRM_OBJECT_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    GetPropertyValueToApply: usize,
    pub OnEndFile: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmClassifierModuleImplementation_Impl: IFsrmPipelineModuleImplementation_Impl {
    fn get_LastModified(&self) -> windows_core::Result<super::VARIANT>;
    fn UseRulesAndDefinitions(&self, rules: windows_core::Ref<super::IFsrmCollection>, propertydefinitions: windows_core::Ref<super::IFsrmCollection>) -> windows_core::Result<()>;
    fn OnBeginFile(&self, propertybag: windows_core::Ref<IFsrmPropertyBag>, arrayruleids: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn DoesPropertyValueApply(&self, property: &windows_core::BSTR, value: &windows_core::BSTR, applyvalue: *mut super::VARIANT_BOOL, idrule: &super::FSRM_OBJECT_ID, idpropdef: &super::FSRM_OBJECT_ID) -> windows_core::Result<()>;
    fn GetPropertyValueToApply(&self, property: &windows_core::BSTR, value: *mut windows_core::BSTR, idrule: &super::FSRM_OBJECT_ID, idpropdef: &super::FSRM_OBJECT_ID) -> windows_core::Result<()>;
    fn OnEndFile(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmClassifierModuleImplementation_Vtbl {
    pub const fn new<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_LastModified<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodified: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleImplementation_Impl::get_LastModified(this) {
                    Ok(ok__) => {
                        lastmodified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UseRulesAndDefinitions<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut core::ffi::c_void, propertydefinitions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleImplementation_Impl::UseRulesAndDefinitions(this, core::mem::transmute_copy(&rules), core::mem::transmute_copy(&propertydefinitions)).into()
            }
        }
        unsafe extern "system" fn OnBeginFile<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertybag: *mut core::ffi::c_void, arrayruleids: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleImplementation_Impl::OnBeginFile(this, core::mem::transmute_copy(&propertybag), core::mem::transmute_copy(&arrayruleids)).into()
            }
        }
        unsafe extern "system" fn DoesPropertyValueApply<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut core::ffi::c_void, value: *mut core::ffi::c_void, applyvalue: *mut super::VARIANT_BOOL, idrule: super::FSRM_OBJECT_ID, idpropdef: super::FSRM_OBJECT_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleImplementation_Impl::DoesPropertyValueApply(this, core::mem::transmute(&property), core::mem::transmute(&value), core::mem::transmute_copy(&applyvalue), core::mem::transmute(&idrule), core::mem::transmute(&idpropdef)).into()
            }
        }
        unsafe extern "system" fn GetPropertyValueToApply<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void, idrule: super::FSRM_OBJECT_ID, idpropdef: super::FSRM_OBJECT_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleImplementation_Impl::GetPropertyValueToApply(this, core::mem::transmute(&property), core::mem::transmute_copy(&value), core::mem::transmute(&idrule), core::mem::transmute(&idpropdef)).into()
            }
        }
        unsafe extern "system" fn OnEndFile<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleImplementation_Impl::OnEndFile(this).into()
            }
        }
        Self {
            base__: IFsrmPipelineModuleImplementation_Vtbl::new::<Identity, OFFSET>(),
            get_LastModified: get_LastModified::<Identity, OFFSET>,
            UseRulesAndDefinitions: UseRulesAndDefinitions::<Identity, OFFSET>,
            OnBeginFile: OnBeginFile::<Identity, OFFSET>,
            DoesPropertyValueApply: DoesPropertyValueApply::<Identity, OFFSET>,
            GetPropertyValueToApply: GetPropertyValueToApply::<Identity, OFFSET>,
            OnEndFile: OnEndFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassifierModuleImplementation as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmPipelineModuleImplementation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmClassifierModuleImplementation {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmPipelineModuleConnector, IFsrmPipelineModuleConnector_Vtbl, 0xc16014f3_9aa1_46b3_b0a7_ab146eb205f2);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmPipelineModuleConnector {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmPipelineModuleConnector, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmPipelineModuleConnector {
    pub unsafe fn get_ModuleImplementation(&self) -> windows_core::Result<IFsrmPipelineModuleImplementation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ModuleImplementation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ModuleName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ModuleName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_HostingUserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HostingUserAccount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_HostingProcessPid(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HostingProcessPid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrm")]
    pub unsafe fn Bind<P0, P1>(&self, moduledefinition: P0, moduleimplementation: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFsrmPipelineModuleDefinition>,
        P1: windows_core::Param<IFsrmPipelineModuleImplementation>,
    {
        unsafe { (windows_core::Interface::vtable(self).Bind)(windows_core::Interface::as_raw(self), moduledefinition.param().abi(), moduleimplementation.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleConnector_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_ModuleImplementation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ModuleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_HostingUserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_HostingProcessPid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "fsrm")]
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrm"))]
    Bind: usize,
}
#[cfg(all(feature = "fsrm", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPipelineModuleConnector_Impl: super::IDispatch_Impl {
    fn get_ModuleImplementation(&self) -> windows_core::Result<IFsrmPipelineModuleImplementation>;
    fn get_ModuleName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_HostingUserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_HostingProcessPid(&self) -> windows_core::Result<i32>;
    fn Bind(&self, moduledefinition: windows_core::Ref<IFsrmPipelineModuleDefinition>, moduleimplementation: windows_core::Ref<IFsrmPipelineModuleImplementation>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPipelineModuleConnector_Vtbl {
    pub const fn new<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ModuleImplementation<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipelinemoduleimplementation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::get_ModuleImplementation(this) {
                    Ok(ok__) => {
                        pipelinemoduleimplementation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ModuleName<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::get_ModuleName(this) {
                    Ok(ok__) => {
                        username.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HostingUserAccount<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useraccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::get_HostingUserAccount(this) {
                    Ok(ok__) => {
                        useraccount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HostingProcessPid<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::get_HostingProcessPid(this) {
                    Ok(ok__) => {
                        pid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Bind<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduledefinition: *mut core::ffi::c_void, moduleimplementation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleConnector_Impl::Bind(this, core::mem::transmute_copy(&moduledefinition), core::mem::transmute_copy(&moduleimplementation)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ModuleImplementation: get_ModuleImplementation::<Identity, OFFSET>,
            get_ModuleName: get_ModuleName::<Identity, OFFSET>,
            get_HostingUserAccount: get_HostingUserAccount::<Identity, OFFSET>,
            get_HostingProcessPid: get_HostingProcessPid::<Identity, OFFSET>,
            Bind: Bind::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleConnector as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmPipelineModuleConnector {}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::define_interface!(IFsrmPipelineModuleDefinition, IFsrmPipelineModuleDefinition_Vtbl, 0x515c1277_2c81_440e_8fcf_367921ed4f59);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl core::ops::Deref for IFsrmPipelineModuleDefinition {
    type Target = super::IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::interface_hierarchy!(IFsrmPipelineModuleDefinition, windows_core::IUnknown, super::IDispatch, super::IFsrmObject);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl IFsrmPipelineModuleDefinition {
    pub unsafe fn get_ModuleClsid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ModuleClsid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ModuleClsid(&self, moduleclsid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ModuleClsid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(moduleclsid)) }
    }
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn get_Company(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Company)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Company(&self, company: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Company)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(company)) }
    }
    pub unsafe fn get_Version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Version(&self, version: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Version)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(version)) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_ModuleType(&self) -> windows_core::Result<super::FsrmPipelineModuleType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ModuleType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Enabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_NeedsFileContent(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NeedsFileContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_NeedsFileContent(&self, needsfilecontent: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NeedsFileContent)(windows_core::Interface::as_raw(self), needsfilecontent) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_Account(&self) -> windows_core::Result<super::FsrmAccountType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Account)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn put_Account(&self, retrievalaccount: super::FsrmAccountType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Account)(windows_core::Interface::as_raw(self), retrievalaccount) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedExtensions(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedExtensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_SupportedExtensions(&self, supportedextensions: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SupportedExtensions)(windows_core::Interface::as_raw(self), supportedextensions) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_Parameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Parameters)(windows_core::Interface::as_raw(self), parameters) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleDefinition_Vtbl {
    pub base__: super::IFsrmObject_Vtbl,
    pub get_ModuleClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ModuleClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Company: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Company: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub get_ModuleType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmPipelineModuleType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_ModuleType: usize,
    #[cfg(feature = "wtypes")]
    pub get_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_NeedsFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_NeedsFileContent: usize,
    #[cfg(feature = "wtypes")]
    pub put_NeedsFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_NeedsFileContent: usize,
    #[cfg(feature = "fsrmenums")]
    pub get_Account: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmAccountType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_Account: usize,
    #[cfg(feature = "fsrmenums")]
    pub put_Account: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmAccountType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    put_Account: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedExtensions: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_SupportedExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_SupportedExtensions: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_Parameters: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_Parameters: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPipelineModuleDefinition_Impl: super::IFsrmObject_Impl {
    fn get_ModuleClsid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ModuleClsid(&self, moduleclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Company(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Company(&self, company: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Version(&self, version: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ModuleType(&self) -> windows_core::Result<super::FsrmPipelineModuleType>;
    fn get_Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_Enabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_NeedsFileContent(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_NeedsFileContent(&self, needsfilecontent: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_Account(&self) -> windows_core::Result<super::FsrmAccountType>;
    fn put_Account(&self, retrievalaccount: super::FsrmAccountType) -> windows_core::Result<()>;
    fn get_SupportedExtensions(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_SupportedExtensions(&self, supportedextensions: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_Parameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPipelineModuleDefinition_Vtbl {
    pub const fn new<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ModuleClsid<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleclsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::get_ModuleClsid(this) {
                    Ok(ok__) => {
                        moduleclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ModuleClsid<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleclsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::put_ModuleClsid(this, core::mem::transmute(&moduleclsid)).into()
            }
        }
        unsafe extern "system" fn get_Name<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Name<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::put_Name(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_Company<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, company: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::get_Company(this) {
                    Ok(ok__) => {
                        company.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Company<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, company: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::put_Company(this, core::mem::transmute(&company)).into()
            }
        }
        unsafe extern "system" fn get_Version<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::get_Version(this) {
                    Ok(ok__) => {
                        version.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Version<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::put_Version(this, core::mem::transmute(&version)).into()
            }
        }
        unsafe extern "system" fn get_ModuleType<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduletype: *mut super::FsrmPipelineModuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::get_ModuleType(this) {
                    Ok(ok__) => {
                        moduletype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Enabled<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::get_Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Enabled<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::put_Enabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn get_NeedsFileContent<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsfilecontent: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::get_NeedsFileContent(this) {
                    Ok(ok__) => {
                        needsfilecontent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NeedsFileContent<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsfilecontent: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::put_NeedsFileContent(this, core::mem::transmute_copy(&needsfilecontent)).into()
            }
        }
        unsafe extern "system" fn get_Account<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retrievalaccount: *mut super::FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::get_Account(this) {
                    Ok(ok__) => {
                        retrievalaccount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Account<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retrievalaccount: super::FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::put_Account(this, core::mem::transmute_copy(&retrievalaccount)).into()
            }
        }
        unsafe extern "system" fn get_SupportedExtensions<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedextensions: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::get_SupportedExtensions(this) {
                    Ok(ok__) => {
                        supportedextensions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SupportedExtensions<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedextensions: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::put_SupportedExtensions(this, core::mem::transmute_copy(&supportedextensions)).into()
            }
        }
        unsafe extern "system" fn get_Parameters<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::get_Parameters(this) {
                    Ok(ok__) => {
                        parameters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Parameters<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::put_Parameters(this, core::mem::transmute_copy(&parameters)).into()
            }
        }
        Self {
            base__: super::IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            get_ModuleClsid: get_ModuleClsid::<Identity, OFFSET>,
            put_ModuleClsid: put_ModuleClsid::<Identity, OFFSET>,
            get_Name: get_Name::<Identity, OFFSET>,
            put_Name: put_Name::<Identity, OFFSET>,
            get_Company: get_Company::<Identity, OFFSET>,
            put_Company: put_Company::<Identity, OFFSET>,
            get_Version: get_Version::<Identity, OFFSET>,
            put_Version: put_Version::<Identity, OFFSET>,
            get_ModuleType: get_ModuleType::<Identity, OFFSET>,
            get_Enabled: get_Enabled::<Identity, OFFSET>,
            put_Enabled: put_Enabled::<Identity, OFFSET>,
            get_NeedsFileContent: get_NeedsFileContent::<Identity, OFFSET>,
            put_NeedsFileContent: put_NeedsFileContent::<Identity, OFFSET>,
            get_Account: get_Account::<Identity, OFFSET>,
            put_Account: put_Account::<Identity, OFFSET>,
            get_SupportedExtensions: get_SupportedExtensions::<Identity, OFFSET>,
            put_SupportedExtensions: put_SupportedExtensions::<Identity, OFFSET>,
            get_Parameters: get_Parameters::<Identity, OFFSET>,
            put_Parameters: put_Parameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleDefinition as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmPipelineModuleDefinition {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmPipelineModuleImplementation, IFsrmPipelineModuleImplementation_Vtbl, 0xb7907906_2b02_4cb5_84a9_fdf54613d6cd);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmPipelineModuleImplementation {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmPipelineModuleImplementation, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmPipelineModuleImplementation {
    #[cfg(feature = "fsrm")]
    pub unsafe fn OnLoad<P0>(&self, moduledefinition: P0) -> windows_core::Result<IFsrmPipelineModuleConnector>
    where
        P0: windows_core::Param<IFsrmPipelineModuleDefinition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnLoad)(windows_core::Interface::as_raw(self), moduledefinition.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn OnUnload(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnUnload)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleImplementation_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "fsrm")]
    pub OnLoad: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrm"))]
    OnLoad: usize,
    pub OnUnload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrm", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPipelineModuleImplementation_Impl: super::IDispatch_Impl {
    fn OnLoad(&self, moduledefinition: windows_core::Ref<IFsrmPipelineModuleDefinition>) -> windows_core::Result<IFsrmPipelineModuleConnector>;
    fn OnUnload(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPipelineModuleImplementation_Vtbl {
    pub const fn new<Identity: IFsrmPipelineModuleImplementation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLoad<Identity: IFsrmPipelineModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduledefinition: *mut core::ffi::c_void, moduleconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleImplementation_Impl::OnLoad(this, core::mem::transmute_copy(&moduledefinition)) {
                    Ok(ok__) => {
                        moduleconnector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnUnload<Identity: IFsrmPipelineModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleImplementation_Impl::OnUnload(this).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), OnLoad: OnLoad::<Identity, OFFSET>, OnUnload: OnUnload::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleImplementation as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmPipelineModuleImplementation {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmProperty, IFsrmProperty_Vtbl, 0x4a73fee4_4102_4fcc_9ffb_38614f9ee768);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmProperty {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmProperty, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmProperty {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_Sources(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Sources)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_PropertyFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertyFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmProperty_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_Sources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_Sources: usize,
    pub get_PropertyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmProperty_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Sources(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_PropertyFlags(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmProperty_Vtbl {
    pub const fn new<Identity: IFsrmProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Value<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::get_Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Sources<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sources: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::get_Sources(this) {
                    Ok(ok__) => {
                        sources.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PropertyFlags<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::get_PropertyFlags(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_Value: get_Value::<Identity, OFFSET>,
            get_Sources: get_Sources::<Identity, OFFSET>,
            get_PropertyFlags: get_PropertyFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmProperty as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmProperty {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmPropertyBag, IFsrmPropertyBag_Vtbl, 0x774589d1_d300_4f7a_9a24_f7b766800250);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmPropertyBag {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyBag, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmPropertyBag {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_RelativePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RelativePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_RelativeNamespaceRoot(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RelativeNamespaceRoot)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_VolumeIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VolumeIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_FileId(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FileId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_ParentDirectoryId(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ParentDirectoryId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_Size(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_SizeAllocated(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SizeAllocated)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_CreationTime(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CreationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_LastAccessTime(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastAccessTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_LastModificationTime(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastModificationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Attributes(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Attributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_OwnerSid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_OwnerSid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_FilePropertyNames(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FilePropertyNames)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_Messages(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Messages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_PropertyBagFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertyBagFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFileProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFileProperty(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn AddMessage(&self, message: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(message)) }
    }
    #[cfg(all(feature = "fsrmenums", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: super::FsrmFileStreamingMode, interfacetype: super::FsrmFileStreamingInterfaceType) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileStreamInterface)(windows_core::Interface::as_raw(self), accessmode, interfacetype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RelativePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RelativeNamespaceRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_VolumeIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_FileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_FileId: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_ParentDirectoryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_ParentDirectoryId: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_Size: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_SizeAllocated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_SizeAllocated: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_CreationTime: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_LastAccessTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_LastAccessTime: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_LastModificationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_LastModificationTime: usize,
    pub get_Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub get_OwnerSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_FilePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_FilePropertyNames: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_Messages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_Messages: usize,
    pub get_PropertyBagFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "fsrmenums", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetFileStreamInterface: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmFileStreamingMode, super::FsrmFileStreamingInterfaceType, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrmenums", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetFileStreamInterface: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPropertyBag_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_RelativePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_RelativeNamespaceRoot(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_VolumeIndex(&self) -> windows_core::Result<u32>;
    fn get_FileId(&self) -> windows_core::Result<super::VARIANT>;
    fn get_ParentDirectoryId(&self) -> windows_core::Result<super::VARIANT>;
    fn get_Size(&self) -> windows_core::Result<super::VARIANT>;
    fn get_SizeAllocated(&self) -> windows_core::Result<super::VARIANT>;
    fn get_CreationTime(&self) -> windows_core::Result<super::VARIANT>;
    fn get_LastAccessTime(&self) -> windows_core::Result<super::VARIANT>;
    fn get_LastModificationTime(&self) -> windows_core::Result<super::VARIANT>;
    fn get_Attributes(&self) -> windows_core::Result<u32>;
    fn get_OwnerSid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_FilePropertyNames(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_Messages(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_PropertyBagFlags(&self) -> windows_core::Result<u32>;
    fn GetFileProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmProperty>;
    fn SetFileProperty(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddMessage(&self, message: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetFileStreamInterface(&self, accessmode: super::FsrmFileStreamingMode, interfacetype: super::FsrmFileStreamingInterfaceType) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPropertyBag_Vtbl {
    pub const fn new<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RelativePath<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_RelativePath(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VolumeName<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volumename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_VolumeName(this) {
                    Ok(ok__) => {
                        volumename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RelativeNamespaceRoot<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativenamespaceroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_RelativeNamespaceRoot(this) {
                    Ok(ok__) => {
                        relativenamespaceroot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VolumeIndex<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volumeid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_VolumeIndex(this) {
                    Ok(ok__) => {
                        volumeid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FileId<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileid: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_FileId(this) {
                    Ok(ok__) => {
                        fileid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ParentDirectoryId<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentdirectoryid: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_ParentDirectoryId(this) {
                    Ok(ok__) => {
                        parentdirectoryid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Size<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_Size(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SizeAllocated<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sizeallocated: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_SizeAllocated(this) {
                    Ok(ok__) => {
                        sizeallocated.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CreationTime<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, creationtime: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_CreationTime(this) {
                    Ok(ok__) => {
                        creationtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastAccessTime<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastaccesstime: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_LastAccessTime(this) {
                    Ok(ok__) => {
                        lastaccesstime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastModificationTime<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodificationtime: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_LastModificationTime(this) {
                    Ok(ok__) => {
                        lastmodificationtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Attributes<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_Attributes(this) {
                    Ok(ok__) => {
                        attributes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_OwnerSid<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ownersid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_OwnerSid(this) {
                    Ok(ok__) => {
                        ownersid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FilePropertyNames<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepropertynames: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_FilePropertyNames(this) {
                    Ok(ok__) => {
                        filepropertynames.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Messages<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messages: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_Messages(this) {
                    Ok(ok__) => {
                        messages.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PropertyBagFlags<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::get_PropertyBagFlags(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFileProperty<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, fileproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::GetFileProperty(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        fileproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileProperty<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyBag_Impl::SetFileProperty(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn AddMessage<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyBag_Impl::AddMessage(this, core::mem::transmute(&message)).into()
            }
        }
        unsafe extern "system" fn GetFileStreamInterface<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accessmode: super::FsrmFileStreamingMode, interfacetype: super::FsrmFileStreamingInterfaceType, pstreaminterface: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::GetFileStreamInterface(this, core::mem::transmute_copy(&accessmode), core::mem::transmute_copy(&interfacetype)) {
                    Ok(ok__) => {
                        pstreaminterface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_RelativePath: get_RelativePath::<Identity, OFFSET>,
            get_VolumeName: get_VolumeName::<Identity, OFFSET>,
            get_RelativeNamespaceRoot: get_RelativeNamespaceRoot::<Identity, OFFSET>,
            get_VolumeIndex: get_VolumeIndex::<Identity, OFFSET>,
            get_FileId: get_FileId::<Identity, OFFSET>,
            get_ParentDirectoryId: get_ParentDirectoryId::<Identity, OFFSET>,
            get_Size: get_Size::<Identity, OFFSET>,
            get_SizeAllocated: get_SizeAllocated::<Identity, OFFSET>,
            get_CreationTime: get_CreationTime::<Identity, OFFSET>,
            get_LastAccessTime: get_LastAccessTime::<Identity, OFFSET>,
            get_LastModificationTime: get_LastModificationTime::<Identity, OFFSET>,
            get_Attributes: get_Attributes::<Identity, OFFSET>,
            get_OwnerSid: get_OwnerSid::<Identity, OFFSET>,
            get_FilePropertyNames: get_FilePropertyNames::<Identity, OFFSET>,
            get_Messages: get_Messages::<Identity, OFFSET>,
            get_PropertyBagFlags: get_PropertyBagFlags::<Identity, OFFSET>,
            GetFileProperty: GetFileProperty::<Identity, OFFSET>,
            SetFileProperty: SetFileProperty::<Identity, OFFSET>,
            AddMessage: AddMessage::<Identity, OFFSET>,
            GetFileStreamInterface: GetFileStreamInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyBag as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmPropertyBag {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmPropertyBag2, IFsrmPropertyBag2_Vtbl, 0x0e46bdbd_2402_4fed_9c30_9266e6eb2cc9);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmPropertyBag2 {
    type Target = IFsrmPropertyBag;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyBag2, windows_core::IUnknown, super::IDispatch, IFsrmPropertyBag);
#[cfg(feature = "oaidl")]
impl IFsrmPropertyBag2 {
    #[cfg(all(feature = "fsrmenums", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetFieldValue(&self, field: super::FsrmPropertyBagField) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFieldValue)(windows_core::Interface::as_raw(self), field, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "fsrm")]
    pub unsafe fn GetUntrustedInFileProperties(&self) -> windows_core::Result<super::IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUntrustedInFileProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag2_Vtbl {
    pub base__: IFsrmPropertyBag_Vtbl,
    #[cfg(all(feature = "fsrmenums", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetFieldValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmPropertyBagField, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrmenums", feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetFieldValue: usize,
    #[cfg(feature = "fsrm")]
    pub GetUntrustedInFileProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrm"))]
    GetUntrustedInFileProperties: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPropertyBag2_Impl: IFsrmPropertyBag_Impl {
    fn GetFieldValue(&self, field: super::FsrmPropertyBagField) -> windows_core::Result<super::VARIANT>;
    fn GetUntrustedInFileProperties(&self) -> windows_core::Result<super::IFsrmCollection>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPropertyBag2_Vtbl {
    pub const fn new<Identity: IFsrmPropertyBag2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFieldValue<Identity: IFsrmPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, field: super::FsrmPropertyBagField, value: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag2_Impl::GetFieldValue(this, core::mem::transmute_copy(&field)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUntrustedInFileProperties<Identity: IFsrmPropertyBag2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, props: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag2_Impl::GetUntrustedInFileProperties(this) {
                    Ok(ok__) => {
                        props.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmPropertyBag_Vtbl::new::<Identity, OFFSET>(),
            GetFieldValue: GetFieldValue::<Identity, OFFSET>,
            GetUntrustedInFileProperties: GetUntrustedInFileProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyBag2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmPropertyBag as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmPropertyBag2 {}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::define_interface!(IFsrmPropertyDefinition, IFsrmPropertyDefinition_Vtbl, 0xede0150f_e9a3_419c_877c_01fe5d24c5d3);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl core::ops::Deref for IFsrmPropertyDefinition {
    type Target = super::IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::interface_hierarchy!(IFsrmPropertyDefinition, windows_core::IUnknown, super::IDispatch, super::IFsrmObject);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl IFsrmPropertyDefinition {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_Type(&self) -> windows_core::Result<super::FsrmPropertyDefinitionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn put_Type(&self, r#type: super::FsrmPropertyDefinitionType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Type)(windows_core::Interface::as_raw(self), r#type) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_PossibleValues(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PossibleValues)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_PossibleValues(&self, possiblevalues: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PossibleValues)(windows_core::Interface::as_raw(self), possiblevalues) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_ValueDescriptions(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ValueDescriptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_ValueDescriptions(&self, valuedescriptions: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ValueDescriptions)(windows_core::Interface::as_raw(self), valuedescriptions) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_Parameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Parameters)(windows_core::Interface::as_raw(self), parameters) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition_Vtbl {
    pub base__: super::IFsrmObject_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub get_Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmPropertyDefinitionType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_Type: usize,
    #[cfg(feature = "fsrmenums")]
    pub put_Type: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmPropertyDefinitionType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    put_Type: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_PossibleValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_PossibleValues: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_PossibleValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_PossibleValues: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_ValueDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_ValueDescriptions: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_ValueDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_ValueDescriptions: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_Parameters: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_Parameters: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPropertyDefinition_Impl: super::IFsrmObject_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Type(&self) -> windows_core::Result<super::FsrmPropertyDefinitionType>;
    fn put_Type(&self, r#type: super::FsrmPropertyDefinitionType) -> windows_core::Result<()>;
    fn get_PossibleValues(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_PossibleValues(&self, possiblevalues: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_ValueDescriptions(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_ValueDescriptions(&self, valuedescriptions: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_Parameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPropertyDefinition_Vtbl {
    pub const fn new<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Name<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::put_Name(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_Type<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut super::FsrmPropertyDefinitionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::get_Type(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Type<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: super::FsrmPropertyDefinitionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::put_Type(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn get_PossibleValues<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, possiblevalues: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::get_PossibleValues(this) {
                    Ok(ok__) => {
                        possiblevalues.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PossibleValues<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, possiblevalues: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::put_PossibleValues(this, core::mem::transmute_copy(&possiblevalues)).into()
            }
        }
        unsafe extern "system" fn get_ValueDescriptions<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuedescriptions: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::get_ValueDescriptions(this) {
                    Ok(ok__) => {
                        valuedescriptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ValueDescriptions<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuedescriptions: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::put_ValueDescriptions(this, core::mem::transmute_copy(&valuedescriptions)).into()
            }
        }
        unsafe extern "system" fn get_Parameters<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::get_Parameters(this) {
                    Ok(ok__) => {
                        parameters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Parameters<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::put_Parameters(this, core::mem::transmute_copy(&parameters)).into()
            }
        }
        Self {
            base__: super::IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            put_Name: put_Name::<Identity, OFFSET>,
            get_Type: get_Type::<Identity, OFFSET>,
            put_Type: put_Type::<Identity, OFFSET>,
            get_PossibleValues: get_PossibleValues::<Identity, OFFSET>,
            put_PossibleValues: put_PossibleValues::<Identity, OFFSET>,
            get_ValueDescriptions: get_ValueDescriptions::<Identity, OFFSET>,
            put_ValueDescriptions: put_ValueDescriptions::<Identity, OFFSET>,
            get_Parameters: get_Parameters::<Identity, OFFSET>,
            put_Parameters: put_Parameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmPropertyDefinition {}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::define_interface!(IFsrmPropertyDefinition2, IFsrmPropertyDefinition2_Vtbl, 0x47782152_d16c_4229_b4e1_0ddfe308b9f6);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl core::ops::Deref for IFsrmPropertyDefinition2 {
    type Target = IFsrmPropertyDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::interface_hierarchy!(IFsrmPropertyDefinition2, windows_core::IUnknown, super::IDispatch, super::IFsrmObject, IFsrmPropertyDefinition);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl IFsrmPropertyDefinition2 {
    pub unsafe fn get_PropertyDefinitionFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertyDefinitionFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_DisplayName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn get_AppliesTo(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AppliesTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ValueDefinitions(&self) -> windows_core::Result<super::IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ValueDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition2_Vtbl {
    pub base__: IFsrmPropertyDefinition_Vtbl,
    pub get_PropertyDefinitionFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_AppliesTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_ValueDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPropertyDefinition2_Impl: IFsrmPropertyDefinition_Impl {
    fn get_PropertyDefinitionFlags(&self) -> windows_core::Result<i32>;
    fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_DisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_AppliesTo(&self) -> windows_core::Result<i32>;
    fn get_ValueDefinitions(&self) -> windows_core::Result<super::IFsrmCollection>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPropertyDefinition2_Vtbl {
    pub const fn new<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_PropertyDefinitionFlags<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertydefinitionflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::get_PropertyDefinitionFlags(this) {
                    Ok(ok__) => {
                        propertydefinitionflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DisplayName<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::get_DisplayName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisplayName<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition2_Impl::put_DisplayName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_AppliesTo<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appliesto: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::get_AppliesTo(this) {
                    Ok(ok__) => {
                        appliesto.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ValueDefinitions<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuedefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::get_ValueDefinitions(this) {
                    Ok(ok__) => {
                        valuedefinitions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFsrmPropertyDefinition_Vtbl::new::<Identity, OFFSET>(),
            get_PropertyDefinitionFlags: get_PropertyDefinitionFlags::<Identity, OFFSET>,
            get_DisplayName: get_DisplayName::<Identity, OFFSET>,
            put_DisplayName: put_DisplayName::<Identity, OFFSET>,
            get_AppliesTo: get_AppliesTo::<Identity, OFFSET>,
            get_ValueDefinitions: get_ValueDefinitions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmPropertyDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmPropertyDefinition2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmPropertyDefinitionValue, IFsrmPropertyDefinitionValue_Vtbl, 0xe946d148_bd67_4178_8e22_1c44925ed710);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmPropertyDefinitionValue {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmPropertyDefinitionValue, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsrmPropertyDefinitionValue {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_UniqueID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UniqueID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinitionValue_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_UniqueID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPropertyDefinitionValue_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_UniqueID(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPropertyDefinitionValue_Vtbl {
    pub const fn new<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DisplayName<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::get_DisplayName(this) {
                    Ok(ok__) => {
                        displayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::get_Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UniqueID<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uniqueid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::get_UniqueID(this) {
                    Ok(ok__) => {
                        uniqueid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_DisplayName: get_DisplayName::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            get_UniqueID: get_UniqueID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinitionValue as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmPropertyDefinitionValue {}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::define_interface!(IFsrmRule, IFsrmRule_Vtbl, 0xcb0df960_16f5_4495_9079_3f9360d831df);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl core::ops::Deref for IFsrmRule {
    type Target = super::IFsrmObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::interface_hierarchy!(IFsrmRule, windows_core::IUnknown, super::IDispatch, super::IFsrmObject);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl IFsrmRule {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Name)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_RuleType(&self) -> windows_core::Result<super::FsrmRuleType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RuleType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ModuleDefinitionName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ModuleDefinitionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ModuleDefinitionName(&self, moduledefinitionname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ModuleDefinitionName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(moduledefinitionname)) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_NamespaceRoots(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NamespaceRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_NamespaceRoots(&self, namespaceroots: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NamespaceRoots)(windows_core::Interface::as_raw(self), namespaceroots) }
    }
    pub unsafe fn get_RuleFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RuleFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_RuleFlags(&self, ruleflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RuleFlags)(windows_core::Interface::as_raw(self), ruleflags) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_Parameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Parameters)(windows_core::Interface::as_raw(self), parameters) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_LastModified(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmRule_Vtbl {
    pub base__: super::IFsrmObject_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub get_RuleType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmRuleType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_RuleType: usize,
    pub get_ModuleDefinitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ModuleDefinitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_NamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_NamespaceRoots: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_NamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_NamespaceRoots: usize,
    pub get_RuleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_RuleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_Parameters: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_Parameters: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_LastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_LastModified: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmRule_Impl: super::IFsrmObject_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Name(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_RuleType(&self) -> windows_core::Result<super::FsrmRuleType>;
    fn get_ModuleDefinitionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ModuleDefinitionName(&self, moduledefinitionname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_NamespaceRoots(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_NamespaceRoots(&self, namespaceroots: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_RuleFlags(&self) -> windows_core::Result<i32>;
    fn put_RuleFlags(&self, ruleflags: i32) -> windows_core::Result<()>;
    fn get_Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_Parameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn get_LastModified(&self) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmRule_Vtbl {
    pub const fn new<Identity: IFsrmRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::get_Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Name<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::put_Name(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_RuleType<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruletype: *mut super::FsrmRuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::get_RuleType(this) {
                    Ok(ok__) => {
                        ruletype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ModuleDefinitionName<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduledefinitionname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::get_ModuleDefinitionName(this) {
                    Ok(ok__) => {
                        moduledefinitionname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ModuleDefinitionName<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduledefinitionname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::put_ModuleDefinitionName(this, core::mem::transmute(&moduledefinitionname)).into()
            }
        }
        unsafe extern "system" fn get_NamespaceRoots<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceroots: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::get_NamespaceRoots(this) {
                    Ok(ok__) => {
                        namespaceroots.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_NamespaceRoots<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceroots: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::put_NamespaceRoots(this, core::mem::transmute_copy(&namespaceroots)).into()
            }
        }
        unsafe extern "system" fn get_RuleFlags<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruleflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::get_RuleFlags(this) {
                    Ok(ok__) => {
                        ruleflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RuleFlags<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruleflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::put_RuleFlags(this, core::mem::transmute_copy(&ruleflags)).into()
            }
        }
        unsafe extern "system" fn get_Parameters<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::get_Parameters(this) {
                    Ok(ok__) => {
                        parameters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Parameters<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::put_Parameters(this, core::mem::transmute_copy(&parameters)).into()
            }
        }
        unsafe extern "system" fn get_LastModified<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodified: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::get_LastModified(this) {
                    Ok(ok__) => {
                        lastmodified.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            put_Name: put_Name::<Identity, OFFSET>,
            get_RuleType: get_RuleType::<Identity, OFFSET>,
            get_ModuleDefinitionName: get_ModuleDefinitionName::<Identity, OFFSET>,
            put_ModuleDefinitionName: put_ModuleDefinitionName::<Identity, OFFSET>,
            get_NamespaceRoots: get_NamespaceRoots::<Identity, OFFSET>,
            put_NamespaceRoots: put_NamespaceRoots::<Identity, OFFSET>,
            get_RuleFlags: get_RuleFlags::<Identity, OFFSET>,
            put_RuleFlags: put_RuleFlags::<Identity, OFFSET>,
            get_Parameters: get_Parameters::<Identity, OFFSET>,
            put_Parameters: put_Parameters::<Identity, OFFSET>,
            get_LastModified: get_LastModified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmRule as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmRule {}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::define_interface!(IFsrmStorageModuleDefinition, IFsrmStorageModuleDefinition_Vtbl, 0x15a81350_497d_4aba_80e9_d4dbcc5521fe);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl core::ops::Deref for IFsrmStorageModuleDefinition {
    type Target = IFsrmPipelineModuleDefinition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
windows_core::imp::interface_hierarchy!(IFsrmStorageModuleDefinition, windows_core::IUnknown, super::IDispatch, super::IFsrmObject, IFsrmPipelineModuleDefinition);
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
impl IFsrmStorageModuleDefinition {
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_Capabilities(&self) -> windows_core::Result<super::FsrmStorageModuleCaps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Capabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn put_Capabilities(&self, capabilities: super::FsrmStorageModuleCaps) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Capabilities)(windows_core::Interface::as_raw(self), capabilities) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn get_StorageType(&self) -> windows_core::Result<super::FsrmStorageModuleType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StorageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn put_StorageType(&self, storagetype: super::FsrmStorageModuleType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StorageType)(windows_core::Interface::as_raw(self), storagetype) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_UpdatesFileContent(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UpdatesFileContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_UpdatesFileContent(&self, updatesfilecontent: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UpdatesFileContent)(windows_core::Interface::as_raw(self), updatesfilecontent) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub get_Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmStorageModuleCaps) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_Capabilities: usize,
    #[cfg(feature = "fsrmenums")]
    pub put_Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmStorageModuleCaps) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    put_Capabilities: usize,
    #[cfg(feature = "fsrmenums")]
    pub get_StorageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmStorageModuleType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    get_StorageType: usize,
    #[cfg(feature = "fsrmenums")]
    pub put_StorageType: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmStorageModuleType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    put_StorageType: usize,
    #[cfg(feature = "wtypes")]
    pub get_UpdatesFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_UpdatesFileContent: usize,
    #[cfg(feature = "wtypes")]
    pub put_UpdatesFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_UpdatesFileContent: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmStorageModuleDefinition_Impl: IFsrmPipelineModuleDefinition_Impl {
    fn get_Capabilities(&self) -> windows_core::Result<super::FsrmStorageModuleCaps>;
    fn put_Capabilities(&self, capabilities: super::FsrmStorageModuleCaps) -> windows_core::Result<()>;
    fn get_StorageType(&self) -> windows_core::Result<super::FsrmStorageModuleType>;
    fn put_StorageType(&self, storagetype: super::FsrmStorageModuleType) -> windows_core::Result<()>;
    fn get_UpdatesFileContent(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_UpdatesFileContent(&self, updatesfilecontent: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmStorageModuleDefinition_Vtbl {
    pub const fn new<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Capabilities<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilities: *mut super::FsrmStorageModuleCaps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmStorageModuleDefinition_Impl::get_Capabilities(this) {
                    Ok(ok__) => {
                        capabilities.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Capabilities<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilities: super::FsrmStorageModuleCaps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleDefinition_Impl::put_Capabilities(this, core::mem::transmute_copy(&capabilities)).into()
            }
        }
        unsafe extern "system" fn get_StorageType<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storagetype: *mut super::FsrmStorageModuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmStorageModuleDefinition_Impl::get_StorageType(this) {
                    Ok(ok__) => {
                        storagetype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StorageType<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storagetype: super::FsrmStorageModuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleDefinition_Impl::put_StorageType(this, core::mem::transmute_copy(&storagetype)).into()
            }
        }
        unsafe extern "system" fn get_UpdatesFileContent<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatesfilecontent: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmStorageModuleDefinition_Impl::get_UpdatesFileContent(this) {
                    Ok(ok__) => {
                        updatesfilecontent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UpdatesFileContent<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatesfilecontent: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleDefinition_Impl::put_UpdatesFileContent(this, core::mem::transmute_copy(&updatesfilecontent)).into()
            }
        }
        Self {
            base__: IFsrmPipelineModuleDefinition_Vtbl::new::<Identity, OFFSET>(),
            get_Capabilities: get_Capabilities::<Identity, OFFSET>,
            put_Capabilities: put_Capabilities::<Identity, OFFSET>,
            get_StorageType: get_StorageType::<Identity, OFFSET>,
            put_StorageType: put_StorageType::<Identity, OFFSET>,
            get_UpdatesFileContent: get_UpdatesFileContent::<Identity, OFFSET>,
            put_UpdatesFileContent: put_UpdatesFileContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmStorageModuleDefinition as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmPipelineModuleDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmStorageModuleDefinition {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsrmStorageModuleImplementation, IFsrmStorageModuleImplementation_Vtbl, 0x0af4a0da_895a_4e50_8712_a96724bcec64);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsrmStorageModuleImplementation {
    type Target = IFsrmPipelineModuleImplementation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsrmStorageModuleImplementation, windows_core::IUnknown, super::IDispatch, IFsrmPipelineModuleImplementation);
#[cfg(feature = "oaidl")]
impl IFsrmStorageModuleImplementation {
    #[cfg(feature = "fsrm")]
    pub unsafe fn UseDefinitions<P0>(&self, propertydefinitions: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IFsrmCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).UseDefinitions)(windows_core::Interface::as_raw(self), propertydefinitions.param().abi()) }
    }
    pub unsafe fn LoadProperties<P0>(&self, propertybag: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFsrmPropertyBag>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadProperties)(windows_core::Interface::as_raw(self), propertybag.param().abi()) }
    }
    pub unsafe fn SaveProperties<P0>(&self, propertybag: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFsrmPropertyBag>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveProperties)(windows_core::Interface::as_raw(self), propertybag.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleImplementation_Vtbl {
    pub base__: IFsrmPipelineModuleImplementation_Vtbl,
    #[cfg(feature = "fsrm")]
    pub UseDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrm"))]
    UseDefinitions: usize,
    pub LoadProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrm", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmStorageModuleImplementation_Impl: IFsrmPipelineModuleImplementation_Impl {
    fn UseDefinitions(&self, propertydefinitions: windows_core::Ref<super::IFsrmCollection>) -> windows_core::Result<()>;
    fn LoadProperties(&self, propertybag: windows_core::Ref<IFsrmPropertyBag>) -> windows_core::Result<()>;
    fn SaveProperties(&self, propertybag: windows_core::Ref<IFsrmPropertyBag>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmStorageModuleImplementation_Vtbl {
    pub const fn new<Identity: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UseDefinitions<Identity: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertydefinitions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleImplementation_Impl::UseDefinitions(this, core::mem::transmute_copy(&propertydefinitions)).into()
            }
        }
        unsafe extern "system" fn LoadProperties<Identity: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertybag: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleImplementation_Impl::LoadProperties(this, core::mem::transmute_copy(&propertybag)).into()
            }
        }
        unsafe extern "system" fn SaveProperties<Identity: IFsrmStorageModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertybag: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleImplementation_Impl::SaveProperties(this, core::mem::transmute_copy(&propertybag)).into()
            }
        }
        Self {
            base__: IFsrmPipelineModuleImplementation_Vtbl::new::<Identity, OFFSET>(),
            UseDefinitions: UseDefinitions::<Identity, OFFSET>,
            LoadProperties: LoadProperties::<Identity, OFFSET>,
            SaveProperties: SaveProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmStorageModuleImplementation as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsrmPipelineModuleImplementation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmStorageModuleImplementation {}
pub const MessageSizeLimit: i32 = 4096;
