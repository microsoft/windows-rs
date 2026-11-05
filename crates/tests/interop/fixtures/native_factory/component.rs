use super::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "NativeFactory.Widget").then(activatable::<Widget, NativeFactory::Widget>)
}

#[implement(NativeFactory::Widget, NativeFactory::IWidgetInterop)]
#[derive(Default)]
struct Widget {
    handle: *mut core::ffi::c_void,
}

impl NativeFactory::IWidget_Impl for Widget_Impl {
    fn HandleValue(&self) -> Result<u64> {
        Ok(self.handle as usize as u64)
    }
}

impl NativeFactory::IWidgetInterop_Impl for Widget_Impl {
    fn Handle(&self) -> *mut core::ffi::c_void {
        self.handle
    }
}

#[unsafe(no_mangle)]
extern "system" fn CreateWidgetFromHandle(
    handle: *mut core::ffi::c_void,
    widget: OutRef<NativeFactory::Widget>,
) -> HRESULT {
    widget.write(Some(Widget { handle }.into())).into()
}
