use super::*;
use crate::element::Icon;
use crate::resources::ElementResources;
use std::rc::Rc;

impl<R: NativeRuntime> Engine<R> {
    pub(crate) fn queue_control_update(
        &mut self,
        id: NodeId,
        update: ControlUpdate,
    ) -> Result<(), EngineError> {
        self.queue_update(id, NativeUpdate::Control(update))
    }

    pub(crate) fn queue_framework_update(
        &mut self,
        id: NodeId,
        update: FrameworkUpdate,
    ) -> Result<(), EngineError> {
        self.queue_update(id, NativeUpdate::Framework(update))
    }

    pub(crate) fn queue_attached_update(
        &mut self,
        id: NodeId,
        update: AttachedUpdate,
    ) -> Result<(), EngineError> {
        self.queue_update(id, NativeUpdate::Attached(update))
    }

    pub(crate) fn queue_visual_update(
        &mut self,
        id: NodeId,
        update: VisualUpdate,
    ) -> Result<(), EngineError> {
        self.queue_update(id, NativeUpdate::Visual(update))
    }

    pub(crate) fn queue_accessibility_update(
        &mut self,
        id: NodeId,
        update: AccessibilityUpdate,
    ) -> Result<(), EngineError> {
        self.queue_update(id, NativeUpdate::Accessibility(update))
    }

    pub(crate) fn queue_text_style_update(
        &mut self,
        id: NodeId,
        update: TextStyleUpdate,
    ) -> Result<(), EngineError> {
        self.queue_update(id, NativeUpdate::TextStyle(update))
    }

    pub(crate) fn queue_input_update(
        &mut self,
        id: NodeId,
        update: InputUpdate,
    ) -> Result<(), EngineError> {
        self.queue_update(id, NativeUpdate::Input(update))
    }

    pub(crate) fn queue_resources_update(
        &mut self,
        id: NodeId,
        resources: ElementResources,
    ) -> Result<(), EngineError> {
        self.queue_update(id, NativeUpdate::Resources(Box::new(resources)))
    }

    pub(crate) fn set_selector_bar_item_icon(
        &mut self,
        id: NodeId,
        value: Option<Icon>,
    ) -> Result<(), EngineError> {
        self.queue_update(
            id,
            NativeUpdate::Control(ControlUpdate::SelectorBarItem(SelectorBarItemUpdate::Icon(
                value.map(Rc::new),
            ))),
        )
    }

    pub fn fade_to(
        &mut self,
        id: NodeId,
        opacity: f32,
        duration: std::time::Duration,
    ) -> Result<(), EngineError> {
        self.queue_update(
            id,
            NativeUpdate::Visual(VisualUpdate::FadeTo { opacity, duration }),
        )
    }

    pub fn set_text(&mut self, id: NodeId, text: impl Into<String>) -> Result<(), EngineError> {
        self.ensure_native(id)?;
        let update = match self.arena.get(id).unwrap().native_kind.unwrap() {
            NativeKind::TextBlock => ControlUpdate::TextBlockText(text.into()),
            NativeKind::TextBox => {
                ControlUpdate::TextBox(Box::new(TextBoxUpdate::Text(text.into())))
            }
            kind => {
                return Err(EngineError::UnsupportedCommand {
                    id,
                    kind,
                    command: "set text",
                });
            }
        };
        self.queue_update(id, NativeUpdate::Control(update))
    }

    pub fn set_grid_columns(
        &mut self,
        id: NodeId,
        values: Vec<GridLength>,
    ) -> Result<(), EngineError> {
        self.queue_update(
            id,
            NativeUpdate::Control(ControlUpdate::Grid(GridUpdate::Columns(
                values.into_boxed_slice(),
            ))),
        )
    }

    pub fn set_grid_rows(
        &mut self,
        id: NodeId,
        values: Vec<GridLength>,
    ) -> Result<(), EngineError> {
        self.queue_update(
            id,
            NativeUpdate::Control(ControlUpdate::Grid(GridUpdate::Rows(
                values.into_boxed_slice(),
            ))),
        )
    }

    pub fn set_padding(&mut self, id: NodeId, value: Option<Thickness>) -> Result<(), EngineError> {
        let value = value.unwrap_or(Thickness {
            left: f64::NAN,
            top: f64::NAN,
            right: f64::NAN,
            bottom: f64::NAN,
        });
        self.queue_update(id, NativeUpdate::Framework(FrameworkUpdate::Padding(value)))
    }

    pub fn set_password(
        &mut self,
        id: NodeId,
        password: impl Into<String>,
    ) -> Result<(), EngineError> {
        self.queue_update(
            id,
            NativeUpdate::Control(ControlUpdate::PasswordBox(Box::new(
                PasswordBoxUpdate::Password(password.into()),
            ))),
        )
    }

    pub fn set_virtual_item_count(&mut self, id: NodeId, count: usize) -> Result<(), EngineError> {
        let update = NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::ItemCount(
            count,
        )));
        self.set_virtual_items(id, count, update)
    }

    pub fn set_virtual_item_keys(
        &mut self,
        id: NodeId,
        keys: Rc<[u64]>,
    ) -> Result<(), EngineError> {
        let count = keys.len();
        let update =
            NativeUpdate::Control(ControlUpdate::Collection(CollectionUpdate::ItemKeys(keys)));
        self.set_virtual_items(id, count, update)
    }

    fn set_virtual_items(
        &mut self,
        id: NodeId,
        count: usize,
        update: NativeUpdate,
    ) -> Result<(), EngineError> {
        self.ensure_update(id, &update)?;
        let stale = match &self.arena.get(id).ok_or(EngineError::InvalidNode(id))?.kind {
            NodeKind::VirtualHost { realized } => realized
                .range(count..)
                .map(|(_, row)| row.root)
                .collect::<Vec<_>>(),
            _ => {
                return Err(EngineError::InvalidNode(id));
            }
        };
        for row in stale {
            self.remove_subtree(row)?;
        }
        self.pending.push(Command::Update { id, update });
        Ok(())
    }

    #[cfg(feature = "canvas")]
    pub(crate) fn update_canvas_image(
        &mut self,
        id: NodeId,
        draw: crate::canvas::CanvasDrawCallback,
        invalidation_revision: u64,
        invalidation_source_changed: bool,
    ) -> Result<(), EngineError> {
        let update = if invalidation_source_changed {
            CanvasUpdate::Rebind {
                draw,
                invalidation_revision,
            }
        } else {
            CanvasUpdate::Props {
                draw,
                invalidation_revision,
            }
        };
        self.queue_update(
            id,
            NativeUpdate::Control(ControlUpdate::CanvasImage(update)),
        )
    }

    #[cfg(feature = "canvas")]
    pub(crate) fn update_swap_chain_canvas(
        &mut self,
        id: NodeId,
        draw: crate::canvas::CanvasDrawCallback,
        invalidation_revision: u64,
        invalidation_source_changed: bool,
    ) -> Result<(), EngineError> {
        let update = if invalidation_source_changed {
            CanvasUpdate::Rebind {
                draw,
                invalidation_revision,
            }
        } else {
            CanvasUpdate::Props {
                draw,
                invalidation_revision,
            }
        };
        self.queue_update(
            id,
            NativeUpdate::Control(ControlUpdate::SwapChainCanvas(Box::new(
                SwapChainCanvasUpdate::Canvas(update),
            ))),
        )
    }

    #[cfg(feature = "canvas")]
    pub(crate) fn invalidate_canvas(
        &mut self,
        id: NodeId,
        revision: u64,
    ) -> Result<(), EngineError> {
        let update = match self.arena.get(id).and_then(|node| node.native_kind) {
            Some(NativeKind::CanvasImage) => {
                ControlUpdate::CanvasImage(CanvasUpdate::Invalidate(revision))
            }
            Some(NativeKind::SwapChainCanvas) => ControlUpdate::SwapChainCanvas(Box::new(
                SwapChainCanvasUpdate::Canvas(CanvasUpdate::Invalidate(revision)),
            )),
            _ => return Err(EngineError::InvalidNode(id)),
        };
        self.queue_update(id, NativeUpdate::Control(update))
    }

    pub(crate) fn run_composition_action(
        &mut self,
        id: NodeId,
        action: crate::composition::CompositionAction,
    ) -> Result<(), EngineError> {
        let Some(MountedKind::CompositionHost(props)) = self
            .arena
            .get(id)
            .and_then(|node| node.mounted.as_ref())
            .map(|mounted| &mounted.kind)
        else {
            return Err(EngineError::InvalidNode(id));
        };
        if props.factory.state_type() != action.state_type() {
            return Err(EngineError::InvalidNode(id));
        }
        self.queue_update(
            id,
            NativeUpdate::Control(ControlUpdate::CompositionHost(Box::new(
                CompositionHostUpdate::Action(action),
            ))),
        )
    }

    #[cfg(feature = "canvas")]
    pub(crate) fn run_swap_chain_host_action(
        &mut self,
        id: NodeId,
        action: crate::canvas::SwapChainHostAction,
    ) -> Result<(), EngineError> {
        let Some(MountedKind::SwapChainHost(props)) = self
            .arena
            .get(id)
            .and_then(|node| node.mounted.as_ref())
            .map(|mounted| &mounted.kind)
        else {
            return Err(EngineError::InvalidNode(id));
        };
        if action
            .state_type()
            .is_some_and(|state_type| state_type != props.factory.state_type())
        {
            return Err(EngineError::InvalidNode(id));
        }
        self.queue_update(
            id,
            NativeUpdate::Control(ControlUpdate::SwapChainHost(Box::new(
                SwapChainHostUpdate::Action(action),
            ))),
        )
    }

    #[cfg(feature = "webview")]
    pub(crate) fn run_webview_action(
        &mut self,
        id: NodeId,
        action: crate::webview::WebViewAction,
    ) -> Result<(), EngineError> {
        if !matches!(
            self.arena
                .get(id)
                .and_then(|node| node.mounted.as_ref())
                .map(|mounted| &mounted.kind),
            Some(MountedKind::WebViewHost(_))
        ) {
            return Err(EngineError::InvalidNode(id));
        }
        self.queue_update(
            id,
            NativeUpdate::Control(ControlUpdate::WebViewHost(WebViewHostUpdate::Action(
                action,
            ))),
        )
    }

    pub fn set_width(&mut self, id: NodeId, value: Option<f64>) -> Result<(), EngineError> {
        self.set_dimension(id, FrameworkUpdate::Width, value)
    }

    pub fn set_height(&mut self, id: NodeId, value: Option<f64>) -> Result<(), EngineError> {
        self.set_dimension(id, FrameworkUpdate::Height, value)
    }

    pub fn set_min_width(&mut self, id: NodeId, value: Option<f64>) -> Result<(), EngineError> {
        self.set_dimension(id, FrameworkUpdate::MinWidth, value)
    }

    pub fn set_max_width(&mut self, id: NodeId, value: Option<f64>) -> Result<(), EngineError> {
        self.set_dimension(id, FrameworkUpdate::MaxWidth, value)
    }

    pub fn set_min_height(&mut self, id: NodeId, value: Option<f64>) -> Result<(), EngineError> {
        self.set_dimension(id, FrameworkUpdate::MinHeight, value)
    }

    pub fn set_max_height(&mut self, id: NodeId, value: Option<f64>) -> Result<(), EngineError> {
        self.set_dimension(id, FrameworkUpdate::MaxHeight, value)
    }

    pub(crate) fn attached_target(&self, edge: NodeId) -> Result<NodeId, EngineError> {
        let roots = self.projected_native_roots(edge);
        if roots.len() != 1 {
            return Err(EngineError::AttachedChildNativeRootCount {
                edge,
                count: roots.len(),
            });
        }
        Ok(roots[0])
    }

    pub(crate) fn set_attached_placement(
        &mut self,
        id: NodeId,
        old: AttachedPlacement,
        new: AttachedPlacement,
    ) -> Result<(), EngineError> {
        match (old, new) {
            (AttachedPlacement::Grid(old), AttachedPlacement::Grid(new)) => {
                self.set_grid_placement(id, old, new)
            }
            (AttachedPlacement::Canvas(old), AttachedPlacement::Canvas(new)) => {
                self.set_canvas_placement(id, old, new)
            }
            (AttachedPlacement::RelativePanel(old), AttachedPlacement::RelativePanel(new)) => {
                self.set_relative_panel_placement(id, old, new)
            }
            _ => unreachable!(),
        }
    }

    fn set_grid_placement(
        &mut self,
        id: NodeId,
        old: GridPlacement,
        new: GridPlacement,
    ) -> Result<(), EngineError> {
        for (changed, update) in [
            (old.row() != new.row(), AttachedUpdate::Row(new.row())),
            (
                old.column() != new.column(),
                AttachedUpdate::Column(new.column()),
            ),
            (
                old.row_span() != new.row_span(),
                AttachedUpdate::RowSpan(new.row_span()),
            ),
            (
                old.column_span() != new.column_span(),
                AttachedUpdate::ColumnSpan(new.column_span()),
            ),
        ] {
            if changed {
                self.queue_update(id, NativeUpdate::Attached(update))?;
            }
        }
        Ok(())
    }

    fn set_canvas_placement(
        &mut self,
        id: NodeId,
        old: CanvasPlacement,
        new: CanvasPlacement,
    ) -> Result<(), EngineError> {
        for (changed, update) in [
            (
                !same_optional_f64(old.left(), new.left()),
                AttachedUpdate::CanvasLeft(new.left()),
            ),
            (
                !same_optional_f64(old.top(), new.top()),
                AttachedUpdate::CanvasTop(new.top()),
            ),
            (
                old.z_index() != new.z_index(),
                AttachedUpdate::CanvasZIndex(new.z_index()),
            ),
        ] {
            if changed {
                self.queue_update(id, NativeUpdate::Attached(update))?;
            }
        }
        Ok(())
    }

    fn set_relative_panel_placement(
        &mut self,
        id: NodeId,
        old: RelativePanelPlacement,
        new: RelativePanelPlacement,
    ) -> Result<(), EngineError> {
        for (changed, update) in [
            (
                old.align_left() != new.align_left(),
                AttachedUpdate::RelativeAlignLeft(new.align_left()),
            ),
            (
                old.align_right() != new.align_right(),
                AttachedUpdate::RelativeAlignRight(new.align_right()),
            ),
            (
                old.align_top() != new.align_top(),
                AttachedUpdate::RelativeAlignTop(new.align_top()),
            ),
            (
                old.align_bottom() != new.align_bottom(),
                AttachedUpdate::RelativeAlignBottom(new.align_bottom()),
            ),
            (
                old.align_horizontal_center() != new.align_horizontal_center(),
                AttachedUpdate::RelativeAlignHorizontalCenter(new.align_horizontal_center()),
            ),
            (
                old.align_vertical_center() != new.align_vertical_center(),
                AttachedUpdate::RelativeAlignVerticalCenter(new.align_vertical_center()),
            ),
        ] {
            if changed {
                self.queue_update(id, NativeUpdate::Attached(update))?;
            }
        }
        Ok(())
    }

    fn set_dimension(
        &mut self,
        id: NodeId,
        update: fn(Dimension) -> FrameworkUpdate,
        value: Option<f64>,
    ) -> Result<(), EngineError> {
        self.queue_update(
            id,
            NativeUpdate::Framework(update(value.map_or(Dimension::Default, Dimension::Pixels))),
        )
    }

    pub fn set_automation_name(
        &mut self,
        id: NodeId,
        value: Option<String>,
    ) -> Result<(), EngineError> {
        self.queue_update(
            id,
            NativeUpdate::Accessibility(AccessibilityUpdate::AutomationName(
                value.unwrap_or_default(),
            )),
        )
    }

    pub fn set_automation_id(
        &mut self,
        id: NodeId,
        value: Option<String>,
    ) -> Result<(), EngineError> {
        self.queue_update(
            id,
            NativeUpdate::Accessibility(AccessibilityUpdate::AutomationId(
                value.unwrap_or_default(),
            )),
        )
    }

    pub fn set_help_text(&mut self, id: NodeId, value: Option<String>) -> Result<(), EngineError> {
        self.queue_update(
            id,
            NativeUpdate::Accessibility(AccessibilityUpdate::HelpText(value.unwrap_or_default())),
        )
    }

    pub fn set_rating_caption(
        &mut self,
        id: NodeId,
        value: impl Into<String>,
    ) -> Result<(), EngineError> {
        self.queue_update(
            id,
            NativeUpdate::Control(ControlUpdate::RatingControl(RatingControlUpdate::Caption(
                value.into(),
            ))),
        )
    }

    pub fn set_hyperlink_button_navigate_uri(
        &mut self,
        id: NodeId,
        value: Option<&str>,
    ) -> Result<(), EngineError> {
        self.queue_update(
            id,
            NativeUpdate::Control(ControlUpdate::HyperlinkButtonNavigateUri(
                value.map(str::to_string),
            )),
        )
    }

    fn ensure_native(&self, id: NodeId) -> Result<(), EngineError> {
        let node = self.arena.get(id).ok_or(EngineError::InvalidNode(id))?;
        if node.kind.is_native() {
            Ok(())
        } else {
            Err(EngineError::InvalidNode(id))
        }
    }

    fn ensure_update(&self, id: NodeId, update: &NativeUpdate) -> Result<(), EngineError> {
        self.ensure_native(id)?;
        let kind = self.arena.get(id).unwrap().native_kind.unwrap();
        if update.supports(kind) {
            Ok(())
        } else {
            Err(EngineError::UnsupportedCommand {
                id,
                kind,
                command: update.name(),
            })
        }
    }

    fn queue_update(&mut self, id: NodeId, update: NativeUpdate) -> Result<(), EngineError> {
        self.ensure_update(id, &update)?;
        self.pending.push(Command::Update { id, update });
        Ok(())
    }
}
