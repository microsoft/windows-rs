use super::*;

pub(super) fn symbol(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<i32> {
    let Handle::SymbolIcon(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a SymbolIcon");
    };
    Ok(value.Symbol()?.0)
}

pub(super) fn font(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<(String, String)> {
    let Handle::FontIcon(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a FontIcon");
    };
    Ok((value.Glyph()?, value.FontFamily()?.Source()?))
}

pub(super) fn bitmap(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<(String, bool)> {
    let Handle::BitmapIcon(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a BitmapIcon");
    };
    Ok((value.UriSource()?.AbsoluteUri()?, value.ShowAsMonochrome()?))
}

pub(super) fn image_size(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<(f64, f64)> {
    let Handle::ImageIcon(value) = &runtime.node(id)?.handle else {
        panic!("native node is not an ImageIcon");
    };
    let framework: bindings::FrameworkElement = value.cast()?;
    Ok((framework.MaxWidth()?, framework.MaxHeight()?))
}

pub(super) fn path_identity(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<(usize, usize)> {
    let Handle::PathIcon(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a PathIcon");
    };
    let data = value.Data()?;
    Ok((value.as_raw() as usize, data.as_raw() as usize))
}
