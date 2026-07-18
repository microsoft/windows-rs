pub const FSRM_DISPID_CLASSIFICATION_EVENTS: u32 = 90177536;
pub const FSRM_DISPID_CLASSIFICATION_MANAGER: u32 = 89128960;
pub const FSRM_DISPID_CLASSIFICATION_MANAGER2: u32 = 89194496;
pub const FSRM_DISPID_CLASSIFICATION_RULE: u32 = 87097344;
pub const FSRM_DISPID_CLASSIFIER_MODULE_DEFINITION: u32 = 88145920;
pub const FSRM_DISPID_CLASSIFIER_MODULE_IMPLEMENTATION: u32 = 102825984;
pub const FSRM_DISPID_EXPIRATION_RULE: u32 = 87162880;
pub const FSRM_DISPID_PIPELINE_MODULE_CONNECTOR: u32 = 103809024;
pub const FSRM_DISPID_PIPELINE_MODULE_DEFINITION: u32 = 88080384;
pub const FSRM_DISPID_PIPELINE_MODULE_IMPLEMENTATION: u32 = 102760448;
pub const FSRM_DISPID_PROPERTY: u32 = 85983232;
pub const FSRM_DISPID_PROPERTY_BAG: u32 = 101711872;
pub const FSRM_DISPID_PROPERTY_BAG2: u32 = 101777408;
pub const FSRM_DISPID_PROPERTY_DEFINITION: u32 = 84934656;
pub const FSRM_DISPID_PROPERTY_DEFINITION2: u32 = 85000192;
pub const FSRM_DISPID_PROPERTY_DEFINITION_VALUE: u32 = 91226112;
pub const FSRM_DISPID_RULE: u32 = 87031808;
pub const FSRM_DISPID_STORAGE_MODULE_DEFINITION: u32 = 88211456;
pub const FSRM_DISPID_STORAGE_MODULE_IMPLEMENTATION: u32 = 102891520;
pub const FsrmAlwaysModified: i32 = -1;
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100;
pub const FsrmNeverModified: u32 = 0;
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
    pub unsafe fn ClassificationReportFormats(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationReportFormats)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClassificationReportFormats)(windows_core::Interface::as_raw(self), formats) }
    }
    pub unsafe fn Logging(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Logging)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLogging(&self, logging: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLogging)(windows_core::Interface::as_raw(self), logging) }
    }
    pub unsafe fn ClassificationReportMailTo(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationReportMailTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetClassificationReportMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClassificationReportMailTo)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mailto)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ClassificationReportEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationReportEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetClassificationReportEnabled(&self, reportenabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClassificationReportEnabled)(windows_core::Interface::as_raw(self), reportenabled) }
    }
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationLastReportPathWithoutExtension)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ClassificationLastError(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationLastError)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn ClassificationRunningStatus(&self) -> windows_core::Result<super::FsrmReportRunningStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassificationRunningStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn EnumPropertyDefinitions(&self, options: super::FsrmEnumOptions) -> windows_core::Result<super::IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumPropertyDefinitions)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "fsrm")]
    pub unsafe fn CreatePropertyDefinition(&self) -> windows_core::Result<IFsrmPropertyDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyDefinition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "fsrm")]
    pub unsafe fn GetPropertyDefinition(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<IFsrmPropertyDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn EnumRules(&self, ruletype: super::FsrmRuleType, options: super::FsrmEnumOptions) -> windows_core::Result<super::IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRules)(windows_core::Interface::as_raw(self), ruletype, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn CreateRule(&self, ruletype: super::FsrmRuleType) -> windows_core::Result<IFsrmRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRule)(windows_core::Interface::as_raw(self), ruletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn GetRule(&self, rulename: &windows_core::BSTR, ruletype: super::FsrmRuleType) -> windows_core::Result<IFsrmRule> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRule)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(rulename), ruletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: super::FsrmPipelineModuleType, options: super::FsrmEnumOptions) -> windows_core::Result<super::IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumModuleDefinitions)(windows_core::Interface::as_raw(self), moduletype, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn CreateModuleDefinition(&self, moduletype: super::FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateModuleDefinition)(windows_core::Interface::as_raw(self), moduletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "fsrm", feature = "fsrmenums"))]
    pub unsafe fn GetModuleDefinition(&self, modulename: &windows_core::BSTR, moduletype: super::FsrmPipelineModuleType) -> windows_core::Result<IFsrmPipelineModuleDefinition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetModuleDefinition)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(modulename), moduletype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(self).EnumFileProperties)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn GetFileProperty(&self, filepath: &windows_core::BSTR, propertyname: &windows_core::BSTR, options: super::FsrmGetFilePropertyOptions) -> windows_core::Result<IFsrmProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(filepath), core::mem::transmute_copy(propertyname), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub ClassificationReportFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub SetClassificationReportFormats: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    pub Logging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetLogging: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ClassificationReportMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClassificationReportMailTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub ClassificationReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ClassificationReportEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetClassificationReportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetClassificationReportEnabled: usize,
    pub ClassificationLastReportPathWithoutExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClassificationLastError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub ClassificationRunningStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmReportRunningStatus) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    ClassificationRunningStatus: usize,
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
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmClassificationManager_Impl: super::IDispatch_Impl {
    fn ClassificationReportFormats(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn SetClassificationReportFormats(&self, formats: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn Logging(&self) -> windows_core::Result<i32>;
    fn SetLogging(&self, logging: i32) -> windows_core::Result<()>;
    fn ClassificationReportMailTo(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetClassificationReportMailTo(&self, mailto: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClassificationReportEnabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetClassificationReportEnabled(&self, reportenabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ClassificationLastReportPathWithoutExtension(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClassificationLastError(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ClassificationRunningStatus(&self) -> windows_core::Result<super::FsrmReportRunningStatus>;
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
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmClassificationManager_Vtbl {
    pub const fn new<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ClassificationReportFormats<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formats: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationReportFormats(this) {
                    Ok(ok__) => {
                        formats.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClassificationReportFormats<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formats: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::SetClassificationReportFormats(this, core::mem::transmute_copy(&formats)).into()
            }
        }
        unsafe extern "system" fn Logging<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logging: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::Logging(this) {
                    Ok(ok__) => {
                        logging.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLogging<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, logging: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::SetLogging(this, core::mem::transmute_copy(&logging)).into()
            }
        }
        unsafe extern "system" fn ClassificationReportMailTo<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationReportMailTo(this) {
                    Ok(ok__) => {
                        mailto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClassificationReportMailTo<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mailto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::SetClassificationReportMailTo(this, core::mem::transmute(&mailto)).into()
            }
        }
        unsafe extern "system" fn ClassificationReportEnabled<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportenabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationReportEnabled(this) {
                    Ok(ok__) => {
                        reportenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClassificationReportEnabled<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reportenabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationManager_Impl::SetClassificationReportEnabled(this, core::mem::transmute_copy(&reportenabled)).into()
            }
        }
        unsafe extern "system" fn ClassificationLastReportPathWithoutExtension<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastreportpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationLastReportPathWithoutExtension(this) {
                    Ok(ok__) => {
                        lastreportpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClassificationLastError<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lasterror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationLastError(this) {
                    Ok(ok__) => {
                        lasterror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClassificationRunningStatus<Identity: IFsrmClassificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runningstatus: *mut super::FsrmReportRunningStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationManager_Impl::ClassificationRunningStatus(this) {
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
            ClassificationReportFormats: ClassificationReportFormats::<Identity, OFFSET>,
            SetClassificationReportFormats: SetClassificationReportFormats::<Identity, OFFSET>,
            Logging: Logging::<Identity, OFFSET>,
            SetLogging: SetLogging::<Identity, OFFSET>,
            ClassificationReportMailTo: ClassificationReportMailTo::<Identity, OFFSET>,
            SetClassificationReportMailTo: SetClassificationReportMailTo::<Identity, OFFSET>,
            ClassificationReportEnabled: ClassificationReportEnabled::<Identity, OFFSET>,
            SetClassificationReportEnabled: SetClassificationReportEnabled::<Identity, OFFSET>,
            ClassificationLastReportPathWithoutExtension: ClassificationLastReportPathWithoutExtension::<Identity, OFFSET>,
            ClassificationLastError: ClassificationLastError::<Identity, OFFSET>,
            ClassificationRunningStatus: ClassificationRunningStatus::<Identity, OFFSET>,
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
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn ClassifyFiles(&self, filepaths: *const super::SAFEARRAY, propertynames: *const super::SAFEARRAY, propertyvalues: *const super::SAFEARRAY, options: super::FsrmGetFilePropertyOptions) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClassifyFiles)(windows_core::Interface::as_raw(self), filepaths, propertynames, propertyvalues, options) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager2_Vtbl {
    pub base__: IFsrmClassificationManager_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub ClassifyFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY, *const super::SAFEARRAY, *const super::SAFEARRAY, super::FsrmGetFilePropertyOptions) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    ClassifyFiles: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmClassificationManager2_Impl: IFsrmClassificationManager_Impl {
    fn ClassifyFiles(&self, filepaths: *const super::SAFEARRAY, propertynames: *const super::SAFEARRAY, propertyvalues: *const super::SAFEARRAY, options: super::FsrmGetFilePropertyOptions) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn ExecutionOption(&self) -> windows_core::Result<super::FsrmExecutionOption> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExecutionOption)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn SetExecutionOption(&self, executionoption: super::FsrmExecutionOption) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExecutionOption)(windows_core::Interface::as_raw(self), executionoption) }
    }
    pub unsafe fn PropertyAffected(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyAffected)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPropertyAffected(&self, property: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyAffected)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(property)) }
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetValue(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationRule_Vtbl {
    pub base__: IFsrmRule_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub ExecutionOption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmExecutionOption) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    ExecutionOption: usize,
    #[cfg(feature = "fsrmenums")]
    pub SetExecutionOption: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmExecutionOption) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    SetExecutionOption: usize,
    pub PropertyAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPropertyAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmClassificationRule_Impl: IFsrmRule_Impl {
    fn ExecutionOption(&self) -> windows_core::Result<super::FsrmExecutionOption>;
    fn SetExecutionOption(&self, executionoption: super::FsrmExecutionOption) -> windows_core::Result<()>;
    fn PropertyAffected(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPropertyAffected(&self, property: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetValue(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmClassificationRule_Vtbl {
    pub const fn new<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExecutionOption<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executionoption: *mut super::FsrmExecutionOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationRule_Impl::ExecutionOption(this) {
                    Ok(ok__) => {
                        executionoption.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExecutionOption<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, executionoption: super::FsrmExecutionOption) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationRule_Impl::SetExecutionOption(this, core::mem::transmute_copy(&executionoption)).into()
            }
        }
        unsafe extern "system" fn PropertyAffected<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationRule_Impl::PropertyAffected(this) {
                    Ok(ok__) => {
                        property.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPropertyAffected<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationRule_Impl::SetPropertyAffected(this, core::mem::transmute(&property)).into()
            }
        }
        unsafe extern "system" fn Value<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassificationRule_Impl::Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IFsrmClassificationRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassificationRule_Impl::SetValue(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: IFsrmRule_Vtbl::new::<Identity, OFFSET>(),
            ExecutionOption: ExecutionOption::<Identity, OFFSET>,
            SetExecutionOption: SetExecutionOption::<Identity, OFFSET>,
            PropertyAffected: PropertyAffected::<Identity, OFFSET>,
            SetPropertyAffected: SetPropertyAffected::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassificationRule as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmRule as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn PropertiesAffected(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertiesAffected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPropertiesAffected(&self, propertiesaffected: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPropertiesAffected)(windows_core::Interface::as_raw(self), propertiesaffected) }
    }
    pub unsafe fn PropertiesUsed(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertiesUsed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPropertiesUsed(&self, propertiesused: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPropertiesUsed)(windows_core::Interface::as_raw(self), propertiesused) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn NeedsExplicitValue(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NeedsExplicitValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetNeedsExplicitValue(&self, needsexplicitvalue: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNeedsExplicitValue)(windows_core::Interface::as_raw(self), needsexplicitvalue) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    pub PropertiesAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub SetPropertiesAffected: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    pub PropertiesUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub SetPropertiesUsed: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub NeedsExplicitValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    NeedsExplicitValue: usize,
    #[cfg(feature = "wtypes")]
    pub SetNeedsExplicitValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetNeedsExplicitValue: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmClassifierModuleDefinition_Impl: IFsrmPipelineModuleDefinition_Impl {
    fn PropertiesAffected(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn SetPropertiesAffected(&self, propertiesaffected: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn PropertiesUsed(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn SetPropertiesUsed(&self, propertiesused: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn NeedsExplicitValue(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetNeedsExplicitValue(&self, needsexplicitvalue: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmClassifierModuleDefinition_Vtbl {
    pub const fn new<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PropertiesAffected<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesaffected: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleDefinition_Impl::PropertiesAffected(this) {
                    Ok(ok__) => {
                        propertiesaffected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPropertiesAffected<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesaffected: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleDefinition_Impl::SetPropertiesAffected(this, core::mem::transmute_copy(&propertiesaffected)).into()
            }
        }
        unsafe extern "system" fn PropertiesUsed<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesused: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleDefinition_Impl::PropertiesUsed(this) {
                    Ok(ok__) => {
                        propertiesused.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPropertiesUsed<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiesused: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleDefinition_Impl::SetPropertiesUsed(this, core::mem::transmute_copy(&propertiesused)).into()
            }
        }
        unsafe extern "system" fn NeedsExplicitValue<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsexplicitvalue: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleDefinition_Impl::NeedsExplicitValue(this) {
                    Ok(ok__) => {
                        needsexplicitvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNeedsExplicitValue<Identity: IFsrmClassifierModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsexplicitvalue: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmClassifierModuleDefinition_Impl::SetNeedsExplicitValue(this, core::mem::transmute_copy(&needsexplicitvalue)).into()
            }
        }
        Self {
            base__: IFsrmPipelineModuleDefinition_Vtbl::new::<Identity, OFFSET>(),
            PropertiesAffected: PropertiesAffected::<Identity, OFFSET>,
            SetPropertiesAffected: SetPropertiesAffected::<Identity, OFFSET>,
            PropertiesUsed: PropertiesUsed::<Identity, OFFSET>,
            SetPropertiesUsed: SetPropertiesUsed::<Identity, OFFSET>,
            NeedsExplicitValue: NeedsExplicitValue::<Identity, OFFSET>,
            SetNeedsExplicitValue: SetNeedsExplicitValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmClassifierModuleDefinition as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmPipelineModuleDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn LastModified(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
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
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub LastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    LastModified: usize,
    #[cfg(feature = "fsrm")]
    pub UseRulesAndDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrm"))]
    UseRulesAndDefinitions: usize,
    pub OnBeginFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
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
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmClassifierModuleImplementation_Impl: IFsrmPipelineModuleImplementation_Impl {
    fn LastModified(&self) -> windows_core::Result<super::VARIANT>;
    fn UseRulesAndDefinitions(&self, rules: windows_core::Ref<super::IFsrmCollection>, propertydefinitions: windows_core::Ref<super::IFsrmCollection>) -> windows_core::Result<()>;
    fn OnBeginFile(&self, propertybag: windows_core::Ref<IFsrmPropertyBag>, arrayruleids: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn DoesPropertyValueApply(&self, property: &windows_core::BSTR, value: &windows_core::BSTR, applyvalue: *mut super::VARIANT_BOOL, idrule: &super::FSRM_OBJECT_ID, idpropdef: &super::FSRM_OBJECT_ID) -> windows_core::Result<()>;
    fn GetPropertyValueToApply(&self, property: &windows_core::BSTR, value: *mut windows_core::BSTR, idrule: &super::FSRM_OBJECT_ID, idpropdef: &super::FSRM_OBJECT_ID) -> windows_core::Result<()>;
    fn OnEndFile(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmClassifierModuleImplementation_Vtbl {
    pub const fn new<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LastModified<Identity: IFsrmClassifierModuleImplementation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodified: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmClassifierModuleImplementation_Impl::LastModified(this) {
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
            LastModified: LastModified::<Identity, OFFSET>,
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
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn ModuleImplementation(&self) -> windows_core::Result<IFsrmPipelineModuleImplementation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleImplementation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ModuleName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HostingUserAccount(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HostingUserAccount)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HostingProcessPid(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HostingProcessPid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub ModuleImplementation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ModuleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HostingUserAccount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HostingProcessPid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "fsrm")]
    pub Bind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrm"))]
    Bind: usize,
}
#[cfg(all(feature = "fsrm", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPipelineModuleConnector_Impl: super::IDispatch_Impl {
    fn ModuleImplementation(&self) -> windows_core::Result<IFsrmPipelineModuleImplementation>;
    fn ModuleName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn HostingUserAccount(&self) -> windows_core::Result<windows_core::BSTR>;
    fn HostingProcessPid(&self) -> windows_core::Result<i32>;
    fn Bind(&self, moduledefinition: windows_core::Ref<IFsrmPipelineModuleDefinition>, moduleimplementation: windows_core::Ref<IFsrmPipelineModuleImplementation>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPipelineModuleConnector_Vtbl {
    pub const fn new<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ModuleImplementation<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipelinemoduleimplementation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::ModuleImplementation(this) {
                    Ok(ok__) => {
                        pipelinemoduleimplementation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModuleName<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, username: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::ModuleName(this) {
                    Ok(ok__) => {
                        username.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HostingUserAccount<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useraccount: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::HostingUserAccount(this) {
                    Ok(ok__) => {
                        useraccount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HostingProcessPid<Identity: IFsrmPipelineModuleConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleConnector_Impl::HostingProcessPid(this) {
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
            ModuleImplementation: ModuleImplementation::<Identity, OFFSET>,
            ModuleName: ModuleName::<Identity, OFFSET>,
            HostingUserAccount: HostingUserAccount::<Identity, OFFSET>,
            HostingProcessPid: HostingProcessPid::<Identity, OFFSET>,
            Bind: Bind::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleConnector as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn ModuleClsid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleClsid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetModuleClsid(&self, moduleclsid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModuleClsid)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(moduleclsid)) }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn Company(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Company)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCompany(&self, company: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCompany)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(company)) }
    }
    pub unsafe fn Version(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetVersion(&self, version: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVersion)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(version)) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn ModuleType(&self) -> windows_core::Result<super::FsrmPipelineModuleType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), enabled) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn NeedsFileContent(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NeedsFileContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNeedsFileContent)(windows_core::Interface::as_raw(self), needsfilecontent) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn Account(&self) -> windows_core::Result<super::FsrmAccountType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Account)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn SetAccount(&self, retrievalaccount: super::FsrmAccountType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAccount)(windows_core::Interface::as_raw(self), retrievalaccount) }
    }
    pub unsafe fn SupportedExtensions(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedExtensions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSupportedExtensions)(windows_core::Interface::as_raw(self), supportedextensions) }
    }
    pub unsafe fn Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetParameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), parameters) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleDefinition_Vtbl {
    pub base__: super::IFsrmObject_Vtbl,
    pub ModuleClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetModuleClsid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Company: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompany: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub ModuleType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmPipelineModuleType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    ModuleType: usize,
    #[cfg(feature = "wtypes")]
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Enabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetEnabled: usize,
    #[cfg(feature = "wtypes")]
    pub NeedsFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    NeedsFileContent: usize,
    #[cfg(feature = "wtypes")]
    pub SetNeedsFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetNeedsFileContent: usize,
    #[cfg(feature = "fsrmenums")]
    pub Account: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmAccountType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    Account: usize,
    #[cfg(feature = "fsrmenums")]
    pub SetAccount: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmAccountType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    SetAccount: usize,
    pub SupportedExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub SetSupportedExtensions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPipelineModuleDefinition_Impl: super::IFsrmObject_Impl {
    fn ModuleClsid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetModuleClsid(&self, moduleclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Company(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCompany(&self, company: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Version(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetVersion(&self, version: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ModuleType(&self) -> windows_core::Result<super::FsrmPipelineModuleType>;
    fn Enabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetEnabled(&self, enabled: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn NeedsFileContent(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetNeedsFileContent(&self, needsfilecontent: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Account(&self) -> windows_core::Result<super::FsrmAccountType>;
    fn SetAccount(&self, retrievalaccount: super::FsrmAccountType) -> windows_core::Result<()>;
    fn SupportedExtensions(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn SetSupportedExtensions(&self, supportedextensions: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn SetParameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPipelineModuleDefinition_Vtbl {
    pub const fn new<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ModuleClsid<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleclsid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::ModuleClsid(this) {
                    Ok(ok__) => {
                        moduleclsid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModuleClsid<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduleclsid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetModuleClsid(this, core::mem::transmute(&moduleclsid)).into()
            }
        }
        unsafe extern "system" fn Name<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Company<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, company: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Company(this) {
                    Ok(ok__) => {
                        company.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCompany<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, company: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetCompany(this, core::mem::transmute(&company)).into()
            }
        }
        unsafe extern "system" fn Version<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Version(this) {
                    Ok(ok__) => {
                        version.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVersion<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, version: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetVersion(this, core::mem::transmute(&version)).into()
            }
        }
        unsafe extern "system" fn ModuleType<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduletype: *mut super::FsrmPipelineModuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::ModuleType(this) {
                    Ok(ok__) => {
                        moduletype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enabled<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Enabled(this) {
                    Ok(ok__) => {
                        enabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn NeedsFileContent<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsfilecontent: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::NeedsFileContent(this) {
                    Ok(ok__) => {
                        needsfilecontent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNeedsFileContent<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, needsfilecontent: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetNeedsFileContent(this, core::mem::transmute_copy(&needsfilecontent)).into()
            }
        }
        unsafe extern "system" fn Account<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retrievalaccount: *mut super::FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Account(this) {
                    Ok(ok__) => {
                        retrievalaccount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccount<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retrievalaccount: super::FsrmAccountType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetAccount(this, core::mem::transmute_copy(&retrievalaccount)).into()
            }
        }
        unsafe extern "system" fn SupportedExtensions<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedextensions: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::SupportedExtensions(this) {
                    Ok(ok__) => {
                        supportedextensions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSupportedExtensions<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedextensions: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetSupportedExtensions(this, core::mem::transmute_copy(&supportedextensions)).into()
            }
        }
        unsafe extern "system" fn Parameters<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPipelineModuleDefinition_Impl::Parameters(this) {
                    Ok(ok__) => {
                        parameters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParameters<Identity: IFsrmPipelineModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPipelineModuleDefinition_Impl::SetParameters(this, core::mem::transmute_copy(&parameters)).into()
            }
        }
        Self {
            base__: super::IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            ModuleClsid: ModuleClsid::<Identity, OFFSET>,
            SetModuleClsid: SetModuleClsid::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Company: Company::<Identity, OFFSET>,
            SetCompany: SetCompany::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            SetVersion: SetVersion::<Identity, OFFSET>,
            ModuleType: ModuleType::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            NeedsFileContent: NeedsFileContent::<Identity, OFFSET>,
            SetNeedsFileContent: SetNeedsFileContent::<Identity, OFFSET>,
            Account: Account::<Identity, OFFSET>,
            SetAccount: SetAccount::<Identity, OFFSET>,
            SupportedExtensions: SupportedExtensions::<Identity, OFFSET>,
            SetSupportedExtensions: SetSupportedExtensions::<Identity, OFFSET>,
            Parameters: Parameters::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPipelineModuleDefinition as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
            (windows_core::Interface::vtable(self).OnLoad)(windows_core::Interface::as_raw(self), moduledefinition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
#[cfg(all(feature = "fsrm", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPipelineModuleImplementation_Impl: super::IDispatch_Impl {
    fn OnLoad(&self, moduledefinition: windows_core::Ref<IFsrmPipelineModuleDefinition>) -> windows_core::Result<IFsrmPipelineModuleConnector>;
    fn OnUnload(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "fsrm", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Sources(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Sources)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PropertyFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmProperty_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Sources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub PropertyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmProperty_Impl: super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Sources(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn PropertyFlags(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmProperty_Vtbl {
    pub const fn new<Identity: IFsrmProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Value<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::Value(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Sources<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sources: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::Sources(this) {
                    Ok(ok__) => {
                        sources.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PropertyFlags<Identity: IFsrmProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmProperty_Impl::PropertyFlags(this) {
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
            Name: Name::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            Sources: Sources::<Identity, OFFSET>,
            PropertyFlags: PropertyFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmProperty as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RelativePath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RelativePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RelativeNamespaceRoot(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RelativeNamespaceRoot)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumeIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FileId(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ParentDirectoryId(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParentDirectoryId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Size(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SizeAllocated(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SizeAllocated)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreationTime(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn LastAccessTime(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastAccessTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn LastModificationTime(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastModificationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Attributes(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Attributes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OwnerSid(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OwnerSid)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FilePropertyNames(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FilePropertyNames)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Messages(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Messages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn PropertyBagFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyBagFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFileProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmProperty> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFileProperty(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn AddMessage(&self, message: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(message)) }
    }
    #[cfg(all(feature = "fsrmenums", feature = "wtypes", feature = "wtypesbase"))]
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
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelativePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelativeNamespaceRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub FileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    FileId: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ParentDirectoryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ParentDirectoryId: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Size: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SizeAllocated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SizeAllocated: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    CreationTime: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub LastAccessTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    LastAccessTime: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub LastModificationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    LastModificationTime: usize,
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub OwnerSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FilePropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub Messages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub PropertyBagFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFileProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "fsrmenums", feature = "wtypes", feature = "wtypesbase"))]
    pub GetFileStreamInterface: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmFileStreamingMode, super::FsrmFileStreamingInterfaceType, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrmenums", feature = "wtypes", feature = "wtypesbase")))]
    GetFileStreamInterface: usize,
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPropertyBag_Impl: super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RelativePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RelativeNamespaceRoot(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeIndex(&self) -> windows_core::Result<u32>;
    fn FileId(&self) -> windows_core::Result<super::VARIANT>;
    fn ParentDirectoryId(&self) -> windows_core::Result<super::VARIANT>;
    fn Size(&self) -> windows_core::Result<super::VARIANT>;
    fn SizeAllocated(&self) -> windows_core::Result<super::VARIANT>;
    fn CreationTime(&self) -> windows_core::Result<super::VARIANT>;
    fn LastAccessTime(&self) -> windows_core::Result<super::VARIANT>;
    fn LastModificationTime(&self) -> windows_core::Result<super::VARIANT>;
    fn Attributes(&self) -> windows_core::Result<u32>;
    fn OwnerSid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FilePropertyNames(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn Messages(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn PropertyBagFlags(&self) -> windows_core::Result<u32>;
    fn GetFileProperty(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsrmProperty>;
    fn SetFileProperty(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddMessage(&self, message: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetFileStreamInterface(&self, accessmode: super::FsrmFileStreamingMode, interfacetype: super::FsrmFileStreamingInterfaceType) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPropertyBag_Vtbl {
    pub const fn new<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RelativePath<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::RelativePath(this) {
                    Ok(ok__) => {
                        path.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeName<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volumename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::VolumeName(this) {
                    Ok(ok__) => {
                        volumename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RelativeNamespaceRoot<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativenamespaceroot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::RelativeNamespaceRoot(this) {
                    Ok(ok__) => {
                        relativenamespaceroot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeIndex<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volumeid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::VolumeIndex(this) {
                    Ok(ok__) => {
                        volumeid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FileId<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fileid: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::FileId(this) {
                    Ok(ok__) => {
                        fileid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ParentDirectoryId<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentdirectoryid: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::ParentDirectoryId(this) {
                    Ok(ok__) => {
                        parentdirectoryid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::Size(this) {
                    Ok(ok__) => {
                        size.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SizeAllocated<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sizeallocated: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::SizeAllocated(this) {
                    Ok(ok__) => {
                        sizeallocated.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreationTime<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, creationtime: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::CreationTime(this) {
                    Ok(ok__) => {
                        creationtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastAccessTime<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastaccesstime: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::LastAccessTime(this) {
                    Ok(ok__) => {
                        lastaccesstime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastModificationTime<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodificationtime: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::LastModificationTime(this) {
                    Ok(ok__) => {
                        lastmodificationtime.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Attributes<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::Attributes(this) {
                    Ok(ok__) => {
                        attributes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OwnerSid<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ownersid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::OwnerSid(this) {
                    Ok(ok__) => {
                        ownersid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FilePropertyNames<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filepropertynames: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::FilePropertyNames(this) {
                    Ok(ok__) => {
                        filepropertynames.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Messages<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messages: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::Messages(this) {
                    Ok(ok__) => {
                        messages.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PropertyBagFlags<Identity: IFsrmPropertyBag_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyBag_Impl::PropertyBagFlags(this) {
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
            Name: Name::<Identity, OFFSET>,
            RelativePath: RelativePath::<Identity, OFFSET>,
            VolumeName: VolumeName::<Identity, OFFSET>,
            RelativeNamespaceRoot: RelativeNamespaceRoot::<Identity, OFFSET>,
            VolumeIndex: VolumeIndex::<Identity, OFFSET>,
            FileId: FileId::<Identity, OFFSET>,
            ParentDirectoryId: ParentDirectoryId::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            SizeAllocated: SizeAllocated::<Identity, OFFSET>,
            CreationTime: CreationTime::<Identity, OFFSET>,
            LastAccessTime: LastAccessTime::<Identity, OFFSET>,
            LastModificationTime: LastModificationTime::<Identity, OFFSET>,
            Attributes: Attributes::<Identity, OFFSET>,
            OwnerSid: OwnerSid::<Identity, OFFSET>,
            FilePropertyNames: FilePropertyNames::<Identity, OFFSET>,
            Messages: Messages::<Identity, OFFSET>,
            PropertyBagFlags: PropertyBagFlags::<Identity, OFFSET>,
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
#[cfg(all(feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "fsrmenums", feature = "wtypes", feature = "wtypesbase"))]
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
            (windows_core::Interface::vtable(self).GetUntrustedInFileProperties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag2_Vtbl {
    pub base__: IFsrmPropertyBag_Vtbl,
    #[cfg(all(feature = "fsrmenums", feature = "wtypes", feature = "wtypesbase"))]
    pub GetFieldValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmPropertyBagField, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "fsrmenums", feature = "wtypes", feature = "wtypesbase")))]
    GetFieldValue: usize,
    #[cfg(feature = "fsrm")]
    pub GetUntrustedInFileProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrm"))]
    GetUntrustedInFileProperties: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPropertyBag2_Impl: IFsrmPropertyBag_Impl {
    fn GetFieldValue(&self, field: super::FsrmPropertyBagField) -> windows_core::Result<super::VARIANT>;
    fn GetUntrustedInFileProperties(&self) -> windows_core::Result<super::IFsrmCollection>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn Type(&self) -> windows_core::Result<super::FsrmPropertyDefinitionType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn SetType(&self, r#type: super::FsrmPropertyDefinitionType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), r#type) }
    }
    pub unsafe fn PossibleValues(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PossibleValues)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPossibleValues)(windows_core::Interface::as_raw(self), possiblevalues) }
    }
    pub unsafe fn ValueDescriptions(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValueDescriptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValueDescriptions)(windows_core::Interface::as_raw(self), valuedescriptions) }
    }
    pub unsafe fn Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetParameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), parameters) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition_Vtbl {
    pub base__: super::IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmPropertyDefinitionType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    Type: usize,
    #[cfg(feature = "fsrmenums")]
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmPropertyDefinitionType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    SetType: usize,
    pub PossibleValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub SetPossibleValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    pub ValueDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub SetValueDescriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPropertyDefinition_Impl: super::IFsrmObject_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<super::FsrmPropertyDefinitionType>;
    fn SetType(&self, r#type: super::FsrmPropertyDefinitionType) -> windows_core::Result<()>;
    fn PossibleValues(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn SetPossibleValues(&self, possiblevalues: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn ValueDescriptions(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn SetValueDescriptions(&self, valuedescriptions: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn SetParameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPropertyDefinition_Vtbl {
    pub const fn new<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn Type<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut super::FsrmPropertyDefinitionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::Type(this) {
                    Ok(ok__) => {
                        r#type.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: super::FsrmPropertyDefinitionType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::SetType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn PossibleValues<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, possiblevalues: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::PossibleValues(this) {
                    Ok(ok__) => {
                        possiblevalues.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPossibleValues<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, possiblevalues: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::SetPossibleValues(this, core::mem::transmute_copy(&possiblevalues)).into()
            }
        }
        unsafe extern "system" fn ValueDescriptions<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuedescriptions: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::ValueDescriptions(this) {
                    Ok(ok__) => {
                        valuedescriptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValueDescriptions<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuedescriptions: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::SetValueDescriptions(this, core::mem::transmute_copy(&valuedescriptions)).into()
            }
        }
        unsafe extern "system" fn Parameters<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition_Impl::Parameters(this) {
                    Ok(ok__) => {
                        parameters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParameters<Identity: IFsrmPropertyDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition_Impl::SetParameters(this, core::mem::transmute_copy(&parameters)).into()
            }
        }
        Self {
            base__: super::IFsrmObject_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            PossibleValues: PossibleValues::<Identity, OFFSET>,
            SetPossibleValues: SetPossibleValues::<Identity, OFFSET>,
            ValueDescriptions: ValueDescriptions::<Identity, OFFSET>,
            SetValueDescriptions: SetValueDescriptions::<Identity, OFFSET>,
            Parameters: Parameters::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn PropertyDefinitionFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyDefinitionFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    pub unsafe fn AppliesTo(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppliesTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ValueDefinitions(&self) -> windows_core::Result<super::IFsrmCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValueDefinitions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition2_Vtbl {
    pub base__: IFsrmPropertyDefinition_Vtbl,
    pub PropertyDefinitionFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppliesTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ValueDefinitions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPropertyDefinition2_Impl: IFsrmPropertyDefinition_Impl {
    fn PropertyDefinitionFlags(&self) -> windows_core::Result<i32>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDisplayName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AppliesTo(&self) -> windows_core::Result<i32>;
    fn ValueDefinitions(&self) -> windows_core::Result<super::IFsrmCollection>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPropertyDefinition2_Vtbl {
    pub const fn new<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PropertyDefinitionFlags<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertydefinitionflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::PropertyDefinitionFlags(this) {
                    Ok(ok__) => {
                        propertydefinitionflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmPropertyDefinition2_Impl::SetDisplayName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn AppliesTo<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, appliesto: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::AppliesTo(this) {
                    Ok(ok__) => {
                        appliesto.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ValueDefinitions<Identity: IFsrmPropertyDefinition2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuedefinitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinition2_Impl::ValueDefinitions(this) {
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
            PropertyDefinitionFlags: PropertyDefinitionFlags::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            AppliesTo: AppliesTo::<Identity, OFFSET>,
            ValueDefinitions: ValueDefinitions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinition2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmPropertyDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UniqueID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UniqueID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinitionValue_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UniqueID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmPropertyDefinitionValue_Impl: super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UniqueID(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmPropertyDefinitionValue_Vtbl {
    pub const fn new<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayName<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::DisplayName(this) {
                    Ok(ok__) => {
                        displayname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::Description(this) {
                    Ok(ok__) => {
                        description.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UniqueID<Identity: IFsrmPropertyDefinitionValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uniqueid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmPropertyDefinitionValue_Impl::UniqueID(this) {
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
            Name: Name::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            UniqueID: UniqueID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmPropertyDefinitionValue as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn RuleType(&self) -> windows_core::Result<super::FsrmRuleType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RuleType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ModuleDefinitionName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleDefinitionName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetModuleDefinitionName(&self, moduledefinitionname: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModuleDefinitionName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(moduledefinitionname)) }
    }
    pub unsafe fn NamespaceRoots(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NamespaceRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNamespaceRoots)(windows_core::Interface::as_raw(self), namespaceroots) }
    }
    pub unsafe fn RuleFlags(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RuleFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRuleFlags)(windows_core::Interface::as_raw(self), ruleflags) }
    }
    pub unsafe fn Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parameters)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetParameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), parameters) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn LastModified(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmRule_Vtbl {
    pub base__: super::IFsrmObject_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "fsrmenums")]
    pub RuleType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmRuleType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    RuleType: usize,
    pub ModuleDefinitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetModuleDefinitionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub SetNamespaceRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    pub RuleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetRuleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Parameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub LastModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    LastModified: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmRule_Impl: super::IFsrmObject_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RuleType(&self) -> windows_core::Result<super::FsrmRuleType>;
    fn ModuleDefinitionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetModuleDefinitionName(&self, moduledefinitionname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NamespaceRoots(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn SetNamespaceRoots(&self, namespaceroots: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn RuleFlags(&self) -> windows_core::Result<i32>;
    fn SetRuleFlags(&self, ruleflags: i32) -> windows_core::Result<()>;
    fn Parameters(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn SetParameters(&self, parameters: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn LastModified(&self) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmRule_Vtbl {
    pub const fn new<Identity: IFsrmRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::Name(this) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::SetName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn RuleType<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruletype: *mut super::FsrmRuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::RuleType(this) {
                    Ok(ok__) => {
                        ruletype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModuleDefinitionName<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduledefinitionname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::ModuleDefinitionName(this) {
                    Ok(ok__) => {
                        moduledefinitionname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModuleDefinitionName<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, moduledefinitionname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::SetModuleDefinitionName(this, core::mem::transmute(&moduledefinitionname)).into()
            }
        }
        unsafe extern "system" fn NamespaceRoots<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceroots: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::NamespaceRoots(this) {
                    Ok(ok__) => {
                        namespaceroots.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamespaceRoots<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namespaceroots: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::SetNamespaceRoots(this, core::mem::transmute_copy(&namespaceroots)).into()
            }
        }
        unsafe extern "system" fn RuleFlags<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruleflags: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::RuleFlags(this) {
                    Ok(ok__) => {
                        ruleflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRuleFlags<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruleflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::SetRuleFlags(this, core::mem::transmute_copy(&ruleflags)).into()
            }
        }
        unsafe extern "system" fn Parameters<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::Parameters(this) {
                    Ok(ok__) => {
                        parameters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetParameters<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameters: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmRule_Impl::SetParameters(this, core::mem::transmute_copy(&parameters)).into()
            }
        }
        unsafe extern "system" fn LastModified<Identity: IFsrmRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lastmodified: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmRule_Impl::LastModified(this) {
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
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            RuleType: RuleType::<Identity, OFFSET>,
            ModuleDefinitionName: ModuleDefinitionName::<Identity, OFFSET>,
            SetModuleDefinitionName: SetModuleDefinitionName::<Identity, OFFSET>,
            NamespaceRoots: NamespaceRoots::<Identity, OFFSET>,
            SetNamespaceRoots: SetNamespaceRoots::<Identity, OFFSET>,
            RuleFlags: RuleFlags::<Identity, OFFSET>,
            SetRuleFlags: SetRuleFlags::<Identity, OFFSET>,
            Parameters: Parameters::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
            LastModified: LastModified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmRule as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Capabilities(&self) -> windows_core::Result<super::FsrmStorageModuleCaps> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Capabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn SetCapabilities(&self, capabilities: super::FsrmStorageModuleCaps) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCapabilities)(windows_core::Interface::as_raw(self), capabilities) }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn StorageType(&self) -> windows_core::Result<super::FsrmStorageModuleType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StorageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "fsrmenums")]
    pub unsafe fn SetStorageType(&self, storagetype: super::FsrmStorageModuleType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStorageType)(windows_core::Interface::as_raw(self), storagetype) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn UpdatesFileContent(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdatesFileContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetUpdatesFileContent(&self, updatesfilecontent: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUpdatesFileContent)(windows_core::Interface::as_raw(self), updatesfilecontent) }
    }
}
#[cfg(all(feature = "fsrm", feature = "oaidl"))]
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleDefinition_Vtbl {
    pub base__: IFsrmPipelineModuleDefinition_Vtbl,
    #[cfg(feature = "fsrmenums")]
    pub Capabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmStorageModuleCaps) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    Capabilities: usize,
    #[cfg(feature = "fsrmenums")]
    pub SetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmStorageModuleCaps) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    SetCapabilities: usize,
    #[cfg(feature = "fsrmenums")]
    pub StorageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FsrmStorageModuleType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    StorageType: usize,
    #[cfg(feature = "fsrmenums")]
    pub SetStorageType: unsafe extern "system" fn(*mut core::ffi::c_void, super::FsrmStorageModuleType) -> windows_core::HRESULT,
    #[cfg(not(feature = "fsrmenums"))]
    SetStorageType: usize,
    #[cfg(feature = "wtypes")]
    pub UpdatesFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    UpdatesFileContent: usize,
    #[cfg(feature = "wtypes")]
    pub SetUpdatesFileContent: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetUpdatesFileContent: usize,
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmStorageModuleDefinition_Impl: IFsrmPipelineModuleDefinition_Impl {
    fn Capabilities(&self) -> windows_core::Result<super::FsrmStorageModuleCaps>;
    fn SetCapabilities(&self, capabilities: super::FsrmStorageModuleCaps) -> windows_core::Result<()>;
    fn StorageType(&self) -> windows_core::Result<super::FsrmStorageModuleType>;
    fn SetStorageType(&self, storagetype: super::FsrmStorageModuleType) -> windows_core::Result<()>;
    fn UpdatesFileContent(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn SetUpdatesFileContent(&self, updatesfilecontent: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsrmStorageModuleDefinition_Vtbl {
    pub const fn new<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Capabilities<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilities: *mut super::FsrmStorageModuleCaps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmStorageModuleDefinition_Impl::Capabilities(this) {
                    Ok(ok__) => {
                        capabilities.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCapabilities<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, capabilities: super::FsrmStorageModuleCaps) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleDefinition_Impl::SetCapabilities(this, core::mem::transmute_copy(&capabilities)).into()
            }
        }
        unsafe extern "system" fn StorageType<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storagetype: *mut super::FsrmStorageModuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmStorageModuleDefinition_Impl::StorageType(this) {
                    Ok(ok__) => {
                        storagetype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStorageType<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, storagetype: super::FsrmStorageModuleType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleDefinition_Impl::SetStorageType(this, core::mem::transmute_copy(&storagetype)).into()
            }
        }
        unsafe extern "system" fn UpdatesFileContent<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatesfilecontent: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsrmStorageModuleDefinition_Impl::UpdatesFileContent(this) {
                    Ok(ok__) => {
                        updatesfilecontent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUpdatesFileContent<Identity: IFsrmStorageModuleDefinition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatesfilecontent: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsrmStorageModuleDefinition_Impl::SetUpdatesFileContent(this, core::mem::transmute_copy(&updatesfilecontent)).into()
            }
        }
        Self {
            base__: IFsrmPipelineModuleDefinition_Vtbl::new::<Identity, OFFSET>(),
            Capabilities: Capabilities::<Identity, OFFSET>,
            SetCapabilities: SetCapabilities::<Identity, OFFSET>,
            StorageType: StorageType::<Identity, OFFSET>,
            SetStorageType: SetStorageType::<Identity, OFFSET>,
            UpdatesFileContent: UpdatesFileContent::<Identity, OFFSET>,
            SetUpdatesFileContent: SetUpdatesFileContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsrmStorageModuleDefinition as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<super::IFsrmObject as windows_core::Interface>::IID || iid == &<IFsrmPipelineModuleDefinition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "fsrm", feature = "fsrmenums", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "fsrm", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsrmStorageModuleImplementation_Impl: IFsrmPipelineModuleImplementation_Impl {
    fn UseDefinitions(&self, propertydefinitions: windows_core::Ref<super::IFsrmCollection>) -> windows_core::Result<()>;
    fn LoadProperties(&self, propertybag: windows_core::Ref<IFsrmPropertyBag>) -> windows_core::Result<()>;
    fn SaveProperties(&self, propertybag: windows_core::Ref<IFsrmPropertyBag>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "fsrm", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "fsrm", feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsrmStorageModuleImplementation {}
pub const MessageSizeLimit: u32 = 4096;
