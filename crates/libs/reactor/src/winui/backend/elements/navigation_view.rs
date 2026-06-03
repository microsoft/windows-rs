//! NavigationView — property dispatch.

use crate::bindings as Xaml;
use crate::core::backend::{Prop, PropValue};
use windows_core::Interface;

pub(in crate::winui::backend) fn set_prop(
    nv: &Xaml::NavigationView,
    prop: Prop,
    value: &PropValue,
) -> Option<windows_core::Result<()>> {
    match (prop, value) {
        (Prop::NavMenuItems, PropValue::NavMenuItems(items)) => Some((|| {
            let menu = nv.get_MenuItems()?;
            menu.Clear()?;
            for item in items {
                let nv_item = super::super::build_nav_view_item(item)?;
                menu.Append(&nv_item)?;
            }
            Ok(())
        })()),
        (Prop::IsPaneOpen, PropValue::Bool(v)) => Some(nv.put_IsPaneOpen(*v)),
        (Prop::PaneDisplayMode, PropValue::NavPaneDisplayMode(mode)) => Some((|| {
            use crate::core::widgets::NavViewPaneDisplayMode as M;
            use Xaml::NavigationViewPaneDisplayMode as W;
            let mapped = match mode {
                M::Auto => W::Auto,
                M::Left => W::Left,
                M::Top => W::Top,
                M::LeftCompact => W::LeftCompact,
                M::LeftMinimal => W::LeftMinimal,
            };
            nv.cast::<Xaml::INavigationView2>()?
                .put_PaneDisplayMode(mapped)
        })()),
        (Prop::IsBackEnabled, PropValue::Bool(v)) => Some((|| {
            nv.cast::<Xaml::INavigationView2>()?.put_IsBackEnabled(*v)
        })()),
        (Prop::IsSettingsVisible, PropValue::Bool(v)) => Some(nv.put_IsSettingsVisible(*v)),
        (Prop::PaneTitle, PropValue::Str(s)) => Some((|| {
            nv.cast::<Xaml::INavigationView2>()?
                .put_PaneTitle(s.as_str())
        })()),
        (Prop::PaneTitle, PropValue::Unset) => {
            Some((|| nv.cast::<Xaml::INavigationView2>()?.put_PaneTitle(""))())
        }
        (Prop::NavHeaderString, PropValue::Str(s)) => Some((|| {
            let tb = super::super::string_as_textblock(s)?;
            nv.put_Header(&tb)
        })()),
        (Prop::NavHeaderString, PropValue::Unset) => Some(nv.put_Header(None)),
        (Prop::NavSelectedTag, PropValue::Str(tag)) => {
            Some(super::super::select_nav_item_by_tag(nv, tag))
        }
        (Prop::NavSelectedTag, PropValue::Unset) => Some(nv.put_SelectedItem(None)),
        (Prop::NavAutoSuggestBox, PropValue::Bool(true)) => Some((|| {
            let asb = Xaml::AutoSuggestBox::new()?;
            nv.put_AutoSuggestBox(&asb)
        })()),
        (Prop::NavAutoSuggestBox, PropValue::Bool(false)) => Some(nv.put_AutoSuggestBox(None)),
        (Prop::NavAutoSuggestPlaceholder, PropValue::Str(s)) => Some((|| {
            if let Ok(asb) = nv.get_AutoSuggestBox() {
                asb.put_PlaceholderText(s.as_str())?;
            }
            Ok(())
        })()),
        (Prop::NavAutoSuggestItems, PropValue::StrList(items)) => Some((|| {
            if let Ok(asb) = nv.get_AutoSuggestBox() {
                let vec: Vec<Option<windows_core::IInspectable>> = items
                    .iter()
                    .map(|s| {
                        let r = windows_reference::IReference::from(s.as_str());
                        Some(r.into())
                    })
                    .collect();
                let ivec: windows_collections::IVector<windows_core::IInspectable> = vec.into();
                asb.cast::<Xaml::IItemsControl>()?.put_ItemsSource(&ivec)?;
            }
            Ok(())
        })()),
        (Prop::IsBackButtonVisible, PropValue::Bool(v)) => Some((|| {
            let val = if *v {
                Xaml::NavigationViewBackButtonVisible::Auto
            } else {
                Xaml::NavigationViewBackButtonVisible::Collapsed
            };
            nv.cast::<Xaml::INavigationView2>()?
                .put_IsBackButtonVisible(val)
        })()),
        (Prop::IsPaneToggleButtonVisible, PropValue::Bool(v)) => {
            Some(nv.put_IsPaneToggleButtonVisible(*v))
        }
        _ => None,
    }
}
