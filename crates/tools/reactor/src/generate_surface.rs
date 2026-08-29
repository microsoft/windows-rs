use proc_macro2::{Ident, Span};
use quote::quote;

use crate::helpers::to_snake_case;
use crate::schema::{
    PropertyAdapter, ResolvedControl, ResolvedEvent, ResolvedPlacement, ResolvedProperty,
    ResolvedSchema, Role, SlotShape, SlotTarget, ValueValidation,
};

pub(crate) fn generate(schema: &ResolvedSchema) -> String {
    let capability_properties = capability_property_cases();
    let capability_builders = capability_properties.iter().map(|property| {
        let function = ident(&format!(
            "capability_{}_{}",
            to_snake_case(property.capability),
            to_snake_case(property.property)
        ));
        let cleared = &property.cleared;
        let initial = &property.initial;
        let alternate = &property.alternate;
        quote! {
            fn #function(stage: usize) -> View {
                match stage {
                    0 | 3 => #cleared,
                    1 => #initial,
                    2 => #alternate,
                    _ => unreachable!(),
                }
            }
        }
    });
    let capability_cases = capability_properties.iter().map(|property| {
        let function = ident(&format!(
            "capability_{}_{}",
            to_snake_case(property.capability),
            to_snake_case(property.property)
        ));
        let name = format!("capability.{}.{}", property.capability, property.property);
        quote! {
            SurfaceCase {
                name: #name,
                kind: SurfaceKind::CapabilityProperty,
                stages: 4,
                subscription_delta: None,
                build: #function,
            }
        }
    });
    let capability_inventory = capability_properties.iter().map(|property| {
        let capability = property.capability;
        let property = property.property;
        quote! {
            CapabilityPropertySurface {
                capability: #capability,
                property: #property,
            }
        }
    });
    let builders = schema.controls.iter().map(|control| {
        let name = ident(&control.name);
        let function = ident(&format!("construct_{}", to_snake_case(&control.name)));
        match control.placement {
            ResolvedPlacement::Visual | ResolvedPlacement::Declaration => quote! {
                fn #function(_stage: usize) -> View {
                    Grid::new().children((#name::new(),))
                }
            },
            ResolvedPlacement::WindowLifetime if !control.slots.is_empty() => {
                let slot_type = ident(&format!("{}Slot", control.name));
                quote! {
                    fn #function(_stage: usize) -> View {
                        Grid::new().children((
                            #name::new().slots(std::iter::empty::<SlotView<#slot_type>>()),
                        ))
                    }
                }
            }
            ResolvedPlacement::WindowLifetime => quote! {
                fn #function(_stage: usize) -> View {
                    Grid::new().children((#name::new(),))
                }
            },
            ResolvedPlacement::TooltipAttachment => quote! {
                fn #function(_stage: usize) -> View {
                    TextBlock::new()
                        .text("tooltip target")
                        .tooltip_with(Tooltip::rich(TextBlock::new().text("tooltip content")))
                }
            },
        }
    });
    let ordered_controls = schema
        .controls
        .iter()
        .filter(|control| control.placement != ResolvedPlacement::WindowLifetime)
        .chain(
            schema
                .controls
                .iter()
                .filter(|control| control.placement == ResolvedPlacement::WindowLifetime),
        );
    let mut property_builders = Vec::new();
    let mut event_builders = Vec::new();
    let mut structural_builders = Vec::new();
    let mut structural_inventory = Vec::new();
    let mut cases = Vec::new();
    for control in ordered_controls {
        let construct = ident(&format!("construct_{}", to_snake_case(&control.name)));
        let construct_name = format!("control.{}.construct", control.name);
        cases.push(quote! {
            SurfaceCase {
                name: #construct_name,
                kind: SurfaceKind::Control,
                stages: 1,
                subscription_delta: None,
                build: #construct,
            }
        });
        for property in &control.properties {
            let function = ident(&format!(
                "property_{}_{}",
                to_snake_case(&control.name),
                property.field
            ));
            let case_name = format!("property.{}.{}", control.name, property.name);
            let control_name = ident(&control.name);
            let (initial, alternate) = property_values(control, property);
            let initial = apply_property(control, property, initial);
            let alternate = apply_property(control, property, alternate);
            let cleared = wrap_control(control, quote! { #control_name::new() });
            property_builders.push(quote! {
                fn #function(stage: usize) -> View {
                    match stage {
                        0 | 3 => #cleared,
                        1 => #initial,
                        2 => #alternate,
                        _ => unreachable!(),
                    }
                }
            });
            cases.push(quote! {
                SurfaceCase {
                    name: #case_name,
                    kind: SurfaceKind::Property,
                    stages: 4,
                    subscription_delta: None,
                    build: #function,
                }
            });
        }
        if control.placement != ResolvedPlacement::TooltipAttachment {
            let control_name = ident(&control.name);
            if control.content.is_some() {
                add_structural_case(
                    control,
                    "Content",
                    wrap_structural(control, quote! { #control_name::new() }),
                    wrap_structural(
                        control,
                        quote! {
                            #control_name::new().content(TextBlock::new().text("surface a"))
                        },
                    ),
                    wrap_structural(
                        control,
                        quote! {
                            #control_name::new().content(TextBlock::new().text("surface b"))
                        },
                    ),
                    &mut structural_builders,
                    &mut cases,
                    &mut structural_inventory,
                );
            } else if matches!(control.role, Role::Children) {
                add_structural_case(
                    control,
                    "Children",
                    wrap_structural(control, quote! { #control_name::new() }),
                    wrap_structural(
                        control,
                        quote! {
                            #control_name::new().children((TextBlock::new().text("surface a"),))
                        },
                    ),
                    wrap_structural(
                        control,
                        quote! {
                            #control_name::new().children((
                                TextBlock::new().text("surface a"),
                                TextBlock::new().text("surface b"),
                            ))
                        },
                    ),
                    &mut structural_builders,
                    &mut cases,
                    &mut structural_inventory,
                );
            } else if matches!(control.role, Role::Virtual) {
                add_structural_case(
                    control,
                    "Items",
                    wrap_structural(control, quote! { #control_name::new() }),
                    wrap_structural(
                        control,
                        quote! {
                            #control_name::new()
                                .item("a", TextBlock::new().text("surface a"))
                        },
                    ),
                    wrap_structural(
                        control,
                        quote! {
                            #control_name::new()
                                .item("a", TextBlock::new().text("surface a"))
                                .item("b", TextBlock::new().text("surface b"))
                        },
                    ),
                    &mut structural_builders,
                    &mut cases,
                    &mut structural_inventory,
                );
            }
            for slot in &control.slots {
                let slot_type = ident(&format!("{}Slot", control.name));
                let slot_name = ident(&slot.name);
                let item_control = control
                    .selection
                    .as_ref()
                    .filter(|selection| selection.slot == slot.name)
                    .map(|selection| selection.item.as_str())
                    .or_else(|| slot.item_controls.first().map(String::as_str))
                    .or_else(|| {
                        slot.collection_item.as_deref().and_then(|item| {
                            schema
                                .controls
                                .iter()
                                .find(|candidate| candidate.type_name == item)
                                .map(|candidate| candidate.name.as_str())
                        })
                    });
                let (initial_child, alternate_child) = if let Some(item) = item_control {
                    let item = ident(item);
                    (
                        quote! { #item::new().width(40.0) },
                        quote! { #item::new().width(80.0) },
                    )
                } else {
                    let target = match slot.shape {
                        SlotShape::Single(target) => target,
                        SlotShape::Collection => SlotTarget::UiElement,
                    };
                    (
                        structural_child(target, "surface a", false),
                        structural_child(target, "surface b", true),
                    )
                };
                let initial = match slot.shape {
                    SlotShape::Single(_) => quote! {
                        #control_name::new().slot(#slot_type::#slot_name, #initial_child)
                    },
                    SlotShape::Collection => quote! {
                        #control_name::new().collection_slot(
                            #slot_type::#slot_name,
                            [KeyedView::new("surface", #initial_child)],
                        )
                    },
                };
                let alternate = match slot.shape {
                    SlotShape::Single(_) => quote! {
                        #control_name::new().slot(#slot_type::#slot_name, #alternate_child)
                    },
                    SlotShape::Collection => quote! {
                        #control_name::new().collection_slot(
                            #slot_type::#slot_name,
                            [KeyedView::new("surface", #alternate_child)],
                        )
                    },
                };
                add_structural_case(
                    control,
                    &format!("Slot.{}", slot.name),
                    wrap_structural(
                        control,
                        quote! {
                            #control_name::new()
                                .slots(std::iter::empty::<SlotView<#slot_type>>())
                        },
                    ),
                    wrap_structural(control, initial),
                    wrap_structural(control, alternate),
                    &mut structural_builders,
                    &mut cases,
                    &mut structural_inventory,
                );
            }
        }
        for event in &control.events {
            let function = ident(&format!(
                "event_{}_{}",
                to_snake_case(&control.name),
                event.field
            ));
            let case_name = format!("event.{}.{}", control.name, event.name);
            let control_name = ident(&control.name);
            let field = ident(&event.field);
            let first_callback = event_callback(event, false);
            let alternate_callback = event_callback(event, true);
            let cleared = wrap_control(control, quote! { #control_name::new() });
            let initial = wrap_control(
                control,
                quote! { #control_name::new().#field(#first_callback) },
            );
            let alternate = wrap_control(
                control,
                quote! { #control_name::new().#field(#alternate_callback) },
            );
            let subscription_delta = usize::from(!control.event_always_active(event));
            event_builders.push(quote! {
                fn #function(stage: usize) -> View {
                    match stage {
                        0 | 3 => #cleared,
                        1 => #initial,
                        2 => #alternate,
                        _ => unreachable!(),
                    }
                }
            });
            cases.push(quote! {
                SurfaceCase {
                    name: #case_name,
                    kind: SurfaceKind::Event,
                    stages: 4,
                    subscription_delta: Some(#subscription_delta),
                    build: #function,
                }
            });
        }
    }
    let control_count = schema.controls.len();
    let property_count = schema
        .controls
        .iter()
        .map(|control| control.properties.len())
        .sum::<usize>();
    let event_count = schema
        .controls
        .iter()
        .map(|control| control.events.len())
        .sum::<usize>();
    let capability_property_count = capability_properties.len();
    let structural_count = structural_inventory.len();
    let properties = schema.controls.iter().flat_map(|control| {
        control.properties.iter().map(|property| {
            let control_name = &control.name;
            let property_name = &property.name;
            let value = &property.value;
            let adapter = property
                .adapter
                .map_or_else(|| "direct".to_string(), |adapter| format!("{adapter:?}"));
            let validation = property.validation.map_or_else(
                || quote! { None },
                |validation| {
                    let validation = format!("{validation:?}");
                    quote! { Some(#validation) }
                },
            );
            let theme_style = property.theme_style;
            quote! {
                PropertySurface {
                    control: #control_name,
                    property: #property_name,
                    value: #value,
                    adapter: #adapter,
                    validation: #validation,
                    clearable: true,
                    theme_style: #theme_style,
                }
            }
        })
    });
    let events = schema.controls.iter().flat_map(|control| {
        control.events.iter().map(|event| {
            let control_name = &control.name;
            let event_name = &event.name;
            let payload = &event.payload;
            let conversion = format!("{:?}", event.conversion);
            let subscription = if control.event_always_active(event) {
                "always"
            } else {
                "callback"
            };
            let delivery = event_delivery_owner(control, event);
            let active_property = event
                .active_property
                .as_ref()
                .map_or_else(|| quote! { None }, |property| quote! { Some(#property) });
            quote! {
                EventSurface {
                    control: #control_name,
                    event: #event_name,
                    payload: #payload,
                    conversion: #conversion,
                    subscription: #subscription,
                    delivery: #delivery,
                    active_property: #active_property,
                }
            }
        })
    });

    quote! {
        // Generated by `tool_reactor`. Do not edit.

        #![allow(clippy::useless_conversion)]

        use windows_reactor::*;

        #[derive(Clone, Copy, Eq, PartialEq)]
        pub(crate) enum SurfaceKind {
            Control,
            Property,
            Event,
            CapabilityProperty,
            Structural,
            Extension,
        }

        pub(crate) struct SurfaceCase {
            pub(crate) name: &'static str,
            pub(crate) kind: SurfaceKind,
            pub(crate) stages: usize,
            pub(crate) subscription_delta: Option<usize>,
            pub(crate) build: fn(usize) -> View,
        }

        pub struct PropertySurface {
            pub control: &'static str,
            pub property: &'static str,
            pub value: &'static str,
            pub adapter: &'static str,
            pub validation: Option<&'static str>,
            pub clearable: bool,
            pub theme_style: bool,
        }

        pub struct EventSurface {
            pub control: &'static str,
            pub event: &'static str,
            pub payload: &'static str,
            pub conversion: &'static str,
            pub subscription: &'static str,
            pub delivery: &'static str,
            pub active_property: Option<&'static str>,
        }

        pub struct CapabilityPropertySurface {
            pub capability: &'static str,
            pub property: &'static str,
        }

        pub struct StructuralSurface {
            pub control: &'static str,
            pub member: &'static str,
        }

        pub struct ExtensionSurface {
            pub name: &'static str,
        }

        pub(crate) const PROJECTED_CONTROL_COUNT: usize = #control_count;
        pub(crate) const PROJECTED_PROPERTY_COUNT: usize = #property_count;
        pub(crate) const PROJECTED_EVENT_COUNT: usize = #event_count;
        pub(crate) const CAPABILITY_PROPERTY_COUNT: usize = #capability_property_count;
        pub(crate) const STRUCTURAL_COUNT: usize = #structural_count;
        pub(crate) const EXTENSION_COUNT: usize = 5;

        #(#builders)*
        #(#property_builders)*
        #(#event_builders)*
        #(#capability_builders)*
        #(#structural_builders)*

        fn extension_tooltip(stage: usize) -> View {
            match stage {
                0 | 3 => TextBlock::new().text("owner").into(),
                1 => TextBlock::new().text("owner").tooltip("surface a"),
                2 => TextBlock::new().text("owner").tooltip_with(Tooltip::rich(
                    StackPanel::new().children((
                        TextBlock::new().text("surface b"),
                        TextBlock::new().text("detail"),
                    )),
                )),
                _ => unreachable!(),
            }
        }

        fn extension_flyout(stage: usize) -> View {
            match stage {
                0 | 3 => Button::new().content(TextBlock::new().text("owner")),
                1 => Button::new()
                    .content(TextBlock::new().text("owner"))
                    .flyout("surface a"),
                2 => Button::new()
                    .content(TextBlock::new().text("owner"))
                    .flyout_with(Flyout::rich(
                        StackPanel::new().children((
                            TextBlock::new().text("surface b"),
                            TextBlock::new().text("detail"),
                        )),
                    )),
                _ => unreachable!(),
            }
        }

        fn extension_menu(stage: usize) -> View {
            match stage {
                0 | 3 => Button::new().content(TextBlock::new().text("owner")),
                1 => Button::new()
                    .content(TextBlock::new().text("owner"))
                    .menu(Menu::new(
                        [
                            MenuItem::item("open", "Open"),
                            MenuItem::separator("separator"),
                        ],
                        |_| {},
                    )),
                2 => Button::new()
                    .content(TextBlock::new().text("owner"))
                    .menu(Menu::new(
                        [MenuItem::submenu(
                            "share",
                            "Share",
                            [MenuItem::item("email", "Email")],
                        )],
                        |_| {},
                    )),
                _ => unreachable!(),
            }
        }

        fn extension_command_bar_flyout(stage: usize) -> View {
            match stage {
                0 | 3 => Button::new().content(TextBlock::new().text("owner")),
                1 => Button::new()
                    .content(TextBlock::new().text("owner"))
                    .command_bar_flyout(CommandBarFlyout::new(
                        [CommandBarCommand::button("bold", "Bold")],
                        [CommandBarCommand::button("copy", "Copy")],
                        |_| {},
                    )),
                2 => Button::new()
                    .content(TextBlock::new().text("owner"))
                    .command_bar_flyout(CommandBarFlyout::new(
                        [CommandBarCommand::separator("separator")],
                        [CommandBarCommand::button("paste", "Paste")],
                        |_| {},
                    )),
                _ => unreachable!(),
            }
        }

        fn extension_tree_nodes(stage: usize) -> View {
            match stage {
                0 | 3 => TreeView::new().nodes(std::iter::empty::<TreeNode>()),
                1 => TreeView::new().nodes([
                    TreeNode::new("root", "Root").children([
                        TreeNode::new("first", "First"),
                        TreeNode::new("second", "Second"),
                    ]),
                ]),
                2 => TreeView::new().nodes([
                    TreeNode::new("root", "Changed").expanded(true).children([
                        TreeNode::new("second", "Second"),
                        TreeNode::new("third", "Third"),
                    ]),
                ]),
                _ => unreachable!(),
            }
        }

        pub(crate) static SURFACE_CASES: &[SurfaceCase] = &[
            #(#capability_cases),*,
            SurfaceCase {
                name: "extension.Tooltip",
                kind: SurfaceKind::Extension,
                stages: 4,
                subscription_delta: None,
                build: extension_tooltip,
            },
            SurfaceCase {
                name: "extension.Flyout",
                kind: SurfaceKind::Extension,
                stages: 4,
                subscription_delta: None,
                build: extension_flyout,
            },
            SurfaceCase {
                name: "extension.Menu",
                kind: SurfaceKind::Extension,
                stages: 4,
                subscription_delta: None,
                build: extension_menu,
            },
            SurfaceCase {
                name: "extension.CommandBarFlyout",
                kind: SurfaceKind::Extension,
                stages: 4,
                subscription_delta: None,
                build: extension_command_bar_flyout,
            },
            SurfaceCase {
                name: "extension.TreeView.Nodes",
                kind: SurfaceKind::Extension,
                stages: 4,
                subscription_delta: None,
                build: extension_tree_nodes,
            },
            #(#cases),*
        ];

        pub static PROJECTED_PROPERTIES: &[PropertySurface] = &[
            #(#properties),*
        ];

        pub static PROJECTED_EVENTS: &[EventSurface] = &[
            #(#events),*
        ];

        pub static CAPABILITY_PROPERTIES: &[CapabilityPropertySurface] = &[
            #(#capability_inventory),*
        ];

        pub static STRUCTURAL_SURFACES: &[StructuralSurface] = &[
            #(#structural_inventory),*
        ];

        pub static EXTENSION_SURFACES: &[ExtensionSurface] = &[
            ExtensionSurface { name: "Tooltip" },
            ExtensionSurface { name: "Flyout" },
            ExtensionSurface { name: "Menu" },
            ExtensionSurface { name: "CommandBarFlyout" },
            ExtensionSurface { name: "TreeView.Nodes" },
        ];

        const _: [(); PROJECTED_CONTROL_COUNT + PROJECTED_PROPERTY_COUNT + PROJECTED_EVENT_COUNT
            + CAPABILITY_PROPERTY_COUNT + STRUCTURAL_COUNT + EXTENSION_COUNT]
            = [(); SURFACE_CASES.len()];
        const _: [(); PROJECTED_PROPERTY_COUNT] = [(); PROJECTED_PROPERTIES.len()];
        const _: [(); PROJECTED_EVENT_COUNT] = [(); PROJECTED_EVENTS.len()];
        const _: [(); CAPABILITY_PROPERTY_COUNT] = [(); CAPABILITY_PROPERTIES.len()];
        const _: [(); STRUCTURAL_COUNT] = [(); STRUCTURAL_SURFACES.len()];
        const _: [(); EXTENSION_COUNT] = [(); EXTENSION_SURFACES.len()];
    }
    .to_string()
}

fn event_delivery_owner(control: &ResolvedControl, event: &ResolvedEvent) -> &'static str {
    match (control.name.as_str(), event.name.as_str()) {
        ("ToggleSwitch", "Toggled")
        | ("PasswordBox", "PasswordChanged")
        | ("Slider", "ValueChanged")
        | ("NumberBox", "ValueChanged")
        | ("ColorPicker", "ColorChanged")
        | ("ListView", "SelectionChanged")
        | ("CalendarDatePicker", "DateChanged")
        | ("TimePicker", "SelectedTimeChanged") => "live:Events_NativePayloadDelivery",
        ("CheckBox", "IsCheckedChanged") => "live:Events_ReplacementAndRevocation",
        ("ListBox", "SelectionChanged") => "live:Controlled_NativeFeedback",
        (
            "Border",
            "PointerEntered" | "PointerMoved" | "PointerPressed" | "PointerReleased"
            | "PointerExited",
        ) => "live:Pointer_RealInputGesture",
        _ => "registration+deterministic",
    }
}

#[allow(clippy::too_many_arguments)]
fn add_structural_case(
    control: &ResolvedControl,
    member: &str,
    cleared: proc_macro2::TokenStream,
    initial: proc_macro2::TokenStream,
    alternate: proc_macro2::TokenStream,
    builders: &mut Vec<proc_macro2::TokenStream>,
    cases: &mut Vec<proc_macro2::TokenStream>,
    inventory: &mut Vec<proc_macro2::TokenStream>,
) {
    let function = ident(&format!(
        "structural_{}_{}",
        to_snake_case(&control.name),
        to_snake_case(&member.replace('.', ""))
    ));
    builders.push(quote! {
        fn #function(stage: usize) -> View {
            match stage {
                0 | 3 => #cleared,
                1 => #initial,
                2 => #alternate,
                _ => unreachable!(),
            }
        }
    });
    let case_name = format!("structural.{}.{}", control.name, member);
    cases.push(quote! {
        SurfaceCase {
            name: #case_name,
            kind: SurfaceKind::Structural,
            stages: 4,
            subscription_delta: None,
            build: #function,
        }
    });
    let control_name = &control.name;
    inventory.push(quote! {
        StructuralSurface {
            control: #control_name,
            member: #member,
        }
    });
}

fn structural_child(
    target: SlotTarget,
    text: &'static str,
    alternate: bool,
) -> proc_macro2::TokenStream {
    match target {
        SlotTarget::UiElement | SlotTarget::Inspectable => {
            quote! { TextBlock::new().text(#text) }
        }
        SlotTarget::IconElement if alternate => {
            quote! { SymbolIcon::new().symbol(Symbol::Accept) }
        }
        SlotTarget::IconElement => quote! { SymbolIcon::new().symbol(Symbol::Add) },
    }
}

struct CapabilityPropertyCase {
    capability: &'static str,
    property: &'static str,
    cleared: proc_macro2::TokenStream,
    initial: proc_macro2::TokenStream,
    alternate: proc_macro2::TokenStream,
}

#[derive(Clone, Copy)]
enum CapabilityHost {
    Grid,
    RelativePanel,
    Canvas,
}

fn capability_property_cases() -> Vec<CapabilityPropertyCase> {
    let mut cases = Vec::new();
    for (property, method, initial, alternate) in [
        ("Width", "width", quote! { 40.0 }, quote! { 80.0 }),
        ("Height", "height", quote! { 30.0 }, quote! { 60.0 }),
        ("MinWidth", "min_width", quote! { 10.0 }, quote! { 20.0 }),
        ("MaxWidth", "max_width", quote! { 100.0 }, quote! { 200.0 }),
        ("MinHeight", "min_height", quote! { 10.0 }, quote! { 20.0 }),
        (
            "MaxHeight",
            "max_height",
            quote! { 100.0 },
            quote! { 200.0 },
        ),
        ("Opacity", "opacity", quote! { 0.25 }, quote! { 0.75 }),
    ] {
        let method = ident(method);
        cases.push(capability_child(
            "Layout",
            property,
            CapabilityHost::Grid,
            quote! { TextBlock::new().#method(#initial) },
            quote! { TextBlock::new().#method(#alternate) },
        ));
    }
    cases.extend([
        capability_child(
            "Layout",
            "HorizontalAlignment",
            CapabilityHost::Grid,
            quote! { TextBlock::new().horizontal_alignment(HorizontalAlignment::Left) },
            quote! { TextBlock::new().horizontal_alignment(HorizontalAlignment::Right) },
        ),
        capability_child(
            "Layout",
            "VerticalAlignment",
            CapabilityHost::Grid,
            quote! { TextBlock::new().vertical_alignment(VerticalAlignment::Top) },
            quote! { TextBlock::new().vertical_alignment(VerticalAlignment::Bottom) },
        ),
        capability_child(
            "Layout",
            "Margin",
            CapabilityHost::Grid,
            quote! { TextBlock::new().margin(Thickness::uniform(4.0)) },
            quote! { TextBlock::new().margin(Thickness::uniform(8.0)) },
        ),
    ]);
    for (property, method, initial, alternate) in [
        ("Row", "grid_row", 1, 2),
        ("Column", "grid_column", 1, 2),
        ("RowSpan", "grid_row_span", 2, 3),
        ("ColumnSpan", "grid_column_span", 2, 3),
    ] {
        let method = ident(method);
        cases.push(capability_child(
            "GridChild",
            property,
            CapabilityHost::Grid,
            quote! { TextBlock::new().#method(#initial) },
            quote! { TextBlock::new().#method(#alternate) },
        ));
    }
    for (property, method) in [
        ("AlignLeft", "relative_align_left"),
        ("AlignTop", "relative_align_top"),
        ("AlignRight", "relative_align_right"),
        ("AlignBottom", "relative_align_bottom"),
        ("AlignHorizontalCenter", "relative_align_horizontal_center"),
        ("AlignVerticalCenter", "relative_align_vertical_center"),
    ] {
        let method = ident(method);
        cases.push(capability_child(
            "RelativePanelChild",
            property,
            CapabilityHost::RelativePanel,
            quote! { TextBlock::new().#method() },
            quote! { TextBlock::new().#method() },
        ));
    }
    for (property, method) in [("Left", "canvas_left"), ("Top", "canvas_top")] {
        let method = ident(method);
        cases.push(capability_child(
            "CanvasChild",
            property,
            CapabilityHost::Canvas,
            quote! { TextBlock::new().#method(10.0) },
            quote! { TextBlock::new().#method(20.0) },
        ));
    }
    cases.extend([
        capability_child(
            "Automation",
            "Name",
            CapabilityHost::Grid,
            quote! { TextBlock::new().automation_name("surface a") },
            quote! { TextBlock::new().automation_name("surface b") },
        ),
        capability_child(
            "Automation",
            "Id",
            CapabilityHost::Grid,
            quote! { TextBlock::new().automation_id("surface-a") },
            quote! { TextBlock::new().automation_id("surface-b") },
        ),
        capability_child(
            "Automation",
            "HeadingLevel",
            CapabilityHost::Grid,
            quote! {
                TextBlock::new().automation_heading_level(AutomationHeadingLevel::Level1)
            },
            quote! {
                TextBlock::new().automation_heading_level(AutomationHeadingLevel::Level2)
            },
        ),
        capability_root(
            "GridDefinitions",
            "Rows",
            quote! { Grid::new().children((TextBlock::new(),)) },
            quote! {
                Grid::new().rows([GridLength::Pixel(20.0)]).children((TextBlock::new(),))
            },
            quote! {
                Grid::new().rows([GridLength::Auto, GridLength::STAR])
                    .children((TextBlock::new(),))
            },
        ),
        capability_root(
            "GridDefinitions",
            "Columns",
            quote! { Grid::new().children((TextBlock::new(),)) },
            quote! {
                Grid::new().columns([GridLength::Pixel(20.0)]).children((TextBlock::new(),))
            },
            quote! {
                Grid::new().columns([GridLength::Auto, GridLength::STAR])
                    .children((TextBlock::new(),))
            },
        ),
    ]);
    cases
}

fn capability_child(
    capability: &'static str,
    property: &'static str,
    host: CapabilityHost,
    initial: proc_macro2::TokenStream,
    alternate: proc_macro2::TokenStream,
) -> CapabilityPropertyCase {
    let wrap = |child| match host {
        CapabilityHost::Grid => quote! { Grid::new().children((#child,)) },
        CapabilityHost::RelativePanel => quote! { RelativePanel::new().children((#child,)) },
        CapabilityHost::Canvas => quote! { Canvas::new().children((#child,)) },
    };
    CapabilityPropertyCase {
        capability,
        property,
        cleared: wrap(quote! { TextBlock::new() }),
        initial: wrap(initial),
        alternate: wrap(alternate),
    }
}

fn capability_root(
    capability: &'static str,
    property: &'static str,
    cleared: proc_macro2::TokenStream,
    initial: proc_macro2::TokenStream,
    alternate: proc_macro2::TokenStream,
) -> CapabilityPropertyCase {
    CapabilityPropertyCase {
        capability,
        property,
        cleared,
        initial,
        alternate,
    }
}

fn event_callback(event: &ResolvedEvent, alternate: bool) -> proc_macro2::TokenStream {
    let marker = u8::from(alternate);
    if event.payload == "Unit" {
        quote! { move || { let _ = #marker; } }
    } else {
        quote! { move |_| { let _ = #marker; } }
    }
}

fn ident(value: &str) -> Ident {
    Ident::new(value, Span::call_site())
}

fn apply_property(
    control: &ResolvedControl,
    property: &ResolvedProperty,
    value: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let control_name = ident(&control.name);
    let field = ident(&property.field);
    let builder = if matches!(
        property.adapter,
        Some(PropertyAdapter::ImageUri | PropertyAdapter::Uri)
    ) {
        quote! { #control_name::new().#field(#value).unwrap() }
    } else {
        quote! { #control_name::new().#field(#value) }
    };
    wrap_control(control, builder)
}

fn wrap_control(
    control: &ResolvedControl,
    value: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match control.placement {
        ResolvedPlacement::Visual | ResolvedPlacement::Declaration => {
            quote! { Grid::new().children((#value,)) }
        }
        ResolvedPlacement::WindowLifetime if !control.slots.is_empty() => {
            let slot_type = ident(&format!("{}Slot", control.name));
            quote! {
                Grid::new().children((
                    (#value).slots(std::iter::empty::<SlotView<#slot_type>>()),
                ))
            }
        }
        ResolvedPlacement::WindowLifetime => quote! { Grid::new().children((#value,)) },
        ResolvedPlacement::TooltipAttachment => unreachable!(),
    }
}

fn wrap_structural(
    control: &ResolvedControl,
    value: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match control.placement {
        ResolvedPlacement::Visual => quote! { (#value).into() },
        ResolvedPlacement::WindowLifetime | ResolvedPlacement::Declaration => {
            quote! { Grid::new().children((#value,)) }
        }
        ResolvedPlacement::TooltipAttachment => unreachable!(),
    }
}

fn property_values(
    control: &ResolvedControl,
    property: &ResolvedProperty,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    if property.theme_style {
        return (
            quote! { Color::rgb(32, 64, 96) },
            quote! { Color::rgb(96, 64, 32) },
        );
    }
    if property.enum_variants.len() >= 2 {
        let value = ident(&property.value);
        let initial = ident(&property.enum_variants[0]);
        let alternate = ident(&property.enum_variants[1]);
        return (quote! { #value::#initial }, quote! { #value::#alternate });
    }

    match property.adapter {
        Some(PropertyAdapter::ClockIdentifier) => {
            (quote! { "12HourClock" }, quote! { "24HourClock" })
        }
        Some(PropertyAdapter::ImageUri) => (
            quote! { "ms-appx:///Files/surface-a.png" },
            quote! { "ms-appx:///Files/surface-b.png" },
        ),
        Some(PropertyAdapter::Uri) => (
            quote! { "https://example.com/a" },
            quote! { "https://example.com/b" },
        ),
        Some(PropertyAdapter::PathData) => (
            quote! { "M 0 0 L 10 10 Z" },
            quote! { "M 0 0 L 20 0 L 10 10 Z" },
        ),
        Some(PropertyAdapter::KeyAccelerators) => (
            quote! {
                KeyAccelerators::new([KeyAccelerator::new(
                    AcceleratorKey::R,
                    AcceleratorModifiers::Control,
                    || {},
                )])
            },
            quote! {
                KeyAccelerators::new([KeyAccelerator::new(
                    AcceleratorKey::Enter,
                    AcceleratorModifiers::None,
                    || {},
                )])
            },
        ),
        Some(PropertyAdapter::ResourceOverrides) => (
            quote! {
                ResourceOverrides::new()
                    .set("ButtonBackground", Color::rgb(32, 64, 96))
            },
            quote! {
                ResourceOverrides::new()
                    .set("ButtonForeground", Color::rgb(96, 64, 32))
            },
        ),
        Some(PropertyAdapter::DropPolicy) => (
            quote! {
                DragDropPolicy::new()
                    .text(DragDropAction::new(DragDropOperation::Copy))
            },
            quote! {
                DragDropPolicy::new()
                    .storage_items(DragDropAction::new(DragDropOperation::Move))
            },
        ),
        Some(PropertyAdapter::RichTextBlocks) => (
            quote! {
                RichText::single_paragraph([
                    RichTextInline::Run(RichTextRun::plain("surface a")),
                ])
            },
            quote! {
                RichText::single_paragraph([
                    RichTextInline::Run(RichTextRun::plain("surface b")),
                ])
            },
        ),
        _ => direct_property_values(control, property),
    }
}

fn direct_property_values(
    control: &ResolvedControl,
    property: &ResolvedProperty,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match property.value.as_str() {
        "Bool" => (quote! { true }, quote! { false }),
        "Brush" => (
            quote! { Color::rgb(32, 64, 96) },
            quote! { Color::rgb(96, 64, 32) },
        ),
        "Color" => (
            quote! { Color::rgb(32, 64, 96) },
            quote! { Color::rgb(96, 64, 32) },
        ),
        "CornerRadius" => (
            quote! { CornerRadius::uniform(2.0) },
            quote! { CornerRadius::uniform(4.0) },
        ),
        "Duration" => (
            quote! { std::time::Duration::from_millis(50) },
            quote! { std::time::Duration::from_millis(100) },
        ),
        "F64" => match property.validation {
            Some(ValueValidation::FinitePositive) => (quote! { 8.0 }, quote! { 16.0 }),
            Some(ValueValidation::FiniteNonNegative) => (quote! { 0.0 }, quote! { 8.0 }),
            _ => (quote! { 1.0 }, quote! { 2.0 }),
        },
        "FontWeight" => (quote! { FontWeight::NORMAL }, quote! { FontWeight::BOLD }),
        "HorizontalAlignment" => (
            quote! { HorizontalAlignment::Left },
            quote! { HorizontalAlignment::Right },
        ),
        "I32" => match property.validation {
            Some(ValueValidation::ZeroToFiftyNine) => (quote! { 5 }, quote! { 15 }),
            _ => (quote! { 1 }, quote! { 2 }),
        },
        "OptionalF64" if property.adapter == Some(PropertyAdapter::RatingValue) => {
            (quote! { Some(3.0) }, quote! { Some(4.0) })
        }
        "OptionalF64" => (quote! { Some(1.0) }, quote! { Some(2.0) }),
        "SelectionIndex" => (quote! { Some(0) }, quote! { None }),
        "Str" => (quote! { "surface a" }, quote! { "surface b" }),
        "StrList" => (
            quote! { ["surface a", "surface b"] },
            quote! { ["surface c", "surface d"] },
        ),
        "Thickness" => (
            quote! { Thickness::uniform(2.0) },
            quote! { Thickness::uniform(4.0) },
        ),
        "VerticalAlignment" => (
            quote! { VerticalAlignment::Top },
            quote! { VerticalAlignment::Bottom },
        ),
        value => panic!(
            "no surface values for {}.{} ({value}, {:?})",
            control.name, property.name, property.adapter
        ),
    }
}
