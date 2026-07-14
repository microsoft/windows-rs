windows_core::imp::define_interface!(IPowerThermalChannelConfiguration, IPowerThermalChannelConfiguration_Vtbl, 0xad8383fa_172d_5d82_b29d_81d93710fb45);
impl windows_core::RuntimeType for IPowerThermalChannelConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.IPowerThermalChannelConfiguration");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerThermalChannelConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PowerThermalChannelId) -> windows_core::HRESULT,
    pub ConfigurationString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConfigurationNumericParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPowerThermalChannelDataConsumer, IPowerThermalChannelDataConsumer_Vtbl, 0x47cca211_7348_5026_898c_b1873123760d);
impl windows_core::RuntimeType for IPowerThermalChannelDataConsumer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.IPowerThermalChannelDataConsumer");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerThermalChannelDataConsumer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetChannelIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut PowerThermalChannelId) -> windows_core::HRESULT,
    pub GetChannelConfigurations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChannelDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveChannelDataReceived: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub BackEndStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PowerThermalBackEndStatus) -> windows_core::HRESULT,
    pub BackEndStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveBackEndStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPowerThermalChannelDataConsumerFactory, IPowerThermalChannelDataConsumerFactory_Vtbl, 0xb42d9ab1_32f0_54bb_899a_9ae0529da381);
impl windows_core::RuntimeType for IPowerThermalChannelDataConsumerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.IPowerThermalChannelDataConsumerFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerThermalChannelDataConsumerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PowerThermalChannelId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPowerThermalChannelDataProducer, IPowerThermalChannelDataProducer_Vtbl, 0xa935f244_1a7d_55d5_9c69_8adc1cd1d993);
impl windows_core::RuntimeType for IPowerThermalChannelDataProducer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.IPowerThermalChannelDataProducer");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerThermalChannelDataProducer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetChannelIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut PowerThermalChannelId) -> windows_core::HRESULT,
    pub GetChannelConfigurations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableChannel: unsafe extern "system" fn(*mut core::ffi::c_void, PowerThermalChannelId) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PublishInputChannelData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PowerThermalChannelData) -> windows_core::HRESULT,
    pub BackEndStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PowerThermalBackEndStatus) -> windows_core::HRESULT,
    pub BackEndStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveBackEndStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPowerThermalChannelDataProducerFactory, IPowerThermalChannelDataProducerFactory_Vtbl, 0xd2d380cd_e09d_5472_ad62_70061e630067);
impl windows_core::RuntimeType for IPowerThermalChannelDataProducerFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.IPowerThermalChannelDataProducerFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerThermalChannelDataProducerFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PowerThermalChannelId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPowerThermalChannelDataReceivedEventArgs, IPowerThermalChannelDataReceivedEventArgs_Vtbl, 0xd6b643e0_6ab6_5683_a8fc_5ed65ee20dc5);
impl windows_core::RuntimeType for IPowerThermalChannelDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.IPowerThermalChannelDataReceivedEventArgs");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerThermalChannelDataReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut PowerThermalChannelData) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPowerThermalChannelDiagnostics, IPowerThermalChannelDiagnostics_Vtbl, 0x628fd5c4_49b7_508f_ad9f_2309b1168aad);
impl windows_core::RuntimeType for IPowerThermalChannelDiagnostics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.IPowerThermalChannelDiagnostics");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerThermalChannelDiagnostics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPowerThermalChannelDiagnosticsStatics, IPowerThermalChannelDiagnosticsStatics_Vtbl, 0xa86ec393_b684_5633_a6ca_dfa1c7eecf87);
impl windows_core::RuntimeType for IPowerThermalChannelDiagnosticsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.IPowerThermalChannelDiagnosticsStatics");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerThermalChannelDiagnosticsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDataForChannels: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const PowerThermalChannelId, *mut u32, *mut *mut PowerThermalChannelData) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPowerThermalChannelFinderStatics, IPowerThermalChannelFinderStatics_Vtbl, 0xdf8d288b_f056_55ce_b370_f3e1c4e32063);
impl windows_core::RuntimeType for IPowerThermalChannelFinderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.IPowerThermalChannelFinderStatics");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPowerThermalChannelFinderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FindChannels: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut u32, *mut *mut PowerThermalChannelId) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PowerThermalBackEndStatus(pub i32);
impl PowerThermalBackEndStatus {
    pub const Stopped: Self = Self(0);
    pub const Started: Self = Self(1);
}
impl windows_core::TypeKind for PowerThermalBackEndStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PowerThermalBackEndStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.Power.Thermal.PowerThermalBackEndStatus;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.PowerThermalBackEndStatus");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PowerThermalChannelConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PowerThermalChannelConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl PowerThermalChannelConfiguration {
    pub fn Id(&self) -> windows_core::Result<PowerThermalChannelId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ConfigurationString(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConfigurationString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetConfigurationNumericParameters(&self) -> windows_core::Result<windows_core::Array<i32>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetConfigurationNumericParameters)(windows_core::Interface::as_raw(self), windows_core::Array::<i32>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeType for PowerThermalChannelConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPowerThermalChannelConfiguration>();
}
unsafe impl windows_core::Interface for PowerThermalChannelConfiguration {
    type Vtable = <IPowerThermalChannelConfiguration as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPowerThermalChannelConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PowerThermalChannelConfiguration {
    const NAME: &'static str = "Windows.System.Power.Thermal.PowerThermalChannelConfiguration";
}
unsafe impl Send for PowerThermalChannelConfiguration {}
unsafe impl Sync for PowerThermalChannelConfiguration {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PowerThermalChannelData {
    pub Id: PowerThermalChannelId,
    pub Value: i32,
}
impl windows_core::TypeKind for PowerThermalChannelData {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PowerThermalChannelData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.System.Power.Thermal.PowerThermalChannelData;struct(Windows.System.Power.Thermal.PowerThermalChannelId;g16;u2);i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.PowerThermalChannelData");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PowerThermalChannelDataConsumer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PowerThermalChannelDataConsumer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PowerThermalChannelDataConsumer, super::super::super::Foundation::IClosable);
impl PowerThermalChannelDataConsumer {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetChannelIds(&self) -> windows_core::Result<windows_core::Array<PowerThermalChannelId>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetChannelIds)(windows_core::Interface::as_raw(self), windows_core::Array::<PowerThermalChannelId>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetChannelConfigurations(&self) -> windows_core::Result<windows_collections::IMapView<PowerThermalChannelId, PowerThermalChannelConfiguration>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelConfigurations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn ChannelDataReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PowerThermalChannelDataReceivedEventArgs>) + Send + 'static,
    {
        let handler = <super::super::super::Foundation::TypedEventHandler<Self, PowerThermalChannelDataReceivedEventArgs>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ChannelDataReceived)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveChannelDataReceived))
        }
    }
    pub fn BackEndStatus(&self) -> windows_core::Result<PowerThermalBackEndStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackEndStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn BackEndStatusChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) + Send + 'static,
    {
        let handler = <super::super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).BackEndStatusChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveBackEndStatusChanged))
        }
    }
    pub fn CreateInstance(channelids: &[PowerThermalChannelId]) -> windows_core::Result<Self> {
        Self::IPowerThermalChannelDataConsumerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), channelids.len().try_into().unwrap(), channelids.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPowerThermalChannelDataConsumerFactory<R, F: FnOnce(&IPowerThermalChannelDataConsumerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PowerThermalChannelDataConsumer, IPowerThermalChannelDataConsumerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PowerThermalChannelDataConsumer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPowerThermalChannelDataConsumer>();
}
unsafe impl windows_core::Interface for PowerThermalChannelDataConsumer {
    type Vtable = <IPowerThermalChannelDataConsumer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPowerThermalChannelDataConsumer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PowerThermalChannelDataConsumer {
    const NAME: &'static str = "Windows.System.Power.Thermal.PowerThermalChannelDataConsumer";
}
unsafe impl Send for PowerThermalChannelDataConsumer {}
unsafe impl Sync for PowerThermalChannelDataConsumer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PowerThermalChannelDataProducer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PowerThermalChannelDataProducer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PowerThermalChannelDataProducer, super::super::super::Foundation::IClosable);
impl PowerThermalChannelDataProducer {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetChannelIds(&self) -> windows_core::Result<windows_core::Array<PowerThermalChannelId>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetChannelIds)(windows_core::Interface::as_raw(self), windows_core::Array::<PowerThermalChannelId>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetChannelConfigurations(&self) -> windows_core::Result<windows_collections::IMapView<PowerThermalChannelId, PowerThermalChannelConfiguration>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelConfigurations)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisableChannel(&self, channelid: PowerThermalChannelId) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DisableChannel)(windows_core::Interface::as_raw(self), channelid).ok() }
    }
    pub fn Start(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn PublishInputChannelData(&self, data: &[PowerThermalChannelData]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).PublishInputChannelData)(windows_core::Interface::as_raw(self), data.len().try_into().unwrap(), data.as_ptr()).ok() }
    }
    pub fn BackEndStatus(&self) -> windows_core::Result<PowerThermalBackEndStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackEndStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn BackEndStatusChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) + Send + 'static,
    {
        let handler = <super::super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).BackEndStatusChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveBackEndStatusChanged))
        }
    }
    pub fn CreateInstance(channelids: &[PowerThermalChannelId]) -> windows_core::Result<Self> {
        Self::IPowerThermalChannelDataProducerFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), channelids.len().try_into().unwrap(), channelids.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPowerThermalChannelDataProducerFactory<R, F: FnOnce(&IPowerThermalChannelDataProducerFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PowerThermalChannelDataProducer, IPowerThermalChannelDataProducerFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PowerThermalChannelDataProducer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPowerThermalChannelDataProducer>();
}
unsafe impl windows_core::Interface for PowerThermalChannelDataProducer {
    type Vtable = <IPowerThermalChannelDataProducer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPowerThermalChannelDataProducer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PowerThermalChannelDataProducer {
    const NAME: &'static str = "Windows.System.Power.Thermal.PowerThermalChannelDataProducer";
}
unsafe impl Send for PowerThermalChannelDataProducer {}
unsafe impl Sync for PowerThermalChannelDataProducer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PowerThermalChannelDataReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PowerThermalChannelDataReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PowerThermalChannelDataReceivedEventArgs {
    pub fn GetData(&self) -> windows_core::Result<windows_core::Array<PowerThermalChannelData>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), windows_core::Array::<PowerThermalChannelData>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeType for PowerThermalChannelDataReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPowerThermalChannelDataReceivedEventArgs>();
}
unsafe impl windows_core::Interface for PowerThermalChannelDataReceivedEventArgs {
    type Vtable = <IPowerThermalChannelDataReceivedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPowerThermalChannelDataReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PowerThermalChannelDataReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Power.Thermal.PowerThermalChannelDataReceivedEventArgs";
}
unsafe impl Send for PowerThermalChannelDataReceivedEventArgs {}
unsafe impl Sync for PowerThermalChannelDataReceivedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PowerThermalChannelDiagnostics(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PowerThermalChannelDiagnostics, windows_core::IUnknown, windows_core::IInspectable);
impl PowerThermalChannelDiagnostics {
    pub fn Current() -> windows_core::Result<Self> {
        Self::IPowerThermalChannelDiagnosticsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDataForChannels(channelids: &[PowerThermalChannelId]) -> windows_core::Result<windows_core::Array<PowerThermalChannelData>> {
        Self::IPowerThermalChannelDiagnosticsStatics(|this| unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetDataForChannels)(windows_core::Interface::as_raw(this), channelids.len().try_into().unwrap(), channelids.as_ptr(), windows_core::Array::<PowerThermalChannelData>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        })
    }
    fn IPowerThermalChannelDiagnosticsStatics<R, F: FnOnce(&IPowerThermalChannelDiagnosticsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PowerThermalChannelDiagnostics, IPowerThermalChannelDiagnosticsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PowerThermalChannelDiagnostics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPowerThermalChannelDiagnostics>();
}
unsafe impl windows_core::Interface for PowerThermalChannelDiagnostics {
    type Vtable = <IPowerThermalChannelDiagnostics as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPowerThermalChannelDiagnostics as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PowerThermalChannelDiagnostics {
    const NAME: &'static str = "Windows.System.Power.Thermal.PowerThermalChannelDiagnostics";
}
unsafe impl Send for PowerThermalChannelDiagnostics {}
unsafe impl Sync for PowerThermalChannelDiagnostics {}
pub struct PowerThermalChannelFinder;
impl PowerThermalChannelFinder {
    pub fn FindChannels(channelinterfacetype: windows_core::GUID) -> windows_core::Result<windows_core::Array<PowerThermalChannelId>> {
        Self::IPowerThermalChannelFinderStatics(|this| unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).FindChannels)(windows_core::Interface::as_raw(this), channelinterfacetype, windows_core::Array::<PowerThermalChannelId>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        })
    }
    fn IPowerThermalChannelFinderStatics<R, F: FnOnce(&IPowerThermalChannelFinderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PowerThermalChannelFinder, IPowerThermalChannelFinderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PowerThermalChannelFinder {
    const NAME: &'static str = "Windows.System.Power.Thermal.PowerThermalChannelFinder";
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PowerThermalChannelId {
    pub InterfaceType: windows_core::GUID,
    pub InstanceId: u16,
}
impl windows_core::TypeKind for PowerThermalChannelId {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PowerThermalChannelId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.System.Power.Thermal.PowerThermalChannelId;g16;u2)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.System.Power.Thermal.PowerThermalChannelId");
}
