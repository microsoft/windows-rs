use super::*;
use Constructors::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    match name {
        "Constructors.Widget" => Some(WidgetFactory.into()),
        "Constructors.Base" => Some(BaseFactory.into()),
        _ => None,
    }
}

#[implement(Constructors::Widget)]
struct Widget(i32);

impl IWidget_Impl for Widget_Impl {
    fn Value(&self) -> Result<i32> {
        Ok(self.0)
    }
}

/// The default constructor uses `IActivationFactory` and the others use the factory interface.
#[implement(IActivationFactory, IWidgetFactory)]
struct WidgetFactory;

impl IActivationFactory_Impl for WidgetFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        let widget: Constructors::Widget = Widget(0).into();
        Ok(widget.into())
    }
}

impl IWidgetFactory_Impl for WidgetFactory_Impl {
    fn WithValue(&self, value: i32) -> Result<Constructors::Widget> {
        Ok(Widget(value).into())
    }
}

#[implement(Constructors::Base)]
struct Base(i32);

impl IBase_Impl for Base_Impl {
    fn Value(&self) -> Result<i32> {
        Ok(self.0)
    }
}

/// All of a composable class's constructors use the factory interface.
#[implement(IActivationFactory, IBaseFactory)]
struct BaseFactory;

impl IActivationFactory_Impl for BaseFactory_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Err(E_NOTIMPL.into())
    }
}

impl BaseFactory_Impl {
    /// The component does not support derived classes, so callers must not pass an outer object.
    /// Like C++/WinRT, a non-aggregated instance is its own inner object, which the caller
    /// releases.
    fn create(
        &self,
        value: i32,
        outer: Ref<IInspectable>,
        inner: OutRef<IInspectable>,
    ) -> Result<Constructors::Base> {
        if outer.is_some() {
            inner.write(None)?;
            return Err(CLASS_E_NOAGGREGATION.into());
        }

        let base: Constructors::Base = Base(value).into();
        inner.write(Some(base.clone().into()))?;
        Ok(base)
    }
}

impl IBaseFactory_Impl for BaseFactory_Impl {
    fn CreateInstance(
        &self,
        outer: Ref<IInspectable>,
        inner: OutRef<IInspectable>,
    ) -> Result<Constructors::Base> {
        self.create(0, outer, inner)
    }

    fn WithValue(
        &self,
        value: i32,
        outer: Ref<IInspectable>,
        inner: OutRef<IInspectable>,
    ) -> Result<Constructors::Base> {
        self.create(value, outer, inner)
    }
}
