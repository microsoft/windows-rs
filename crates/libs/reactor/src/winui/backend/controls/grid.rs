//! Typed handler for the `Grid` widget.

use super::EventCtx;
use crate::bindings as Xaml;
use crate::core::widgets::Grid;
use crate::winui::backend::Handle;
use crate::winui::backend::convert::to_xaml_gridlength;
use windows_core::Interface as _;

pub fn mount(w: &Grid, handle: &Handle, _ctx: &mut EventCtx) -> windows_core::Result<()> {
    let g = handle.cast_inner::<Xaml::IGrid>()?;
    if !w.rows.is_empty() {
        let defs = g.get_RowDefinitions()?;
        let vec = defs.cast::<windows_collections::IVector<Xaml::RowDefinition>>()?;
        for r in &w.rows {
            let rd = Xaml::RowDefinition::new()?;
            rd.cast::<Xaml::IRowDefinition>()?
                .put_Height(to_xaml_gridlength(*r)?)?;
            vec.Append(&rd)?;
        }
    }
    if !w.columns.is_empty() {
        let defs = g.get_ColumnDefinitions()?;
        let vec = defs.cast::<windows_collections::IVector<Xaml::ColumnDefinition>>()?;
        for c in &w.columns {
            let cd = Xaml::ColumnDefinition::new()?;
            cd.cast::<Xaml::IColumnDefinition>()?
                .put_Width(to_xaml_gridlength(*c)?)?;
            vec.Append(&cd)?;
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

pub fn diff(
    old: &Grid,
    new: &Grid,
    handle: &Handle,
    _ctx: &mut EventCtx,
) -> windows_core::Result<()> {
    let g = handle.cast_inner::<Xaml::IGrid>()?;

    if new.rows != old.rows {
        let defs = g.get_RowDefinitions()?;
        let vec = defs.cast::<windows_collections::IVector<Xaml::RowDefinition>>()?;
        vec.Clear()?;
        for r in &new.rows {
            let rd = Xaml::RowDefinition::new()?;
            rd.cast::<Xaml::IRowDefinition>()?
                .put_Height(to_xaml_gridlength(*r)?)?;
            vec.Append(&rd)?;
        }
    }
    if new.columns != old.columns {
        let defs = g.get_ColumnDefinitions()?;
        let vec = defs.cast::<windows_collections::IVector<Xaml::ColumnDefinition>>()?;
        vec.Clear()?;
        for c in &new.columns {
            let cd = Xaml::ColumnDefinition::new()?;
            cd.cast::<Xaml::IColumnDefinition>()?
                .put_Width(to_xaml_gridlength(*c)?)?;
            vec.Append(&cd)?;
        }
    }
    if new.row_spacing != old.row_spacing {
        g.put_RowSpacing(new.row_spacing.unwrap_or(0.0))?;
    }
    if new.column_spacing != old.column_spacing {
        g.put_ColumnSpacing(new.column_spacing.unwrap_or(0.0))?;
    }
    Ok(())
}
