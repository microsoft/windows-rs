use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct NativeStyleState {
    base: Option<ButtonStyle>,
    brushes: BTreeMap<PropertyId, ThemeBrush>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) struct NativeStyleKey {
    kind: ObjectType,
    base: Option<ButtonStyle>,
    brushes: Vec<(PropertyId, ThemeBrush)>,
}

impl WinUiAdapter {
    pub(super) fn set_resource_overrides(
        &mut self,
        object: ObjectId,
        resources: &ResourceOverrides,
    ) -> Result<(), WinUiError> {
        let dictionary = self
            .ui_element(object)?
            .cast::<native::IFrameworkElement>()?
            .Resources()?;
        let map = dictionary.cast::<IMap<IInspectable, IInspectable>>()?;
        let desired = resources
            .values()
            .map(|(key, _)| key.to_string())
            .collect::<HashSet<_>>();
        if let Some(previous) = self.resource_override_keys.get(&object) {
            for key in previous.difference(&desired) {
                let key = windows_reference::IReference::from(key.as_str());
                if map.HasKey(&key)? {
                    map.Remove(&key)?;
                }
            }
        }
        for (key, value) in resources.values() {
            let key = windows_reference::IReference::from(key);
            let value: IInspectable = match value {
                ResourceValue::Color(value) => solid_color_brush(*value)?.into(),
                ResourceValue::Thickness(value) => {
                    windows_reference::IReference::from(native::Thickness {
                        left: value.left,
                        top: value.top,
                        right: value.right,
                        bottom: value.bottom,
                    })
                    .into()
                }
                ResourceValue::CornerRadius(value) => {
                    windows_reference::IReference::from(native::CornerRadius {
                        top_left: value.top_left,
                        top_right: value.top_right,
                        bottom_right: value.bottom_right,
                        bottom_left: value.bottom_left,
                    })
                    .into()
                }
            };
            map.Insert(&key, &value)?;
        }
        if desired.is_empty() {
            self.resource_override_keys.remove(&object);
        } else {
            self.resource_override_keys.insert(object, desired);
        }
        Ok(())
    }

    pub(super) fn set_button_style(
        &mut self,
        object: ObjectId,
        value: Option<ButtonStyle>,
    ) -> Result<(), WinUiError> {
        let base = match value {
            None | Some(ButtonStyle::Default) => None,
            Some(value) => Some(value),
        };
        // State exists only while a style is applied, so clearing without state is a no-op.
        if base.is_none() && !self.style_states.contains_key(&object) {
            return Ok(());
        }
        self.style_states.entry(object).or_default().base = base;
        self.apply_style_state(object)
    }

    pub(super) fn set_theme_brush(
        &mut self,
        object: ObjectId,
        property: PropertyId,
        value: Option<ThemeBrush>,
    ) -> Result<(), WinUiError> {
        if value.is_none() && !self.style_states.contains_key(&object) {
            return Ok(());
        }
        let state = self.style_states.entry(object).or_default();
        if let Some(value) = value {
            state.brushes.insert(property, value);
        } else {
            state.brushes.remove(&property);
        }
        self.apply_style_state(object)
    }

    fn apply_style_state(&mut self, object: ObjectId) -> Result<(), WinUiError> {
        let kind = self.kind(object)?;
        let state = self.style_states.get(&object).cloned().unwrap_or_default();
        let element = self
            .ui_element(object)?
            .cast::<native::IFrameworkElement>()?;
        if state.base.is_none() && state.brushes.is_empty() {
            self.style_states.remove(&object);
            element.SetStyle(None::<&native::Style>)?;
            return Ok(());
        }
        if state.brushes.is_empty() {
            let style = self.lookup_button_style(state.base.unwrap())?;
            element.SetStyle(&style)?;
            return Ok(());
        }
        let key = NativeStyleKey {
            kind,
            base: state.base,
            brushes: state
                .brushes
                .iter()
                .map(|(property, brush)| (*property, *brush))
                .collect(),
        };
        let style = if let Some(style) = self.style_cache.get(&key) {
            style.clone()
        } else {
            let (target, properties) =
                theme_style_info(kind).ok_or(WinUiError::InvalidObject(object))?;
            let based_on = state
                .base
                .map(button_style_resource)
                .map(|resource| format!(" BasedOn='{{StaticResource {resource}}}'"))
                .unwrap_or_default();
            let mut xaml = format!(
                "<Style xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' \
                 TargetType='{target}'{based_on}>"
            );
            for (property, native_name) in properties {
                if let Some(brush) = state.brushes.get(property) {
                    xaml.push_str(&format!(
                        "<Setter Property='{native_name}' Value='{{ThemeResource {}}}'/>",
                        brush.resource_key()
                    ));
                }
            }
            xaml.push_str("</Style>");
            let style = native::XamlReader::Load(&xaml)?.cast::<native::Style>()?;
            self.style_cache.insert(key, style.clone());
            style
        };
        element.SetStyle(&style)?;
        Ok(())
    }

    fn lookup_button_style(&self, style: ButtonStyle) -> Result<native::Style, WinUiError> {
        let resources = native::Application::Current()?.Resources()?;
        let resources = resources.cast::<IMap<IInspectable, IInspectable>>()?;
        let key = windows_reference::IReference::from(button_style_resource(style));
        Ok(resources.Lookup(&key)?.cast()?)
    }
}

fn button_style_resource(style: ButtonStyle) -> &'static str {
    match style {
        ButtonStyle::Default => unreachable!("default button style has no resource"),
        ButtonStyle::Accent => "AccentButtonStyle",
        ButtonStyle::Subtle => "SubtleButtonStyle",
        ButtonStyle::TextLink => "TextBlockButtonStyle",
    }
}
