use super::*;
use bindings::Application;
use bindings::*;
use std::sync::OnceLock;
use windows_core::*;

implement_decl! {
    #[compose]
    impl ReactorApplicationOverrides as pub ReactorApplicationOverrides_Impl:
        [IApplicationOverrides, IXamlMetadataProvider]
}

pub struct ReactorApplicationOverrides {
    controls_provider: OnceLock<XamlControlsXamlMetaDataProvider>,
    on_launched: local::Local<RefCell<Option<Box<dyn FnOnce() -> Result<()>>>>>,
}

impl ReactorApplicationOverrides {
    fn new(on_launched: Box<dyn FnOnce() -> Result<()>>) -> Self {
        Self {
            controls_provider: OnceLock::new(),
            on_launched: local::Local::new(RefCell::new(Some(on_launched))),
        }
    }

    fn provider(&self) -> Result<XamlControlsXamlMetaDataProvider> {
        if let Some(provider) = self.controls_provider.get() {
            return Ok(provider.clone());
        }
        let provider = XamlControlsXamlMetaDataProvider::new()?;
        Ok(self.controls_provider.get_or_init(|| provider).clone())
    }
}

impl IApplicationOverrides_Impl for ReactorApplicationOverrides_Impl {
    fn OnLaunched(&self, _args: Ref<LaunchActivatedEventArgs>) -> Result<()> {
        self.on_launched.try_with(|on_launched| {
            let on_launched = on_launched.borrow_mut().take();
            if let Some(on_launched) = on_launched {
                on_launched()?;
            }
            Ok(())
        })?
    }
}

impl IXamlMetadataProvider_Impl for ReactorApplicationOverrides_Impl {
    fn GetXamlType(&self, r#type: &TypeName) -> Result<IXamlType> {
        self.provider()?.GetXamlType(r#type)
    }

    fn GetXamlTypeByFullName(&self, full_name: &HSTRING) -> Result<IXamlType> {
        self.provider()?
            .GetXamlTypeByFullName(&full_name.to_string_lossy())
    }

    fn GetXmlnsDefinitions(&self) -> Result<Array<XmlnsDefinition>> {
        self.provider()?.GetXmlnsDefinitions()
    }
}

pub(super) fn create_application(
    on_launched: Box<dyn FnOnce() -> Result<()>>,
) -> Result<Application> {
    Application::compose(ReactorApplicationOverrides::new(on_launched))
}

pub(super) fn install_xaml_controls_resources(application: &Application) -> Result<()> {
    let controls = XamlControlsResources::new()?;
    let resources: ResourceDictionary = controls.cast()?;
    application
        .Resources()?
        .MergedDictionaries()?
        .Append(&resources)
}
