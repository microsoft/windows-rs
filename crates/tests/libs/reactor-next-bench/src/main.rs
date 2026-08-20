use windows_reactor_next::*;

fn main() {
    let tree = StackPanel::new().children(
        (0_u64..512)
            .map(|index| KeyedElement::new(index, TextBlock::new().text(index.to_string()))),
    );

    println!("constructed {} keyed elements", tree.child_elements().len());
}
