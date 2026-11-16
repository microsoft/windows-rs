use super::*;

#[test]
fn typed_references_cover_all_imperative_controls() {
    let text_box = ElementRef::<TextBox>::new();
    let grid = ElementRef::<Grid>::new();
    let image = ElementRef::<Image>::new();
    let webview = ElementRef::<WebView2>::new();
    let swap_chain = ElementRef::<SwapChainPanel>::new();

    let _: View = TextBox::new().text("").element_ref(&text_box).into();
    let _: View = Grid::new().element_ref(&grid).into();
    let _: View = Image::new().element_ref(&image).into();
    let _: View = WebView2::new().element_ref(&webview).into();
    let _: View = SwapChainPanel::new().element_ref(&swap_chain).into();
}
