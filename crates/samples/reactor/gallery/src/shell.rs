use std::rc::Rc;

use windows_reactor::{
    Application, AutoSuggestBox, Button, Element, Icon, IconSymbol, NavigationItem,
    NavigationPaneDisplayMode, NavigationView, RenderCx, TitleBar, TitleBarHeight, Window,
    WindowBackdrop, WindowTheme,
};

use crate::{registry, router};

pub fn gallery(cx: &mut RenderCx<'_>) -> Element {
    let open = cx.use_state(|| true);
    let selected = cx.use_state(|| router::HOME_KEY);
    let history = cx.use_state(Vec::<u64>::new);
    let search = cx.use_state(String::new);
    let pane_open = cx.use_state(|| true);
    let theme = cx.use_state(|| WindowTheme::System);

    let current = selected.value();
    let current_history = history.value();
    let current_search = search.value();
    let current_pane_open = pane_open.value();
    let current_theme = theme.value();

    let navigate: Rc<dyn Fn(u64)> = Rc::new({
        let selected = selected.clone();
        let history = history.clone();
        move |next| {
            if next != current {
                history.update(|values| values.push(current));
                selected.set(next);
            }
        }
    });

    let search_items = registry::ALL_CONTROLS
        .iter()
        .enumerate()
        .filter(|(_, info)| {
            let query = current_search.to_lowercase();
            !query.is_empty()
                && (info.title.to_lowercase().contains(&query)
                    || info.description.to_lowercase().contains(&query))
        })
        .map(|(index, info)| (router::control_key(index), info.title));

    let search_box = AutoSuggestBox::new(current_search.clone(), {
        let search = search.clone();
        move |value| {
            search.set(value);
        }
    })
    .items(search_items)
    .placeholder_text("Search controls and samples...")
    .on_query_submitted({
        let navigate = Rc::clone(&navigate);
        move |query| {
            if let Some((index, _)) = registry::ALL_CONTROLS
                .iter()
                .enumerate()
                .find(|(_, info)| info.title.eq_ignore_ascii_case(query.trim()))
            {
                navigate(router::control_key(index));
            }
        }
    })
    .on_suggestion_chosen({
        let navigate = Rc::clone(&navigate);
        move |key| {
            if let Some(info) = router::control_info(key) {
                search.set(info.title.to_string());
                navigate(key);
            }
        }
    })
    .width(380.0)
    .build();

    let title_bar = TitleBar::custom("Reactor WinUI Gallery")
        .content(search_box)
        .right_header(
            Button::new(match current_theme {
                WindowTheme::Dark => "Light theme",
                _ => "Dark theme",
            })
            .on_click(move || {
                theme.set(if current_theme == WindowTheme::Dark {
                    WindowTheme::Light
                } else {
                    WindowTheme::Dark
                });
            })
            .build(),
        )
        .back_button_visible(true)
        .back_button_enabled(!current_history.is_empty())
        .pane_toggle_button_visible(true)
        .height(TitleBarHeight::Tall)
        .on_back_requested({
            move || {
                let mut values = current_history.clone();
                if let Some(previous) = values.pop() {
                    history.set(values);
                    selected.set(previous);
                }
            }
        })
        .on_pane_requested({
            let pane_open = pane_open.clone();
            move || {
                pane_open.set(!current_pane_open);
            }
        });

    let mut items = vec![
        NavigationItem::new(router::HOME_KEY, "Home").icon(Icon::symbol(IconSymbol::HOME)),
        NavigationItem::new(router::SETTINGS_KEY, "Settings")
            .icon(Icon::symbol(IconSymbol::SETTINGS)),
    ];
    items.extend(
        registry::CATEGORIES
            .iter()
            .enumerate()
            .map(|(index, category)| NavigationItem::new(router::category_key(index), *category)),
    );
    items.extend(
        registry::ALL_CONTROLS
            .iter()
            .enumerate()
            .map(|(index, info)| {
                NavigationItem::new(
                    router::control_key(index),
                    format!("{} / {}", info.category, info.title),
                )
            }),
    );

    let content = router::route(current, Rc::clone(&navigate));
    let navigation = NavigationView::new(items, content, move |key| {
        if let Some(key) = key {
            navigate(key);
        }
    })
    .selected_key(Some(current))
    .pane_display_mode(NavigationPaneDisplayMode::Left)
    .pane_title("Reactor Gallery")
    .pane_toggle_visible(false)
    .pane_open(current_pane_open, move |value| {
        pane_open.set(value);
    })
    .settings_visible(false)
    .build();

    let windows = if open.value() {
        vec![
            Window::new("Reactor WinUI Gallery", navigation, move || {
                open.set(false);
            })
            .client_size(1400.0, 900.0)
            .backdrop(WindowBackdrop::Mica)
            .theme(current_theme)
            .title_bar(title_bar)
            .build()
            .key(0),
        ]
    } else {
        Vec::new()
    };

    Application::new(windows).build()
}
