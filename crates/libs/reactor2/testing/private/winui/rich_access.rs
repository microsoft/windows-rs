use super::*;

pub(in crate::winui) fn rich_edit_box(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(String, bool)> {
    let Handle::RichEditBox(state) = &runtime.node(id)?.handle else {
        panic!("native node is not a RichEditBox");
    };
    let mut text = windows_core::HSTRING::new();
    state
        .value
        .Document()?
        .GetText(bindings::TextGetOptions::None, &mut text)?;
    let mut text = text.to_string_lossy();
    if text.ends_with('\r') {
        text.pop();
    }
    Ok((text, state.value.IsReadOnly()?))
}

pub(in crate::winui) fn rich_text_block(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<(f64, bool, bool, Vec<Vec<(String, bool, bool)>>)> {
    let Handle::RichTextBlock(value) = &runtime.node(id)?.handle else {
        panic!("native node is not a RichTextBlock");
    };
    let mut paragraphs = Vec::new();
    let blocks = value.Blocks()?;
    for index in 0..blocks.Size()? {
        let paragraph = blocks.GetAt(index)?.cast::<bindings::Paragraph>()?;
        let inlines = paragraph.Inlines()?;
        let mut runs = Vec::new();
        for inline_index in 0..inlines.Size()? {
            let run = inlines.GetAt(inline_index)?.cast::<bindings::Run>()?;
            let element: bindings::ITextElement = run.cast()?;
            runs.push((
                run.Text()?,
                element.FontWeight()?.weight >= 700,
                element.FontStyle()? == bindings::FontStyle::Italic,
            ));
        }
        paragraphs.push(runs);
    }
    Ok((
        value.FontSize()?,
        value.IsTextSelectionEnabled()?,
        value.TextWrapping()? == bindings::TextWrapping::Wrap,
        paragraphs,
    ))
}

pub(in crate::winui) fn tree_node_identity(
    runtime: &WinUiRuntime,
    id: NodeId,
    key: u64,
) -> WindowsResult<usize> {
    let Handle::TreeView(state) = &runtime.node(id)?.handle else {
        panic!("native node is not a TreeView");
    };
    Ok(Interface::as_raw(state.nodes.borrow().get(&key).unwrap()) as usize)
}

pub(in crate::winui) fn tree_node_expanded(
    runtime: &WinUiRuntime,
    id: NodeId,
    key: u64,
) -> WindowsResult<bool> {
    let Handle::TreeView(state) = &runtime.node(id)?.handle else {
        panic!("native node is not a TreeView");
    };
    state.nodes.borrow().get(&key).unwrap().IsExpanded()
}
