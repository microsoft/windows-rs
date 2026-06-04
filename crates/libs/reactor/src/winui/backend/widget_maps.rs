//! Typed property dispatch tables for the WinUI backend.
//!
//! Each control that has a typed map here bypasses the intermediate
//! `Prop`/`PropValue` enums entirely, calling WinUI COM setters directly
//! from widget struct fields. Controls not yet mapped fall through to the
//! legacy `bindings()` + `set_prop()` path.

use windows_collections;
use windows_core::Interface as _;

use crate::bindings as Xaml;
use crate::core::widgets::*;

use super::Handle;

// ─── Widget map definitions ──────────────────────────────────────────────────

pub(super) fn mount_text_block(w: &TextBlock, handle: &Handle) -> windows_core::Result<()> {
    let tb = handle.cast_inner::<Xaml::ITextBlock>()?;
    tb.put_Text(&w.content)?;
    if let Some(fs) = w.font_size {
        tb.put_FontSize(fs)?;
    }
    if let Some(fw) = w.font_weight {
        tb.put_FontWeight(Xaml::FontWeight { Weight: fw })?;
    }
    if w.wrap_text {
        tb.put_TextWrapping(Xaml::TextWrapping::Wrap)?;
    }
    if w.is_text_selection_enabled {
        tb.put_IsTextSelectionEnabled(true)?;
    }
    Ok(())
}

pub(super) fn diff_text_block(
    old: &TextBlock,
    new: &TextBlock,
    handle: &Handle,
) -> windows_core::Result<()> {
    let tb = handle.cast_inner::<Xaml::ITextBlock>()?;
    if old.content != new.content {
        tb.put_Text(&new.content)?;
    }
    if old.font_size != new.font_size {
        match new.font_size {
            Some(fs) => tb.put_FontSize(fs)?,
            None => tb.put_FontSize(14.0)?, // WinUI default
        }
    }
    if old.font_weight != new.font_weight {
        match new.font_weight {
            Some(fw) => tb.put_FontWeight(Xaml::FontWeight { Weight: fw })?,
            None => tb.put_FontWeight(Xaml::FontWeight { Weight: 400 })?,
        }
    }
    if old.wrap_text != new.wrap_text {
        tb.put_TextWrapping(if new.wrap_text {
            Xaml::TextWrapping::Wrap
        } else {
            Xaml::TextWrapping::NoWrap
        })?;
    }
    if old.is_text_selection_enabled != new.is_text_selection_enabled {
        tb.put_IsTextSelectionEnabled(new.is_text_selection_enabled)?;
    }
    Ok(())
}

pub(super) fn mount_stack_panel(w: &StackPanel, handle: &Handle) -> windows_core::Result<()> {
    let sp = handle.cast_inner::<Xaml::IStackPanel>()?;
    if let Some(spacing) = w.spacing {
        sp.put_Spacing(spacing)?;
    }
    // WinUI defaults to Vertical; always set explicitly since Rust default is horizontal
    sp.put_Orientation(if w.vertical {
        Xaml::Orientation::Vertical
    } else {
        Xaml::Orientation::Horizontal
    })?;
    Ok(())
}

pub(super) fn diff_stack_panel(
    old: &StackPanel,
    new: &StackPanel,
    handle: &Handle,
) -> windows_core::Result<()> {
    let sp = handle.cast_inner::<Xaml::IStackPanel>()?;
    if old.spacing != new.spacing {
        sp.put_Spacing(new.spacing.unwrap_or(0.0))?;
    }
    if old.vertical != new.vertical {
        sp.put_Orientation(if new.vertical {
            Xaml::Orientation::Vertical
        } else {
            Xaml::Orientation::Horizontal
        })?;
    }
    Ok(())
}

pub(super) fn mount_progress_bar(w: &ProgressBar, handle: &Handle) -> windows_core::Result<()> {
    let pb = handle.cast_inner::<Xaml::IProgressBar>()?;
    let rb = pb.cast::<Xaml::IRangeBase>()?;
    rb.put_Minimum(w.minimum)?;
    rb.put_Maximum(w.maximum)?;
    rb.put_Value(w.value)?;
    if w.is_indeterminate {
        pb.put_IsIndeterminate(true)?;
    }
    Ok(())
}

pub(super) fn diff_progress_bar(
    old: &ProgressBar,
    new: &ProgressBar,
    handle: &Handle,
) -> windows_core::Result<()> {
    let pb = handle.cast_inner::<Xaml::IProgressBar>()?;
    let rb = pb.cast::<Xaml::IRangeBase>()?;
    if old.minimum != new.minimum {
        rb.put_Minimum(new.minimum)?;
    }
    if old.maximum != new.maximum {
        rb.put_Maximum(new.maximum)?;
    }
    if old.value != new.value {
        rb.put_Value(new.value)?;
    }
    if old.is_indeterminate != new.is_indeterminate {
        pb.put_IsIndeterminate(new.is_indeterminate)?;
    }
    Ok(())
}

pub(super) fn mount_progress_ring(w: &ProgressRing, handle: &Handle) -> windows_core::Result<()> {
    let pr = handle.cast_inner::<Xaml::IProgressRing>()?;
    pr.put_Minimum(w.minimum)?;
    pr.put_Maximum(w.maximum)?;
    pr.cast::<Xaml::IRangeBase>()?.put_Value(w.value)?;
    if w.is_indeterminate {
        pr.put_IsIndeterminate(true)?;
    }
    if w.is_active {
        pr.put_IsActive(true)?;
    }
    Ok(())
}

pub(super) fn diff_progress_ring(
    old: &ProgressRing,
    new: &ProgressRing,
    handle: &Handle,
) -> windows_core::Result<()> {
    let pr = handle.cast_inner::<Xaml::IProgressRing>()?;
    if old.minimum != new.minimum {
        pr.put_Minimum(new.minimum)?;
    }
    if old.maximum != new.maximum {
        pr.put_Maximum(new.maximum)?;
    }
    if old.value != new.value {
        pr.cast::<Xaml::IRangeBase>()?.put_Value(new.value)?;
    }
    if old.is_indeterminate != new.is_indeterminate {
        pr.put_IsIndeterminate(new.is_indeterminate)?;
    }
    if old.is_active != new.is_active {
        pr.put_IsActive(new.is_active)?;
    }
    Ok(())
}

/// Dispatch table for typed mount. Returns `Some` if the widget was handled.
#[allow(clippy::manual_map)]
pub(super) fn try_mount(
    widget: &dyn std::any::Any,
    handle: &Handle,
) -> Option<windows_core::Result<()>> {
    if let Some(w) = widget.downcast_ref::<TextBlock>() {
        Some(mount_text_block(w, handle))
    } else if let Some(w) = widget.downcast_ref::<StackPanel>() {
        Some(mount_stack_panel(w, handle))
    } else if let Some(w) = widget.downcast_ref::<ProgressBar>() {
        Some(mount_progress_bar(w, handle))
    } else if let Some(w) = widget.downcast_ref::<ProgressRing>() {
        Some(mount_progress_ring(w, handle))
    } else if let Some(w) = widget.downcast_ref::<Border>() {
        Some(mount_border(w, handle))
    } else if let Some(w) = widget.downcast_ref::<Image>() {
        Some(mount_image(w, handle))
    } else if let Some(w) = widget.downcast_ref::<PersonPicture>() {
        Some(mount_person_picture(w, handle))
    } else if let Some(w) = widget.downcast_ref::<InfoBadge>() {
        Some(mount_info_badge(w, handle))
    } else if let Some(w) = widget.downcast_ref::<Viewbox>() {
        Some(mount_viewbox(w, handle))
    } else if let Some(w) = widget.downcast_ref::<ScrollViewer>() {
        Some(mount_scroll_viewer(w, handle))
    } else if let Some(w) = widget.downcast_ref::<Grid>() {
        Some(mount_grid(w, handle))
    } else {
        None
    }
}

/// Dispatch table for typed diff. Returns `Some` if the widget was handled.
pub(super) fn try_diff(
    old: &dyn std::any::Any,
    new: &dyn std::any::Any,
    handle: &Handle,
) -> Option<windows_core::Result<()>> {
    if let Some(old_w) = old.downcast_ref::<TextBlock>() {
        let new_w = new.downcast_ref::<TextBlock>().unwrap();
        Some(diff_text_block(old_w, new_w, handle))
    } else if let Some(old_w) = old.downcast_ref::<StackPanel>() {
        let new_w = new.downcast_ref::<StackPanel>().unwrap();
        Some(diff_stack_panel(old_w, new_w, handle))
    } else if let Some(old_w) = old.downcast_ref::<ProgressBar>() {
        let new_w = new.downcast_ref::<ProgressBar>().unwrap();
        Some(diff_progress_bar(old_w, new_w, handle))
    } else if let Some(old_w) = old.downcast_ref::<ProgressRing>() {
        let new_w = new.downcast_ref::<ProgressRing>().unwrap();
        Some(diff_progress_ring(old_w, new_w, handle))
    } else if let Some(old_w) = old.downcast_ref::<Border>() {
        let new_w = new.downcast_ref::<Border>().unwrap();
        Some(diff_border(old_w, new_w, handle))
    } else if let Some(old_w) = old.downcast_ref::<Image>() {
        let new_w = new.downcast_ref::<Image>().unwrap();
        Some(diff_image(old_w, new_w, handle))
    } else if let Some(old_w) = old.downcast_ref::<PersonPicture>() {
        let new_w = new.downcast_ref::<PersonPicture>().unwrap();
        Some(diff_person_picture(old_w, new_w, handle))
    } else if let Some(old_w) = old.downcast_ref::<InfoBadge>() {
        let new_w = new.downcast_ref::<InfoBadge>().unwrap();
        Some(diff_info_badge(old_w, new_w, handle))
    } else if let Some(old_w) = old.downcast_ref::<Viewbox>() {
        let new_w = new.downcast_ref::<Viewbox>().unwrap();
        Some(diff_viewbox(old_w, new_w, handle))
    } else if let Some(old_w) = old.downcast_ref::<ScrollViewer>() {
        let new_w = new.downcast_ref::<ScrollViewer>().unwrap();
        Some(diff_scroll_viewer(old_w, new_w, handle))
    } else if let Some(old_w) = old.downcast_ref::<Grid>() {
        let new_w = new.downcast_ref::<Grid>().unwrap();
        Some(diff_grid(old_w, new_w, handle))
    } else {
        None
    }
}

// ─── Border ──────────────────────────────────────────────────────────────────

pub(super) fn mount_border(w: &Border, handle: &Handle) -> windows_core::Result<()> {
    let b = handle.cast_inner::<Xaml::IBorder>()?;
    if let Some(cr) = w.corner_radius {
        b.put_CornerRadius(Xaml::CornerRadius {
            TopLeft: cr,
            TopRight: cr,
            BottomRight: cr,
            BottomLeft: cr,
        })?;
    }
    if let Some(BrushBinding::Direct(br)) = &w.border_brush {
        b.put_BorderBrush(&super::convert::brush_of(br)?)?;
    }
    if let Some(t) = w.border_thickness {
        b.put_BorderThickness(super::convert::to_xaml_thickness(t))?;
    }
    Ok(())
}

pub(super) fn diff_border(old: &Border, new: &Border, handle: &Handle) -> windows_core::Result<()> {
    let b = handle.cast_inner::<Xaml::IBorder>()?;
    if old.corner_radius != new.corner_radius {
        let cr = new.corner_radius.unwrap_or(0.0);
        b.put_CornerRadius(Xaml::CornerRadius {
            TopLeft: cr,
            TopRight: cr,
            BottomRight: cr,
            BottomLeft: cr,
        })?;
    }
    if old.border_brush != new.border_brush {
        match &new.border_brush {
            Some(BrushBinding::Direct(br)) => {
                b.put_BorderBrush(&super::convert::brush_of(br)?)?;
            }
            _ => b.put_BorderBrush(None)?,
        }
    }
    if old.border_thickness != new.border_thickness {
        let t = new.border_thickness.unwrap_or_default();
        b.put_BorderThickness(super::convert::to_xaml_thickness(t))?;
    }
    Ok(())
}

// ─── Image ───────────────────────────────────────────────────────────────────

fn map_stretch(s: ImageStretch) -> Xaml::Stretch {
    match s {
        ImageStretch::Uniform => Xaml::Stretch::Uniform,
        ImageStretch::UniformToFill => Xaml::Stretch::UniformToFill,
        ImageStretch::Fill => Xaml::Stretch::Fill,
        ImageStretch::None => Xaml::Stretch::None,
    }
}

pub(super) fn mount_image(w: &Image, handle: &Handle) -> windows_core::Result<()> {
    let img = handle.cast_inner::<Xaml::IImage>()?;
    if !w.source.is_empty() {
        let uri = Xaml::Uri::CreateUri(&w.source)?;
        let bmp = Xaml::BitmapImage::new()?;
        bmp.cast::<Xaml::IBitmapImage>()?.put_UriSource(&uri)?;
        img.put_Source(&bmp.cast::<Xaml::ImageSource>()?)?;
    }
    img.put_Stretch(map_stretch(w.stretch))?;
    Ok(())
}

pub(super) fn diff_image(old: &Image, new: &Image, handle: &Handle) -> windows_core::Result<()> {
    let img = handle.cast_inner::<Xaml::IImage>()?;
    if old.source != new.source {
        if new.source.is_empty() {
            img.put_Source(None)?;
        } else {
            let uri = Xaml::Uri::CreateUri(&new.source)?;
            let bmp = Xaml::BitmapImage::new()?;
            bmp.cast::<Xaml::IBitmapImage>()?.put_UriSource(&uri)?;
            img.put_Source(&bmp.cast::<Xaml::ImageSource>()?)?;
        }
    }
    if old.stretch != new.stretch {
        img.put_Stretch(map_stretch(new.stretch))?;
    }
    Ok(())
}

// ─── PersonPicture ───────────────────────────────────────────────────────────

pub(super) fn mount_person_picture(w: &PersonPicture, handle: &Handle) -> windows_core::Result<()> {
    let p = handle.cast_inner::<Xaml::IPersonPicture>()?;
    p.put_DisplayName(w.display_name.as_deref().unwrap_or_default())?;
    p.put_Initials(w.initials.as_deref().unwrap_or_default())?;
    Ok(())
}

pub(super) fn diff_person_picture(
    old: &PersonPicture,
    new: &PersonPicture,
    handle: &Handle,
) -> windows_core::Result<()> {
    let p = handle.cast_inner::<Xaml::IPersonPicture>()?;
    if old.display_name != new.display_name {
        p.put_DisplayName(new.display_name.as_deref().unwrap_or_default())?;
    }
    if old.initials != new.initials {
        p.put_Initials(new.initials.as_deref().unwrap_or_default())?;
    }
    Ok(())
}

// ─── InfoBadge ───────────────────────────────────────────────────────────────

pub(super) fn mount_info_badge(w: &InfoBadge, handle: &Handle) -> windows_core::Result<()> {
    let ib = handle.cast_inner::<Xaml::IInfoBadge>()?;
    ib.put_Value(w.value.unwrap_or(-1))?;
    Ok(())
}

pub(super) fn diff_info_badge(
    old: &InfoBadge,
    new: &InfoBadge,
    handle: &Handle,
) -> windows_core::Result<()> {
    if old.value != new.value {
        let ib = handle.cast_inner::<Xaml::IInfoBadge>()?;
        ib.put_Value(new.value.unwrap_or(-1))?;
    }
    Ok(())
}

// ─── Viewbox ─────────────────────────────────────────────────────────────────

pub(super) fn mount_viewbox(w: &Viewbox, handle: &Handle) -> windows_core::Result<()> {
    let vb = handle.cast_inner::<Xaml::IViewbox>()?;
    vb.put_Stretch(map_stretch(w.stretch))?;
    Ok(())
}

pub(super) fn diff_viewbox(
    old: &Viewbox,
    new: &Viewbox,
    handle: &Handle,
) -> windows_core::Result<()> {
    if old.stretch != new.stretch {
        let vb = handle.cast_inner::<Xaml::IViewbox>()?;
        vb.put_Stretch(map_stretch(new.stretch))?;
    }
    Ok(())
}

// ─── ScrollViewer ────────────────────────────────────────────────────────────

pub(super) fn mount_scroll_viewer(w: &ScrollViewer, handle: &Handle) -> windows_core::Result<()> {
    let sv = handle.cast_inner::<Xaml::IScrollViewer>()?;
    sv.put_HorizontalScrollBarVisibility(super::convert::to_xaml_scroll_visibility(
        w.horizontal_scroll_bar_visibility,
    ))?;
    sv.put_VerticalScrollBarVisibility(super::convert::to_xaml_scroll_visibility(
        w.vertical_scroll_bar_visibility,
    ))?;
    Ok(())
}

pub(super) fn diff_scroll_viewer(
    old: &ScrollViewer,
    new: &ScrollViewer,
    handle: &Handle,
) -> windows_core::Result<()> {
    let sv = handle.cast_inner::<Xaml::IScrollViewer>()?;
    if old.horizontal_scroll_bar_visibility != new.horizontal_scroll_bar_visibility {
        sv.put_HorizontalScrollBarVisibility(super::convert::to_xaml_scroll_visibility(
            new.horizontal_scroll_bar_visibility,
        ))?;
    }
    if old.vertical_scroll_bar_visibility != new.vertical_scroll_bar_visibility {
        sv.put_VerticalScrollBarVisibility(super::convert::to_xaml_scroll_visibility(
            new.vertical_scroll_bar_visibility,
        ))?;
    }
    Ok(())
}

// ─── Grid ────────────────────────────────────────────────────────────────────

pub(super) fn mount_grid(w: &Grid, handle: &Handle) -> windows_core::Result<()> {
    let g = handle.cast_inner::<Xaml::IGrid>()?;
    if !w.rows.is_empty() {
        let defs = g.get_RowDefinitions()?;
        for r in &w.rows {
            let rd = Xaml::RowDefinition::new()?;
            rd.cast::<Xaml::IRowDefinition>()?
                .put_Height(super::convert::to_xaml_gridlength(*r)?)?;
            defs.cast::<windows_collections::IVector<Xaml::RowDefinition>>()?
                .Append(&rd)?;
        }
    }
    if !w.columns.is_empty() {
        let defs = g.get_ColumnDefinitions()?;
        for c in &w.columns {
            let cd = Xaml::ColumnDefinition::new()?;
            cd.cast::<Xaml::IColumnDefinition>()?
                .put_Width(super::convert::to_xaml_gridlength(*c)?)?;
            defs.cast::<windows_collections::IVector<Xaml::ColumnDefinition>>()?
                .Append(&cd)?;
        }
    }
    if let Some(v) = w.row_spacing {
        g.put_RowSpacing(v)?;
    }
    if let Some(v) = w.column_spacing {
        g.put_ColumnSpacing(v)?;
    }
    Ok(())
}

pub(super) fn diff_grid(old: &Grid, new: &Grid, handle: &Handle) -> windows_core::Result<()> {
    let g = handle.cast_inner::<Xaml::IGrid>()?;
    if old.rows != new.rows {
        let defs = g.get_RowDefinitions()?;
        defs.cast::<windows_collections::IVector<Xaml::RowDefinition>>()?
            .Clear()?;
        for r in &new.rows {
            let rd = Xaml::RowDefinition::new()?;
            rd.cast::<Xaml::IRowDefinition>()?
                .put_Height(super::convert::to_xaml_gridlength(*r)?)?;
            defs.cast::<windows_collections::IVector<Xaml::RowDefinition>>()?
                .Append(&rd)?;
        }
    }
    if old.columns != new.columns {
        let defs = g.get_ColumnDefinitions()?;
        defs.cast::<windows_collections::IVector<Xaml::ColumnDefinition>>()?
            .Clear()?;
        for c in &new.columns {
            let cd = Xaml::ColumnDefinition::new()?;
            cd.cast::<Xaml::IColumnDefinition>()?
                .put_Width(super::convert::to_xaml_gridlength(*c)?)?;
            defs.cast::<windows_collections::IVector<Xaml::ColumnDefinition>>()?
                .Append(&cd)?;
        }
    }
    if old.row_spacing != new.row_spacing {
        g.put_RowSpacing(new.row_spacing.unwrap_or(0.0))?;
    }
    if old.column_spacing != new.column_spacing {
        g.put_ColumnSpacing(new.column_spacing.unwrap_or(0.0))?;
    }
    Ok(())
}
