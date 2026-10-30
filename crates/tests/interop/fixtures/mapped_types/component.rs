use super::*;
use MappedTypes::*;
use bindings::Windows::Foundation::{
    IClosable, IClosable_Impl, IStringable, IStringable_Impl, Uri,
};
use std::sync::atomic::{AtomicI32, Ordering};
use windows_time::{DateTime, TimeSpan};

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "MappedTypes.Widget").then(|| WidgetFactory.into())
}

static CLOSED: AtomicI32 = AtomicI32::new(0);

#[implement(MappedTypes::Widget, IClosable, IStringable)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn Ticks(&self, time: &DateTime) -> Result<i64> {
        Ok(time.ticks())
    }

    fn Duration(&self, span: &TimeSpan) -> Result<i64> {
        Ok(span.ticks())
    }

    fn Add(&self, time: &DateTime, span: &TimeSpan) -> Result<DateTime> {
        time.checked_add(*span).ok_or_else(|| E_INVALIDARG.into())
    }

    fn Host(&self, uri: Ref<Uri>) -> Result<HSTRING> {
        uri.ok()?.Host()
    }

    fn CreateUri(&self, value: &HSTRING) -> Result<Uri> {
        Uri::CreateUri(value)
    }
}

impl IClosable_Impl for Widget_Impl {
    fn Close(&self) -> Result<()> {
        CLOSED.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl IStringable_Impl for Widget_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(h!("widget").clone())
    }
}

#[implement(IActivationFactory, IWidgetStatics)]
struct WidgetFactory;

impl IActivationFactory_Impl for WidgetFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        let widget: MappedTypes::Widget = Widget.into();
        Ok(widget.into())
    }
}

impl IWidgetStatics_Impl for WidgetFactory_Impl {
    fn Closed(&self) -> Result<i32> {
        Ok(CLOSED.load(Ordering::Relaxed))
    }
}
