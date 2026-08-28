//! The gallery application shell: a `TitleBar` with search and a theme toggle above a
//! `NavigationView` that hosts Home, Settings, the 11 category listing pages, and the 65 leaf
//! control pages resolved through [`crate::router`].
//!
//! The pane lists Home, expandable categories with their control pages, and Settings. Typing in the
//! title bar's search box temporarily swaps the hierarchy for matching leaf destinations so every
//! control remains directly reachable from search.

use crate::controls::{CardItem, card_grid, category_icon, page_header};
use crate::registry::{self, ALL_CONTROLS, CATEGORIES, ControlInfo};
use crate::router;
use windows_reactor::*;

pub struct Gallery {
    backdrop: WindowBackdrop,
    history: Vec<String>,
    pane_open: bool,
    search: String,
    selected_tag: String,
    theme: WindowTheme,
}

#[derive(Clone)]
pub enum Message {
    Back,
    BackdropChanged(WindowBackdrop),
    CycleTheme,
    Navigate(String),
    PaneOpenChanged(bool),
    SearchChanged(String),
    SelectedTagChanged(Option<String>),
    TogglePane,
}

fn theme_name(theme: WindowTheme) -> &'static str {
    match theme {
        WindowTheme::System => "System",
        WindowTheme::Light => "Light",
        WindowTheme::Dark => "Dark",
    }
}

/// The destination title shown in the title bar subtitle for the current tag.
fn destination_title(tag: &str) -> String {
    if tag == "home" {
        return "Home".to_string();
    }
    if tag == "settings" {
        return "Settings".to_string();
    }
    if let Some(category) = CATEGORIES.iter().find(|c| registry::category_tag(c) == tag) {
        return (*category).to_string();
    }
    ALL_CONTROLS
        .iter()
        .find(|c| c.tag == tag)
        .map_or_else(|| tag.to_string(), |c| c.title.to_string())
}

fn nav_item(
    tag: &str,
    label: &str,
    icon: Option<Symbol>,
    selected: bool,
    expanded: bool,
    children: Vec<KeyedView>,
) -> KeyedView {
    let has_children = !children.is_empty();
    let mut slots = vec![SlotView::new(NavigationViewItemSlot::Content, label)];
    if let Some(symbol) = icon {
        slots.push(SlotView::new(
            NavigationViewItemSlot::Icon,
            SymbolIcon::new().symbol(symbol),
        ));
    }
    if has_children {
        slots.push(SlotView::collection(
            NavigationViewItemSlot::MenuItems,
            children,
        ));
    }
    let mut item = NavigationViewItem::new()
        .tag(tag)
        .is_selected(selected)
        .selects_on_invoked(true);
    if has_children {
        item = item.is_expanded(expanded);
    }
    KeyedView::new(tag.to_string(), item.slots(slots))
}

/// Renders a category's control list as a card grid, matching the incumbent gallery's category
/// page. This stays a plain view function (not a `Component`) because it owns no state of its
/// own; it is recomputed directly from the registry and the shell's navigation callback.
fn category_page(category: &'static str, on_navigate: Callback<String>) -> View {
    let controls = registry::controls_in_category(category);
    let count = controls.len();
    let items: Vec<CardItem> = controls
        .iter()
        .map(|c| CardItem {
            title: c.title.to_string(),
            subtitle: c.description.to_string(),
            image_file: c.image.to_string(),
            key: c.tag.to_string(),
        })
        .collect();

    ScrollViewer::new().content(
        Border::new()
            .padding(Thickness::new(36.0, 24.0, 36.0, 36.0))
            .content(StackPanel::new().spacing(24.0).children((
                page_header(category, &format!("{count} controls")),
                card_grid(&items, move |tag| {
                    let _ = on_navigate.call(tag);
                }),
            ))),
    )
}

impl Gallery {
    fn navigate(&mut self, tag: String) {
        if tag != self.selected_tag {
            self.history
                .push(std::mem::replace(&mut self.selected_tag, tag));
        }
    }
}

impl Component for Gallery {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            backdrop: WindowBackdrop::Mica,
            history: Vec::new(),
            pane_open: true,
            search: String::new(),
            selected_tag: "home".to_string(),
            theme: WindowTheme::System,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Back => {
                if let Some(tag) = self.history.pop() {
                    self.selected_tag = tag;
                }
            }
            Message::BackdropChanged(value) => self.backdrop = value,
            Message::CycleTheme => {
                self.theme = match self.theme {
                    WindowTheme::System => WindowTheme::Light,
                    WindowTheme::Light => WindowTheme::Dark,
                    WindowTheme::Dark => WindowTheme::System,
                };
            }
            Message::Navigate(tag) => self.navigate(tag),
            Message::PaneOpenChanged(value) => self.pane_open = value,
            Message::SearchChanged(value) => self.search = value,
            Message::SelectedTagChanged(tag) => {
                if let Some(tag) = tag {
                    self.navigate(tag);
                }
            }
            Message::TogglePane => self.pane_open = !self.pane_open,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title(format!(
            "Reactor gallery - {}",
            destination_title(&self.selected_tag)
        ));
        context.window_visuals(
            WindowVisuals::new()
                .theme(self.theme)
                .backdrop(self.backdrop)
                .client_size(1400.0, 900.0),
        );

        let content: View = if self.selected_tag == "home" {
            View::component::<crate::pages::home::HomePage>(crate::pages::home::HomeInput {
                on_navigate: context.callback(Message::Navigate),
            })
        } else if self.selected_tag == "settings" {
            View::component::<crate::pages::settings::SettingsPage>(())
        } else if self.selected_tag == "materials" {
            View::component::<crate::pages::design::MaterialsPage>(
                crate::pages::design::MaterialsInput {
                    backdrop: self.backdrop,
                    on_backdrop_changed: context.callback(Message::BackdropChanged),
                },
            )
        } else if let Some(category) = CATEGORIES
            .iter()
            .find(|c| registry::category_tag(c) == self.selected_tag)
        {
            category_page(category, context.callback(Message::Navigate))
        } else {
            router::route(&self.selected_tag)
        };

        let selected_menu_tag = if self.search.trim().is_empty() {
            ALL_CONTROLS
                .iter()
                .find(|control| control.tag == self.selected_tag)
                .map_or_else(
                    || self.selected_tag.clone(),
                    |control| registry::category_tag(control.category),
                )
        } else {
            self.selected_tag.clone()
        };
        let mut menu_items: Vec<KeyedView> = vec![nav_item(
            "home",
            "Home",
            Some(Symbol::Home),
            selected_menu_tag == "home",
            false,
            Vec::new(),
        )];
        if self.search.trim().is_empty() {
            for &category in CATEGORIES {
                let tag = registry::category_tag(category);
                let selected = selected_menu_tag == tag;
                let children = registry::controls_in_category(category)
                    .into_iter()
                    .map(|control| {
                        nav_item(
                            control.tag,
                            control.title,
                            None,
                            self.selected_tag == control.tag,
                            false,
                            Vec::new(),
                        )
                    })
                    .collect();
                menu_items.push(nav_item(
                    &tag,
                    category,
                    Some(category_icon(category)),
                    self.selected_tag == tag,
                    selected,
                    children,
                ));
            }
        } else {
            let matches: Vec<&ControlInfo> = registry::search(&self.search);
            for info in &matches {
                menu_items.push(nav_item(
                    info.tag,
                    info.title,
                    None,
                    self.selected_tag == info.tag,
                    false,
                    Vec::new(),
                ));
            }
            let already_shown = matches.iter().any(|info| info.tag == self.selected_tag);
            if !already_shown
                && let Some(info) = ALL_CONTROLS.iter().find(|c| c.tag == self.selected_tag)
            {
                menu_items.push(nav_item(
                    info.tag,
                    info.title,
                    None,
                    true,
                    false,
                    Vec::new(),
                ));
            }
        }
        menu_items.push(nav_item(
            "settings",
            "Settings",
            Some(Symbol::Setting),
            selected_menu_tag == "settings",
            false,
            Vec::new(),
        ));

        let navigation = NavigationView::new()
            .pane_display_mode(NavigationViewPaneDisplayMode::Left)
            .is_pane_toggle_button_visible(false)
            .is_back_button_visible(NavigationViewBackButtonVisible::Collapsed)
            .is_settings_visible(false)
            .always_show_header(false)
            .pane_title("Reactor gallery")
            .is_pane_open(self.pane_open)
            .on_is_pane_open_changed(context.callback(Message::PaneOpenChanged))
            .on_selected_tag_changed(context.callback(Message::SelectedTagChanged))
            .grid_row(1)
            .slots([
                SlotView::collection(NavigationViewSlot::MenuItems, menu_items),
                SlotView::new(NavigationViewSlot::Content, content),
            ]);

        let title_bar = TitleBar::new()
            .preferred_height(WindowTitleBarHeight::Tall)
            .title("Reactor gallery")
            .subtitle(destination_title(&self.selected_tag))
            .is_back_button_visible(true)
            .is_back_button_enabled(!self.history.is_empty())
            .is_pane_toggle_button_visible(true)
            .on_back_requested(context.message(Message::Back))
            .on_pane_toggle_requested(context.message(Message::TogglePane))
            .grid_row(0)
            .slots([
                SlotView::new(
                    TitleBarSlot::Content,
                    TextBox::new()
                        .text(self.search.clone())
                        .placeholder_text("Search controls and samples...")
                        .on_text_changed(context.callback(Message::SearchChanged)),
                ),
                SlotView::new(
                    TitleBarSlot::RightHeader,
                    Button::new()
                        .on_click(context.message(Message::CycleTheme))
                        .content(format!("Theme: {}", theme_name(self.theme))),
                ),
            ]);

        Grid::new()
            .rows([GridLength::Auto, GridLength::Star(1.0)])
            .children((title_bar, navigation))
    }
}
