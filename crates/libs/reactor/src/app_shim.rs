use std::cell::RefCell;
use windows_core::*;

use super::bindings::*;

implement_decl! {
    impl ReactorApplicationOverrides as pub ReactorApplicationOverrides_Impl: [IApplicationOverrides, IXamlMetadataProvider]
}

pub struct ReactorApplicationOverrides {
    controls_provider: RefCell<Option<XamlControlsXamlMetaDataProvider>>,
    on_launched: RefCell<Option<Box<dyn FnOnce() -> Result<()>>>>,
}

impl ReactorApplicationOverrides {
    fn new(on_launched: Box<dyn FnOnce() -> Result<()>>) -> Self {
        Self {
            controls_provider: RefCell::new(None),
            on_launched: RefCell::new(Some(on_launched)),
        }
    }

    fn provider(&self) -> Result<XamlControlsXamlMetaDataProvider> {
        if let Some(p) = self.controls_provider.borrow().as_ref() {
            return Ok(p.clone());
        }
        let p = XamlControlsXamlMetaDataProvider::new()?;
        *self.controls_provider.borrow_mut() = Some(p.clone());
        Ok(p)
    }
}

impl IApplicationOverrides_Impl for ReactorApplicationOverrides_Impl {
    fn OnLaunched(&self, _args: Ref<LaunchActivatedEventArgs>) -> Result<()> {
        if let Some(cb) = self.on_launched.borrow_mut().take() {
            cb()?;
        }
        Ok(())
    }
}

impl IXamlMetadataProvider_Impl for ReactorApplicationOverrides_Impl {
    fn GetXamlType(&self, r#type: &TypeName) -> Result<IXamlType> {
        self.provider()?.GetXamlType(r#type)
    }

    fn GetXamlTypeByFullName(&self, full_name: &HSTRING) -> Result<IXamlType> {
        let full_name = full_name.to_string_lossy();
        self.provider()?.GetXamlTypeByFullName(&full_name)
    }

    fn GetXmlnsDefinitions(&self) -> Result<Array<XmlnsDefinition>> {
        self.provider()?.GetXmlnsDefinitions()
    }
}

pub fn create_reactor_application(
    on_launched: Box<dyn FnOnce() -> Result<()>>,
) -> Result<Application> {
    Application::compose(ReactorApplicationOverrides::new(on_launched))
}

pub fn install_xaml_controls_resources(app: &Application) -> Result<()> {
    let controls = XamlControlsResources::new()?;
    let as_rd: ResourceDictionary = controls.cast()?;
    let resources = app.Resources()?;
    let merged = resources.MergedDictionaries()?;
    merged.Append(&as_rd)?;
    Ok(())
}
