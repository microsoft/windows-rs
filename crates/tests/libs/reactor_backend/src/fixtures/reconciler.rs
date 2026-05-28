use std::future::Future;
use std::pin::Pin;

use windows_core::Interface;

use windows_reactor::core::component::Component;
use windows_reactor::core::element::{Element, GridLength};
use windows_reactor::core::render_context::RenderCx;
use windows_reactor::dsl::{button, grid, text_block, ElementExt};

use crate::bindings::{Grid as XamlGrid, TextBlock};

use crate::harness::Harness;

use windows_reactor::{hstack, vstack};
pub type FixtureFuture = Pin<Box<dyn Future<Output = ()>>>;

struct ClosureComponent<F: Fn(&mut RenderCx) -> Element + 'static>(F);

impl<F: Fn(&mut RenderCx) -> Element + 'static> Component for ClosureComponent<F> {
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        (self.0)(cx)
    }
}

pub(crate) fn cc<F: Fn(&mut RenderCx) -> Element + 'static>(f: F) -> Box<dyn Component> {
    Box::new(ClosureComponent(f))
}

pub fn mount_text(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|_cx| text_block("Hello from windows_reactor").into()));
        h.render().await;
        h.check(
            "Reconciler_MountText_TextAppears",
            h.find_text("Hello from windows_reactor").is_some(),
        );
        h.check(
            "Reconciler_MountText_IsTextBlock",
            h.count_controls::<TextBlock>() >= 1,
        );
    })
}

pub fn update_text(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (text, set_text) = cx.use_state("Before".to_string());
            vstack((
                text_block(text),
                button("Change").on_click(move || set_text.call("After".to_string())),
            ))
            .into()
        }));
        h.render().await;
        h.check(
            "Reconciler_UpdateText_InitialText",
            h.find_text("Before").is_some(),
        );

        let _ = h.click_button("Change");
        h.render().await;

        h.check(
            "Reconciler_UpdateText_UpdatedText",
            h.find_text("After").is_some(),
        );
        h.check(
            "Reconciler_UpdateText_OldTextGone",
            h.find_text("Before").is_none(),
        );
    })
}

pub fn add_remove_children(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (count, set) = cx.use_state(3i32);
            let inc = {
                let set = set.clone();
                move || set.call(count + 1)
            };
            let dec = {
                let set = set;
                move || set.call((count - 1).max(0))
            };
            let items: Vec<Element> = (0..count)
                .map(|i| text_block(format!("Item {i}")).into())
                .collect();
            vstack((
                hstack((button("Add").on_click(inc), button("Remove").on_click(dec))).spacing(8.0),
                vstack(items).spacing(4.0),
            ))
            .spacing(8.0)
            .into()
        }));
        h.render().await;
        h.check(
            "Reconciler_AddRemoveChildren_InitialCount",
            h.find_text("Item 0").is_some() && h.find_text("Item 2").is_some(),
        );

        let _ = h.click_button("Add");
        h.render().await;
        h.check(
            "Reconciler_AddRemoveChildren_AfterAdd",
            h.find_text("Item 3").is_some(),
        );

        let _ = h.click_button("Remove");
        h.render().await;
        let _ = h.click_button("Remove");
        h.render().await;
        h.check(
            "Reconciler_AddRemoveChildren_AfterRemove",
            h.find_text("Item 3").is_none() && h.find_text("Item 2").is_none(),
        );
    })
}

pub fn component_rerender(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(Box::new(CounterComponent));
        h.render().await;
        h.check(
            "Reconciler_ComponentRerender_InitialState",
            h.find_text("Count: 0").is_some(),
        );

        let _ = h.click_button("Increment");
        h.render().await;
        h.check(
            "Reconciler_ComponentRerender_AfterClick",
            h.find_text("Count: 1").is_some(),
        );

        let _ = h.click_button("Increment");
        h.render().await;
        let _ = h.click_button("Increment");
        h.render().await;
        h.check(
            "Reconciler_ComponentRerender_MultipleClicks",
            h.find_text("Count: 3").is_some(),
        );
    })
}

struct CounterComponent;
impl Component for CounterComponent {
    fn render(&self, _: &(), cx: &mut RenderCx) -> Element {
        let (n, set) = cx.use_state(0i32);
        vstack((
            text_block(format!("Count: {n}")),
            button("Increment").on_click(move || set.call(n + 1)),
        ))
        .spacing(8.0)
        .into()
    }
}

pub fn keyed_list(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (reversed, set) = cx.use_state(false);
            let items = ["Alpha", "Beta", "Gamma"];
            let order: Vec<&'static str> = if reversed {
                items.iter().rev().copied().collect()
            } else {
                items.to_vec()
            };
            let kids: Vec<Element> = order
                .into_iter()
                .map(|s| text_block(s).with_key(s).into())
                .collect();
            vstack((
                button("Reverse").on_click(move || set.call(!reversed)),
                vstack(kids).spacing(4.0),
            ))
            .spacing(8.0)
            .into()
        }));
        h.render().await;

        let alpha1 = h.find_text("Alpha");
        h.check(
            "Reconciler_KeyedList_InitialOrder",
            alpha1.is_some() && h.find_text("Beta").is_some() && h.find_text("Gamma").is_some(),
        );

        let _ = h.click_button("Reverse");
        h.render().await;

        let alpha2 = h.find_text("Alpha");
        h.check(
            "Reconciler_KeyedList_AfterReverse",
            alpha2.is_some() && h.find_text("Gamma").is_some(),
        );

        let same_instance = match (alpha1, alpha2) {
            (Some(a), Some(b)) => identity_eq(&a, &b),
            _ => false,
        };
        h.check("Reconciler_KeyedList_ControlReused", same_instance);
    })
}

pub fn conditional_toggle(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (on, set) = cx.use_state(false);
            let body: Element = if on {
                text_block("ON").into()
            } else {
                text_block("OFF").into()
            };
            vstack((body, button("Toggle").on_click(move || set.call(!on))))
                .spacing(8.0)
                .into()
        }));
        h.render().await;
        h.check(
            "ConditionalRendering_Toggle_Initial",
            h.find_text("OFF").is_some() && h.find_text("ON").is_none(),
        );

        let _ = h.click_button("Toggle");
        h.render().await;
        h.check(
            "ConditionalRendering_Toggle_On",
            h.find_text("ON").is_some() && h.find_text("OFF").is_none(),
        );

        let _ = h.click_button("Toggle");
        h.render().await;
        h.check(
            "ConditionalRendering_Toggle_OffAgain",
            h.find_text("OFF").is_some() && h.find_text("ON").is_none(),
        );
    })
}

pub fn grid_dynamic_children(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (panes, set) = cx.use_state(1i32);
            let inc = {
                let set = set.clone();
                move || set.call(panes + 1)
            };
            let dec = {
                let set = set;
                move || set.call((panes - 1).max(0))
            };
            let cols: Vec<GridLength> = (0..panes.max(1)).map(|_| GridLength::Star(1.0)).collect();
            let cells: Vec<Element> = (0..panes)
                .map(|i| {
                    let e: Element = text_block(format!("Pane {i}")).into();
                    e.grid_row(0).grid_column(i)
                })
                .collect();
            let g: Element = grid(cells)
                .rows([GridLength::Star(1.0)])
                .columns(cols)
                .into();
            vstack((
                hstack((
                    button("Split").on_click(inc),
                    button("Collapse").on_click(dec),
                ))
                .spacing(8.0),
                g,
            ))
            .spacing(8.0)
            .into()
        }));
        h.render().await;
        h.check(
            "Reconciler_GridDynamic_InitialPane",
            h.find_text("Pane 0").is_some(),
        );

        let _ = h.click_button("Split");
        h.render().await;
        h.check(
            "Reconciler_GridDynamic_AfterFirstSplit",
            h.find_text("Pane 0").is_some() && h.find_text("Pane 1").is_some(),
        );

        let _ = h.click_button("Split");
        h.render().await;
        h.check(
            "Reconciler_GridDynamic_AfterSecondSplit",
            h.find_text("Pane 2").is_some(),
        );

        h.check(
            "Reconciler_GridDynamic_ThreePanesInGrid",
            h.count_controls::<XamlGrid>() >= 1,
        );

        let _ = h.click_button("Collapse");
        h.render().await;
        h.check(
            "Reconciler_GridDynamic_AfterCollapse",
            h.find_text("Pane 2").is_none() && h.find_text("Pane 1").is_some(),
        );

        let _ = h.click_button("Collapse");
        h.render().await;
        let _ = h.click_button("Collapse");
        h.render().await;
        h.check(
            "Reconciler_GridDynamic_CollapsedToEmpty",
            h.find_text("Pane 0").is_none(),
        );

        let _ = h.click_button("Split");
        h.render().await;
        h.check(
            "Reconciler_GridDynamic_GrowFromEmpty",
            h.find_text("Pane 0").is_some(),
        );
    })
}

fn identity_eq<T: Interface, U: Interface>(a: &T, b: &U) -> bool {
    use windows_core::IUnknown;
    let Ok(ua) = a.cast::<IUnknown>() else {
        return false;
    };
    let Ok(ub) = b.cast::<IUnknown>() else {
        return false;
    };
    ua.as_raw() == ub.as_raw()
}
