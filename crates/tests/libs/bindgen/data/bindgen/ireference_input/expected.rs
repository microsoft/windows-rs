#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MapRouteDrivingOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MapRouteDrivingOptions,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl MapRouteDrivingOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            MapRouteDrivingOptions,
            windows_core::imp::IGenericFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MaxAlternateRouteCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxAlternateRouteCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetMaxAlternateRouteCount(&self, value: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxAlternateRouteCount)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn InitialHeading(&self) -> windows_core::Result<windows_reference::IReference<f64>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitialHeading)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInitialHeading(&self, value: Option<f64>) -> windows_core::Result<()> {
        let value__ =
            value.map(<windows_reference::IReference<f64> as core::convert::From<_>>::from);
        unsafe {
            (windows_core::Interface::vtable(self).SetInitialHeading)(
                windows_core::Interface::as_raw(self),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
    pub fn RouteOptimization(
        &self,
    ) -> windows_core::Result<windows::Services::Maps::MapRouteOptimization> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RouteOptimization)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRouteOptimization(
        &self,
        value: windows::Services::Maps::MapRouteOptimization,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRouteOptimization)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn RouteRestrictions(
        &self,
    ) -> windows_core::Result<windows::Services::Maps::MapRouteRestrictions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RouteRestrictions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetRouteRestrictions(
        &self,
        value: windows::Services::Maps::MapRouteRestrictions,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRouteRestrictions)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
    pub fn DepartureTime(
        &self,
    ) -> windows_core::Result<windows_reference::IReference<windows::Foundation::DateTime>> {
        let this = &windows_core::Interface::cast::<
            windows::Services::Maps::IMapRouteDrivingOptions2,
        >(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DepartureTime)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDepartureTime(
        &self,
        value: Option<windows::Foundation::DateTime>,
    ) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<
            windows::Services::Maps::IMapRouteDrivingOptions2,
        >(self)?;
        let value__ = value.map(
            <windows_reference::IReference<windows::Foundation::DateTime> as core::convert::From<
                _,
            >>::from,
        );
        unsafe {
            (windows_core::Interface::vtable(this).SetDepartureTime)(
                windows_core::Interface::as_raw(this),
                windows_core::Param::param(value__.as_ref()).abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeType for MapRouteDrivingOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
        Self,
        windows::Services::Maps::IMapRouteDrivingOptions,
    >();
}
unsafe impl windows_core::Interface for MapRouteDrivingOptions {
    type Vtable =
        <windows::Services::Maps::IMapRouteDrivingOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <windows::Services::Maps::IMapRouteDrivingOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MapRouteDrivingOptions {
    const NAME: &'static str = "Windows.Services.Maps.MapRouteDrivingOptions";
}
unsafe impl Send for MapRouteDrivingOptions {}
unsafe impl Sync for MapRouteDrivingOptions {}
