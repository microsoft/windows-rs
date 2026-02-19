#[cfg(feature = "AI_Actions_Hosting")]
pub mod Hosting;
#[cfg(feature = "AI_Actions_Provider")]
pub mod Provider;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActionEntity, super::super::Foundation::IClosable);
impl ActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionEntity>();
}
unsafe impl windows_core::Interface for ActionEntity {
    type Vtable = <IActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.ActionEntity";
}
unsafe impl Send for ActionEntity {}
unsafe impl Sync for ActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionEntityDisplayInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionEntityDisplayInfo, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActionEntityDisplayInfo, super::super::Foundation::IClosable);
impl ActionEntityDisplayInfo {
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActionEntityDisplayInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionEntityDisplayInfo>();
}
unsafe impl windows_core::Interface for ActionEntityDisplayInfo {
    type Vtable = <IActionEntityDisplayInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionEntityDisplayInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionEntityDisplayInfo {
    const NAME: &'static str = "Windows.AI.Actions.ActionEntityDisplayInfo";
}
unsafe impl Send for ActionEntityDisplayInfo {}
unsafe impl Sync for ActionEntityDisplayInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionEntityFactory(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionEntityFactory, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActionEntityFactory, super::super::Foundation::IClosable);
impl ActionEntityFactory {
    pub fn CreateFileEntity(&self, path: &windows_core::HSTRING) -> windows_core::Result<FileActionEntity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFileEntity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateDocumentEntity(&self, path: &windows_core::HSTRING) -> windows_core::Result<DocumentActionEntity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDocumentEntity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreatePhotoEntity(&self, path: &windows_core::HSTRING) -> windows_core::Result<PhotoActionEntity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePhotoEntity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateTextEntity(&self, text: &windows_core::HSTRING) -> windows_core::Result<TextActionEntity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTextEntity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateRemoteFileEntity<P2>(&self, sourceid: &windows_core::HSTRING, filekind: RemoteFileKind, sourceuri: P2, fileid: &windows_core::HSTRING, contenttype: &windows_core::HSTRING, driveid: &windows_core::HSTRING, accountid: &windows_core::HSTRING, extension: &windows_core::HSTRING) -> windows_core::Result<RemoteFileActionEntity>
    where
        P2: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IActionEntityFactory3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRemoteFileEntity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sourceid), filekind, sourceuri.param().abi(), core::mem::transmute_copy(fileid), core::mem::transmute_copy(contenttype), core::mem::transmute_copy(driveid), core::mem::transmute_copy(accountid), core::mem::transmute_copy(extension), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateTextEntityWithTextFormat(&self, text: &windows_core::HSTRING, textformat: ActionEntityTextFormat) -> windows_core::Result<TextActionEntity> {
        let this = &windows_core::Interface::cast::<IActionEntityFactory3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTextEntityWithTextFormat)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), textformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateStreamingTextActionEntityWriter(&self, textformat: ActionEntityTextFormat) -> windows_core::Result<StreamingTextActionEntityWriter> {
        let this = &windows_core::Interface::cast::<IActionEntityFactory3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateStreamingTextActionEntityWriter)(windows_core::Interface::as_raw(this), textformat, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateTableEntity(&self, data: &[windows_core::HSTRING], columncount: u32) -> windows_core::Result<TableActionEntity> {
        let this = &windows_core::Interface::cast::<IActionEntityFactory4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTableEntity)(windows_core::Interface::as_raw(this), data.len().try_into().unwrap(), core::mem::transmute(data.as_ptr()), columncount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn CreateContactEntity<P0>(&self, contact: P0) -> windows_core::Result<ContactActionEntity>
    where
        P0: windows_core::Param<super::super::ApplicationModel::Contacts::Contact>,
    {
        let this = &windows_core::Interface::cast::<IActionEntityFactory4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateContactEntity)(windows_core::Interface::as_raw(this), contact.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateUriEntity<P0>(&self, uri: P0) -> windows_core::Result<UriActionEntity>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IActionEntityFactory5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUriEntity)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateArrayEntity(&self, kind: ActionEntityKind, entities: &[Option<ActionEntity>]) -> windows_core::Result<ArrayActionEntity> {
        let this = &windows_core::Interface::cast::<IActionEntityFactory5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateArrayEntity)(windows_core::Interface::as_raw(this), kind, entities.len().try_into().unwrap(), core::mem::transmute(entities.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateDateTimeEntity(&self, datetime: super::super::Foundation::DateTime) -> windows_core::Result<DateTimeActionEntity> {
        let this = &windows_core::Interface::cast::<IActionEntityFactory6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTimeEntity)(windows_core::Interface::as_raw(this), datetime, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub fn CreateAppointmentEntity<P1>(&self, sourceid: &windows_core::HSTRING, appointment: P1, attendees: &[Option<ContactActionEntity>]) -> windows_core::Result<AppointmentActionEntity>
    where
        P1: windows_core::Param<super::super::ApplicationModel::Appointments::Appointment>,
    {
        let this = &windows_core::Interface::cast::<IActionEntityFactory6>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAppointmentEntity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(sourceid), appointment.param().abi(), attendees.len().try_into().unwrap(), core::mem::transmute(attendees.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateCustomTextEntity<P2>(&self, kind: &windows_core::HSTRING, keyphrase: &windows_core::HSTRING, props: P2) -> windows_core::Result<CustomTextActionEntity>
    where
        P2: windows_core::Param<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IActionEntityFactory7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCustomTextEntity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(kind), core::mem::transmute_copy(keyphrase), props.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateArrayEntityWithCustomKind(&self, elementkind: ActionEntityKind, customkind: &windows_core::HSTRING, entities: &[Option<ActionEntity>]) -> windows_core::Result<ArrayActionEntity> {
        let this = &windows_core::Interface::cast::<IActionEntityFactory7>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateArrayEntityWithCustomKind)(windows_core::Interface::as_raw(this), elementkind, core::mem::transmute_copy(customkind), entities.len().try_into().unwrap(), core::mem::transmute(entities.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActionEntityFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionEntityFactory2>();
}
unsafe impl windows_core::Interface for ActionEntityFactory {
    type Vtable = <IActionEntityFactory2 as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionEntityFactory2 as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionEntityFactory {
    const NAME: &'static str = "Windows.AI.Actions.ActionEntityFactory";
}
unsafe impl Send for ActionEntityFactory {}
unsafe impl Sync for ActionEntityFactory {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ActionEntityKind(pub i32);
impl ActionEntityKind {
    pub const None: Self = Self(0i32);
    pub const Document: Self = Self(1i32);
    pub const File: Self = Self(2i32);
    pub const Photo: Self = Self(3i32);
    pub const Text: Self = Self(4i32);
    pub const StreamingText: Self = Self(5i32);
    pub const RemoteFile: Self = Self(6i32);
    pub const Table: Self = Self(7i32);
    pub const Contact: Self = Self(8i32);
    pub const Uri: Self = Self(9i32);
    pub const Array: Self = Self(10i32);
    pub const Appointment: Self = Self(11i32);
    pub const Date: Self = Self(12i32);
    pub const CustomText: Self = Self(13i32);
}
impl windows_core::TypeKind for ActionEntityKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ActionEntityKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.Actions.ActionEntityKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ActionEntityTextFormat(pub i32);
impl ActionEntityTextFormat {
    pub const Plain: Self = Self(0i32);
    pub const Markdown: Self = Self(1i32);
}
impl windows_core::TypeKind for ActionEntityTextFormat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ActionEntityTextFormat {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.Actions.ActionEntityTextFormat;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionFeedback(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionFeedback, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActionFeedback, super::super::Foundation::IClosable);
impl ActionFeedback {
    pub fn FeedbackKind(&self) -> windows_core::Result<ActionFeedbackKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FeedbackKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActionFeedback {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionFeedback>();
}
unsafe impl windows_core::Interface for ActionFeedback {
    type Vtable = <IActionFeedback as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionFeedback as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionFeedback {
    const NAME: &'static str = "Windows.AI.Actions.ActionFeedback";
}
unsafe impl Send for ActionFeedback {}
unsafe impl Sync for ActionFeedback {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ActionFeedbackKind(pub i32);
impl ActionFeedbackKind {
    pub const Positive: Self = Self(0i32);
    pub const Negative: Self = Self(1i32);
}
impl windows_core::TypeKind for ActionFeedbackKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ActionFeedbackKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.Actions.ActionFeedbackKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionInvocationContext(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionInvocationContext, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActionInvocationContext, super::super::Foundation::IClosable);
impl ActionInvocationContext {
    pub fn EntityFactory(&self) -> windows_core::Result<ActionEntityFactory> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EntityFactory)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInputEntity<P1>(&self, inputname: &windows_core::HSTRING, inputvalue: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ActionEntity>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInputEntity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(inputname), inputvalue.param().abi()).ok() }
    }
    pub fn GetInputEntities(&self) -> windows_core::Result<windows_core::Array<NamedActionEntity>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetInputEntities)(windows_core::Interface::as_raw(this), windows_core::Array::<NamedActionEntity>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetOutputEntity<P1>(&self, outputname: &windows_core::HSTRING, outputvalue: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ActionEntity>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetOutputEntity)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(outputname), outputvalue.param().abi()).ok() }
    }
    pub fn GetOutputEntities(&self) -> windows_core::Result<windows_core::Array<NamedActionEntity>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetOutputEntities)(windows_core::Interface::as_raw(this), windows_core::Array::<NamedActionEntity>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn Result(&self) -> windows_core::Result<ActionInvocationResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Result)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetResult(&self, value: ActionInvocationResult) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResult)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExtendedError(&self, value: windows_core::HRESULT) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetExtendedError)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    pub fn InvokerWindowId(&self) -> windows_core::Result<super::super::UI::WindowId> {
        let this = &windows_core::Interface::cast::<IActionInvocationContext2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvokerWindowId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HelpDetails(&self) -> windows_core::Result<ActionInvocationHelpDetails> {
        let this = &windows_core::Interface::cast::<IActionInvocationContext2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HelpDetails)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ActionId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionInvocationContext2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn InvokerAppUserModelId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionInvocationContext2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InvokerAppUserModelId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActionInvocationContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionInvocationContext>();
}
unsafe impl windows_core::Interface for ActionInvocationContext {
    type Vtable = <IActionInvocationContext as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionInvocationContext as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionInvocationContext {
    const NAME: &'static str = "Windows.AI.Actions.ActionInvocationContext";
}
unsafe impl Send for ActionInvocationContext {}
unsafe impl Sync for ActionInvocationContext {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionInvocationHelpDetails(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionInvocationHelpDetails, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActionInvocationHelpDetails, super::super::Foundation::IClosable);
impl ActionInvocationHelpDetails {
    pub fn Kind(&self) -> windows_core::Result<ActionInvocationHelpKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKind(&self, value: ActionInvocationHelpKind) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTitle)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetDescription)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn HelpUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HelpUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetHelpUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHelpUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn HelpUriDescription(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HelpUriDescription)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetHelpUriDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHelpUriDescription)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Changed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<ActionInvocationHelpDetails, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<IActionInvocationHelpDetails2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Changed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IActionInvocationHelpDetails2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ActionInvocationHelpDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionInvocationHelpDetails>();
}
unsafe impl windows_core::Interface for ActionInvocationHelpDetails {
    type Vtable = <IActionInvocationHelpDetails as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionInvocationHelpDetails as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionInvocationHelpDetails {
    const NAME: &'static str = "Windows.AI.Actions.ActionInvocationHelpDetails";
}
unsafe impl Send for ActionInvocationHelpDetails {}
unsafe impl Sync for ActionInvocationHelpDetails {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ActionInvocationHelpKind(pub i32);
impl ActionInvocationHelpKind {
    pub const None: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const Warning: Self = Self(2i32);
}
impl windows_core::TypeKind for ActionInvocationHelpKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ActionInvocationHelpKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.Actions.ActionInvocationHelpKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ActionInvocationResult(pub i32);
impl ActionInvocationResult {
    pub const Success: Self = Self(0i32);
    pub const UserCanceled: Self = Self(1i32);
    pub const Unsupported: Self = Self(2i32);
    pub const Unavailable: Self = Self(3i32);
}
impl windows_core::TypeKind for ActionInvocationResult {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ActionInvocationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.Actions.ActionInvocationResult;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionRuntime(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ActionRuntime, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ActionRuntime, super::super::Foundation::IClosable);
impl ActionRuntime {
    #[cfg(feature = "AI_Actions_Hosting")]
    pub fn ActionCatalog(&self) -> windows_core::Result<Hosting::ActionCatalog> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActionCatalog)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EntityFactory(&self) -> windows_core::Result<ActionEntityFactory> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EntityFactory)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInvocationContext(&self, actionid: &windows_core::HSTRING) -> windows_core::Result<ActionInvocationContext> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInvocationContext)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(actionid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateActionFeedback(&self, feedbackkind: ActionFeedbackKind) -> windows_core::Result<ActionFeedback> {
        let this = &windows_core::Interface::cast::<IActionRuntime2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateActionFeedback)(windows_core::Interface::as_raw(this), feedbackkind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetActionAvailability(&self, actionid: &windows_core::HSTRING, isavailable: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IActionRuntime2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetActionAvailability)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(actionid), isavailable).ok() }
    }
    pub fn GetActionAvailability(&self, actionid: &windows_core::HSTRING) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IActionRuntime2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetActionAvailability)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(actionid), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI")]
    pub fn CreateInvocationContextWithWindowId(&self, actionid: &windows_core::HSTRING, invokerwindowid: super::super::UI::WindowId) -> windows_core::Result<ActionInvocationContext> {
        let this = &windows_core::Interface::cast::<IActionRuntime3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInvocationContextWithWindowId)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(actionid), invokerwindowid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetActionEntityById(&self, entityid: &windows_core::HSTRING) -> windows_core::Result<ActionEntity> {
        let this = &windows_core::Interface::cast::<IActionRuntime3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetActionEntityById)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(entityid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LatestSupportedSchemaVersion(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IActionRuntime3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LatestSupportedSchemaVersion)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetActionInvocationContextFromToken(&self, token: &windows_core::HSTRING) -> windows_core::Result<ActionInvocationContext> {
        let this = &windows_core::Interface::cast::<IActionRuntime4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetActionInvocationContextFromToken)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(token), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CustomEntityStore(&self) -> windows_core::Result<CustomActionEntityStore> {
        let this = &windows_core::Interface::cast::<IActionRuntime5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomEntityStore)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<ActionRuntime> {
        Self::IActionRuntimeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    fn IActionRuntimeStatics<R, F: FnOnce(&IActionRuntimeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ActionRuntime, IActionRuntimeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ActionRuntime {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IActionRuntime>();
}
unsafe impl windows_core::Interface for ActionRuntime {
    type Vtable = <IActionRuntime as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IActionRuntime as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ActionRuntime {
    const NAME: &'static str = "Windows.AI.Actions.ActionRuntime";
}
unsafe impl Send for ActionRuntime {}
unsafe impl Sync for ActionRuntime {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppointmentActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppointmentActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(AppointmentActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl AppointmentActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SourceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub fn Appointment(&self) -> windows_core::Result<super::super::ApplicationModel::Appointments::Appointment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Appointment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAttendees(&self) -> windows_core::Result<windows_core::Array<ContactActionEntity>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetAttendees)(windows_core::Interface::as_raw(this), windows_core::Array::<ContactActionEntity>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetPresentedFiles(&self) -> windows_core::Result<windows_core::Array<RemoteFileActionEntity>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetPresentedFiles)(windows_core::Interface::as_raw(this), windows_core::Array::<RemoteFileActionEntity>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetPresentedFiles(&self, files: &[Option<RemoteFileActionEntity>]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPresentedFiles)(windows_core::Interface::as_raw(this), files.len().try_into().unwrap(), core::mem::transmute(files.as_ptr())).ok() }
    }
    pub fn GetSharedFiles(&self) -> windows_core::Result<windows_core::Array<RemoteFileActionEntity>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetSharedFiles)(windows_core::Interface::as_raw(this), windows_core::Array::<RemoteFileActionEntity>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetSharedFiles(&self, files: &[Option<RemoteFileActionEntity>]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSharedFiles)(windows_core::Interface::as_raw(this), files.len().try_into().unwrap(), core::mem::transmute(files.as_ptr())).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for AppointmentActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppointmentActionEntity>();
}
unsafe impl windows_core::Interface for AppointmentActionEntity {
    type Vtable = <IAppointmentActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppointmentActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppointmentActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.AppointmentActionEntity";
}
unsafe impl Send for AppointmentActionEntity {}
unsafe impl Sync for AppointmentActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArrayActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ArrayActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ArrayActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl ArrayActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ElementKind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetAll(&self) -> windows_core::Result<windows_core::Array<ActionEntity>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetAll)(windows_core::Interface::as_raw(this), windows_core::Array::<ActionEntity>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn CustomElementKind(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IArrayActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomElementKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ArrayActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IArrayActionEntity>();
}
unsafe impl windows_core::Interface for ArrayActionEntity {
    type Vtable = <IArrayActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IArrayActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ArrayActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.ArrayActionEntity";
}
unsafe impl Send for ArrayActionEntity {}
unsafe impl Sync for ArrayActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ContactActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl ContactActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> windows_core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Contact)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactActionEntity>();
}
unsafe impl windows_core::Interface for ContactActionEntity {
    type Vtable = <IContactActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContactActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.ContactActionEntity";
}
unsafe impl Send for ContactActionEntity {}
unsafe impl Sync for ContactActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomActionEntityStore(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CustomActionEntityStore, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CustomActionEntityStore, super::super::Foundation::IClosable);
impl CustomActionEntityStore {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetLastModifiedTime(&self, kind: &windows_core::HSTRING) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetLastModifiedTime)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(kind), &mut result__).map(|| result__)
        }
    }
    pub fn Insert<P0>(&self, entity: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CustomTextActionEntity>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), entity.param().abi()).ok() }
    }
    pub fn InsertMany(&self, entities: &[Option<CustomTextActionEntity>]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InsertMany)(windows_core::Interface::as_raw(this), entities.len().try_into().unwrap(), core::mem::transmute(entities.as_ptr())).ok() }
    }
    pub fn Delete(&self, kind: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Delete)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(kind)).ok() }
    }
}
impl windows_core::RuntimeType for CustomActionEntityStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICustomActionEntityStore>();
}
unsafe impl windows_core::Interface for CustomActionEntityStore {
    type Vtable = <ICustomActionEntityStore as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICustomActionEntityStore as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CustomActionEntityStore {
    const NAME: &'static str = "Windows.AI.Actions.CustomActionEntityStore";
}
unsafe impl Send for CustomActionEntityStore {}
unsafe impl Sync for CustomActionEntityStore {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomTextActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CustomTextActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CustomTextActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl CustomTextActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CustomTextKind(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CustomTextKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn KeyPhrase(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyPhrase)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Properties(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Properties)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CustomTextActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICustomTextActionEntity>();
}
unsafe impl windows_core::Interface for CustomTextActionEntity {
    type Vtable = <ICustomTextActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICustomTextActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CustomTextActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.CustomTextActionEntity";
}
unsafe impl Send for CustomTextActionEntity {}
unsafe impl Sync for CustomTextActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateTimeActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DateTimeActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DateTimeActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl DateTimeActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DateTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DateTimeActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDateTimeActionEntity>();
}
unsafe impl windows_core::Interface for DateTimeActionEntity {
    type Vtable = <IDateTimeActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDateTimeActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DateTimeActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.DateTimeActionEntity";
}
unsafe impl Send for DateTimeActionEntity {}
unsafe impl Sync for DateTimeActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DocumentActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DocumentActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl DocumentActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FullPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FullPath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for DocumentActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDocumentActionEntity>();
}
unsafe impl windows_core::Interface for DocumentActionEntity {
    type Vtable = <IDocumentActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDocumentActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DocumentActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.DocumentActionEntity";
}
unsafe impl Send for DocumentActionEntity {}
unsafe impl Sync for DocumentActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(FileActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl FileActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FullPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FullPath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for FileActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFileActionEntity>();
}
unsafe impl windows_core::Interface for FileActionEntity {
    type Vtable = <IFileActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFileActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.FileActionEntity";
}
unsafe impl Send for FileActionEntity {}
unsafe impl Sync for FileActionEntity {}
windows_core::imp::define_interface!(IActionEntity, IActionEntity_Vtbl, 0x445e700f_2122_5668_9a16_4cab2982c5f4);
impl windows_core::RuntimeType for IActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActionEntityKind) -> windows_core::HRESULT,
    pub DisplayInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionEntity2, IActionEntity2_Vtbl, 0x98fe136d_dd3a_58c1_af76_feb4e19dce9e);
impl windows_core::RuntimeType for IActionEntity2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntity2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionEntityDisplayInfo, IActionEntityDisplayInfo_Vtbl, 0x057a9ede_03e1_55c6_acba_c7056216735a);
impl windows_core::RuntimeType for IActionEntityDisplayInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntityDisplayInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionEntityFactory, IActionEntityFactory_Vtbl, 0x9cb752a0_5bf8_5be2_916e_b00eff80088d);
impl windows_core::RuntimeType for IActionEntityFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntityFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IActionEntityFactory2, IActionEntityFactory2_Vtbl, 0xea2fb6a5_ec6d_5180_9d30_bc663b84e7b8);
impl windows_core::RuntimeType for IActionEntityFactory2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntityFactory2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFileEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDocumentEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePhotoEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionEntityFactory3, IActionEntityFactory3_Vtbl, 0x4910e689_00b5_56bb_9c65_0fcc76215283);
impl windows_core::RuntimeType for IActionEntityFactory3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntityFactory3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateRemoteFileEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, RemoteFileKind, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextEntityWithTextFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, ActionEntityTextFormat, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStreamingTextActionEntityWriter: unsafe extern "system" fn(*mut core::ffi::c_void, ActionEntityTextFormat, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionEntityFactory4, IActionEntityFactory4_Vtbl, 0x332eda05_de0e_5a58_b318_a2ad771f013d);
impl windows_core::RuntimeType for IActionEntityFactory4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntityFactory4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateTableEntity: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::HSTRING, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub CreateContactEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    CreateContactEntity: usize,
}
windows_core::imp::define_interface!(IActionEntityFactory5, IActionEntityFactory5_Vtbl, 0xb59faab1_cfe4_564a_a5ba_53ad7ff6f924);
impl windows_core::RuntimeType for IActionEntityFactory5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntityFactory5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateUriEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateArrayEntity: unsafe extern "system" fn(*mut core::ffi::c_void, ActionEntityKind, u32, *const ActionEntity, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionEntityFactory6, IActionEntityFactory6_Vtbl, 0xda7123da_5639_590f_a2db_c3b5e221f3b6);
impl windows_core::RuntimeType for IActionEntityFactory6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntityFactory6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDateTimeEntity: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub CreateAppointmentEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const ContactActionEntity, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    CreateAppointmentEntity: usize,
}
windows_core::imp::define_interface!(IActionEntityFactory7, IActionEntityFactory7_Vtbl, 0xb814b8d5_c9b2_51b5_a342_9fe054d8a1eb);
impl windows_core::RuntimeType for IActionEntityFactory7 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntityFactory7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateCustomTextEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateArrayEntityWithCustomKind: unsafe extern "system" fn(*mut core::ffi::c_void, ActionEntityKind, *mut core::ffi::c_void, u32, *const ActionEntity, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionEntityFactoryFactory, IActionEntityFactoryFactory_Vtbl, 0xc9147d8f_88a0_5ec0_a564_47e2a1081412);
impl windows_core::RuntimeType for IActionEntityFactoryFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionEntityFactoryFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IActionFeedback, IActionFeedback_Vtbl, 0xa12ee7ab_2454_56c9_bbdf_c089457fbc5e);
impl windows_core::RuntimeType for IActionFeedback {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionFeedback_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FeedbackKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActionFeedbackKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionInvocationContext, IActionInvocationContext_Vtbl, 0xc32b622e_86e1_5eba_9661_605910104978);
impl windows_core::RuntimeType for IActionInvocationContext {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionInvocationContext_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EntityFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInputEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInputEntities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetOutputEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputEntities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActionInvocationResult) -> windows_core::HRESULT,
    pub SetResult: unsafe extern "system" fn(*mut core::ffi::c_void, ActionInvocationResult) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub SetExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionInvocationContext2, IActionInvocationContext2_Vtbl, 0x7c843086_9279_5bcd_8f2e_d15121e7a827);
impl windows_core::RuntimeType for IActionInvocationContext2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionInvocationContext2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub InvokerWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::UI::WindowId) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    InvokerWindowId: usize,
    pub HelpDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvokerAppUserModelId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionInvocationHelpDetails, IActionInvocationHelpDetails_Vtbl, 0x5430f272_078f_5722_8f7d_90cf8ddd595e);
impl windows_core::RuntimeType for IActionInvocationHelpDetails {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionInvocationHelpDetails_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActionInvocationHelpKind) -> windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(*mut core::ffi::c_void, ActionInvocationHelpKind) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HelpUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHelpUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HelpUriDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHelpUriDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionInvocationHelpDetails2, IActionInvocationHelpDetails2_Vtbl, 0x307f6ba5_5fda_59f1_9722_1859801ad550);
impl windows_core::RuntimeType for IActionInvocationHelpDetails2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionInvocationHelpDetails2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionRuntime, IActionRuntime_Vtbl, 0x206efa2c_c909_508a_b4b0_9482be96db9c);
impl windows_core::RuntimeType for IActionRuntime {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionRuntime_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "AI_Actions_Hosting")]
    pub ActionCatalog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "AI_Actions_Hosting"))]
    ActionCatalog: usize,
    pub EntityFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInvocationContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionRuntime2, IActionRuntime2_Vtbl, 0x2da4d2c0_e593_5350_8143_15bb24f63411);
impl windows_core::RuntimeType for IActionRuntime2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionRuntime2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateActionFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, ActionFeedbackKind, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetActionAvailability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub GetActionAvailability: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionRuntime3, IActionRuntime3_Vtbl, 0xf020c3c0_caec_5928_ad00_81069b80fbc1);
impl windows_core::RuntimeType for IActionRuntime3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionRuntime3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub CreateInvocationContextWithWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::UI::WindowId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateInvocationContextWithWindowId: usize,
    pub GetActionEntityById: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LatestSupportedSchemaVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionRuntime4, IActionRuntime4_Vtbl, 0x06851dcd_c743_5c7f_88a1_bbaeb02f5e28);
impl windows_core::RuntimeType for IActionRuntime4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionRuntime4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetActionInvocationContextFromToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionRuntime5, IActionRuntime5_Vtbl, 0xc2e995b1_52a9_5f3a_bebb_a04655e96218);
impl windows_core::RuntimeType for IActionRuntime5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionRuntime5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CustomEntityStore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IActionRuntimeFactory, IActionRuntimeFactory_Vtbl, 0xd3f366e9_8dc9_50a0_8040_e5c14fa609d6);
impl windows_core::RuntimeType for IActionRuntimeFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionRuntimeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IActionRuntimeStatics, IActionRuntimeStatics_Vtbl, 0x2c697aab_55f2_55aa_9d63_a73ec190cecd);
impl windows_core::RuntimeType for IActionRuntimeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IActionRuntimeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppointmentActionEntity, IAppointmentActionEntity_Vtbl, 0x29daa00e_b474_581c_b555_6187d1aa8231);
impl windows_core::RuntimeType for IAppointmentActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub Appointment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    Appointment: usize,
    pub GetAttendees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPresentedFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPresentedFiles: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const RemoteFileActionEntity) -> windows_core::HRESULT,
    pub GetSharedFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSharedFiles: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const RemoteFileActionEntity) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IArrayActionEntity, IArrayActionEntity_Vtbl, 0x45798e78_1059_5311_8a1b_de0081a4ca3b);
impl windows_core::RuntimeType for IArrayActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IArrayActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ElementKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActionEntityKind) -> windows_core::HRESULT,
    pub GetAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IArrayActionEntity2, IArrayActionEntity2_Vtbl, 0x7366e049_7fe8_5df9_bbca_cea5c0f3d316);
impl windows_core::RuntimeType for IArrayActionEntity2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IArrayActionEntity2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CustomElementKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactActionEntity, IContactActionEntity_Vtbl, 0x458c3e07_5892_5485_bd9b_8f7a540c9501);
impl windows_core::RuntimeType for IContactActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
windows_core::imp::define_interface!(ICustomActionEntityStore, ICustomActionEntityStore_Vtbl, 0xfa7b44d0_1762_5828_9938_e7cae5199e01);
impl windows_core::RuntimeType for ICustomActionEntityStore {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomActionEntityStore_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetLastModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub Insert: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertMany: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const CustomTextActionEntity) -> windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICustomActionEntityStoreFactory, ICustomActionEntityStoreFactory_Vtbl, 0xd8b46bdb_68a5_5e07_9113_abb9241aaab1);
impl windows_core::RuntimeType for ICustomActionEntityStoreFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomActionEntityStoreFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICustomTextActionEntity, ICustomTextActionEntity_Vtbl, 0x50eaac95_1d6c_54b0_8963_e38dea3f6aec);
impl windows_core::RuntimeType for ICustomTextActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomTextActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CustomTextKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeyPhrase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDateTimeActionEntity, IDateTimeActionEntity_Vtbl, 0xfd5a0880_eeae_553a_bfed_a9229d57447d);
impl windows_core::RuntimeType for IDateTimeActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDateTimeActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDocumentActionEntity, IDocumentActionEntity_Vtbl, 0x56715297_960b_59ff_af4b_ece1098b2e36);
impl windows_core::RuntimeType for IDocumentActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDocumentActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FullPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileActionEntity, IFileActionEntity_Vtbl, 0xf20ab43f_4c80_5904_bd42_3e6248babfcf);
impl windows_core::RuntimeType for IFileActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FullPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INamedActionEntity, INamedActionEntity_Vtbl, 0x1aaebeef_435b_5a0d_8182_05fe4dd47712);
impl windows_core::RuntimeType for INamedActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Entity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPhotoActionEntity, IPhotoActionEntity_Vtbl, 0x425123b3_20ef_51a6_b35f_8414384765c5);
impl windows_core::RuntimeType for IPhotoActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FullPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteFileActionEntity, IRemoteFileActionEntity_Vtbl, 0xa5d8ec21_a2bd_545a_abfc_d7aa79fd0b81);
impl windows_core::RuntimeType for IRemoteFileActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteFileActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RemoteFileKind) -> windows_core::HRESULT,
    pub SourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DriveId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Extension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRemoteFileActionEntity2, IRemoteFileActionEntity2_Vtbl, 0x9cc8cc54_77d8_5537_83c4_6f18c1bc9f67);
impl windows_core::RuntimeType for IRemoteFileActionEntity2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteFileActionEntity2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Filename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFilename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Creator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCreator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastUpdatedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLastUpdatedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContributors: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const ContactActionEntity) -> windows_core::HRESULT,
    pub GetContributors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamingTextActionEntity, IStreamingTextActionEntity_Vtbl, 0x44cd8a16_abc9_5703_b4bf_6fe8b7a802fd);
impl windows_core::RuntimeType for IStreamingTextActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamingTextActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TextFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActionEntityTextFormat) -> windows_core::HRESULT,
    pub TextChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTextChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamingTextActionEntityTextChangedArgs, IStreamingTextActionEntityTextChangedArgs_Vtbl, 0x2c62011f_3e06_588b_a3bd_d726bd82fb13);
impl windows_core::RuntimeType for IStreamingTextActionEntityTextChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamingTextActionEntityTextChangedArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStreamingTextActionEntityWriter, IStreamingTextActionEntityWriter_Vtbl, 0x6bce2f76_a8af_5ff2_833c_108737ba0f42);
impl windows_core::RuntimeType for IStreamingTextActionEntityWriter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStreamingTextActionEntityWriter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReaderEntity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TextFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActionEntityTextFormat) -> windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITableActionEntity, ITableActionEntity_Vtbl, 0x0f252cdb_ba24_5dbb_9d17_1b300773d141);
impl windows_core::RuntimeType for ITableActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetTextContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextActionEntity, ITextActionEntity_Vtbl, 0x3c4ec25f_5adb_5f73_b8f3_080fbeadd612);
impl windows_core::RuntimeType for ITextActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextActionEntity2, ITextActionEntity2_Vtbl, 0x7c500889_cf08_51e7_beca_f0bbc7a7486c);
impl windows_core::RuntimeType for ITextActionEntity2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextActionEntity2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TextFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActionEntityTextFormat) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriActionEntity, IUriActionEntity_Vtbl, 0xa81cde77_bc25_532d_905e_b0725c5bcd4e);
impl windows_core::RuntimeType for IUriActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriActionEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamedActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NamedActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(NamedActionEntity, super::super::Foundation::IClosable);
impl NamedActionEntity {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Entity(&self) -> windows_core::Result<ActionEntity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Entity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetEntity<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ActionEntity>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetEntity)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for NamedActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INamedActionEntity>();
}
unsafe impl windows_core::Interface for NamedActionEntity {
    type Vtable = <INamedActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INamedActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NamedActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.NamedActionEntity";
}
unsafe impl Send for NamedActionEntity {}
unsafe impl Sync for NamedActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhotoActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PhotoActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PhotoActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl PhotoActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FullPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FullPath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for PhotoActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPhotoActionEntity>();
}
unsafe impl windows_core::Interface for PhotoActionEntity {
    type Vtable = <IPhotoActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPhotoActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PhotoActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.PhotoActionEntity";
}
unsafe impl Send for PhotoActionEntity {}
unsafe impl Sync for PhotoActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteFileActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RemoteFileActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RemoteFileActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl RemoteFileActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SourceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FileKind(&self) -> windows_core::Result<RemoteFileKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SourceUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DriveId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DriveId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn AccountId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccountId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Extension(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Extension)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Filename(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IRemoteFileActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Filename)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetFilename(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRemoteFileActionEntity2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFilename)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Creator(&self) -> windows_core::Result<ContactActionEntity> {
        let this = &windows_core::Interface::cast::<IRemoteFileActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Creator)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCreator<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ContactActionEntity>,
    {
        let this = &windows_core::Interface::cast::<IRemoteFileActionEntity2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCreator)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn LastUpdatedTime(&self) -> windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &windows_core::Interface::cast::<IRemoteFileActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastUpdatedTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetLastUpdatedTime<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = &windows_core::Interface::cast::<IRemoteFileActionEntity2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLastUpdatedTime)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SetContributors(&self, contributors: &[Option<ContactActionEntity>]) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRemoteFileActionEntity2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetContributors)(windows_core::Interface::as_raw(this), contributors.len().try_into().unwrap(), core::mem::transmute(contributors.as_ptr())).ok() }
    }
    pub fn GetContributors(&self) -> windows_core::Result<windows_core::Array<ContactActionEntity>> {
        let this = &windows_core::Interface::cast::<IRemoteFileActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetContributors)(windows_core::Interface::as_raw(this), windows_core::Array::<ContactActionEntity>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeType for RemoteFileActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRemoteFileActionEntity>();
}
unsafe impl windows_core::Interface for RemoteFileActionEntity {
    type Vtable = <IRemoteFileActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRemoteFileActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RemoteFileActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.RemoteFileActionEntity";
}
unsafe impl Send for RemoteFileActionEntity {}
unsafe impl Sync for RemoteFileActionEntity {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RemoteFileKind(pub i32);
impl RemoteFileKind {
    pub const Document: Self = Self(0i32);
    pub const Photo: Self = Self(1i32);
    pub const File: Self = Self(2i32);
}
impl windows_core::TypeKind for RemoteFileKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for RemoteFileKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.AI.Actions.RemoteFileKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamingTextActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamingTextActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StreamingTextActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl StreamingTextActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsComplete(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsComplete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetText)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TextFormat(&self) -> windows_core::Result<ActionEntityTextFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TextChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::Foundation::TypedEventHandler<StreamingTextActionEntity, StreamingTextActionEntityTextChangedArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveTextChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveTextChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
}
impl windows_core::RuntimeType for StreamingTextActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamingTextActionEntity>();
}
unsafe impl windows_core::Interface for StreamingTextActionEntity {
    type Vtable = <IStreamingTextActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStreamingTextActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamingTextActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.StreamingTextActionEntity";
}
unsafe impl Send for StreamingTextActionEntity {}
unsafe impl Sync for StreamingTextActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamingTextActionEntityTextChangedArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamingTextActionEntityTextChangedArgs, windows_core::IUnknown, windows_core::IInspectable);
impl StreamingTextActionEntityTextChangedArgs {
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsComplete(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsComplete)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for StreamingTextActionEntityTextChangedArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamingTextActionEntityTextChangedArgs>();
}
unsafe impl windows_core::Interface for StreamingTextActionEntityTextChangedArgs {
    type Vtable = <IStreamingTextActionEntityTextChangedArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStreamingTextActionEntityTextChangedArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamingTextActionEntityTextChangedArgs {
    const NAME: &'static str = "Windows.AI.Actions.StreamingTextActionEntityTextChangedArgs";
}
unsafe impl Send for StreamingTextActionEntityTextChangedArgs {}
unsafe impl Sync for StreamingTextActionEntityTextChangedArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamingTextActionEntityWriter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StreamingTextActionEntityWriter, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StreamingTextActionEntityWriter, super::super::Foundation::IClosable);
impl StreamingTextActionEntityWriter {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReaderEntity(&self) -> windows_core::Result<StreamingTextActionEntity> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReaderEntity)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TextFormat(&self) -> windows_core::Result<ActionEntityTextFormat> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetText(&self, text: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text)).ok() }
    }
}
impl windows_core::RuntimeType for StreamingTextActionEntityWriter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStreamingTextActionEntityWriter>();
}
unsafe impl windows_core::Interface for StreamingTextActionEntityWriter {
    type Vtable = <IStreamingTextActionEntityWriter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStreamingTextActionEntityWriter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StreamingTextActionEntityWriter {
    const NAME: &'static str = "Windows.AI.Actions.StreamingTextActionEntityWriter";
}
unsafe impl Send for StreamingTextActionEntityWriter {}
unsafe impl Sync for StreamingTextActionEntityWriter {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TableActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TableActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl TableActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetTextContent(&self) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetTextContent)(windows_core::Interface::as_raw(this), windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn RowCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RowCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ColumnCount(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ColumnCount)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TableActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITableActionEntity>();
}
unsafe impl windows_core::Interface for TableActionEntity {
    type Vtable = <ITableActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITableActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TableActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.TableActionEntity";
}
unsafe impl Send for TableActionEntity {}
unsafe impl Sync for TableActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TextActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl TextActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TextFormat(&self) -> windows_core::Result<ActionEntityTextFormat> {
        let this = &windows_core::Interface::cast::<ITextActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextFormat)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TextActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextActionEntity>();
}
unsafe impl windows_core::Interface for TextActionEntity {
    type Vtable = <ITextActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.TextActionEntity";
}
unsafe impl Send for TextActionEntity {}
unsafe impl Sync for TextActionEntity {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UriActionEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UriActionEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(UriActionEntity, super::super::Foundation::IClosable, ActionEntity);
impl UriActionEntity {
    pub fn Kind(&self) -> windows_core::Result<ActionEntityKind> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayInfo(&self) -> windows_core::Result<ActionEntityDisplayInfo> {
        let this = &windows_core::Interface::cast::<IActionEntity>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayInfo)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IActionEntity2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for UriActionEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUriActionEntity>();
}
unsafe impl windows_core::Interface for UriActionEntity {
    type Vtable = <IUriActionEntity as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUriActionEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UriActionEntity {
    const NAME: &'static str = "Windows.AI.Actions.UriActionEntity";
}
unsafe impl Send for UriActionEntity {}
unsafe impl Sync for UriActionEntity {}
