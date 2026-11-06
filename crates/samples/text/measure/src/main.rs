use windows_text::*;

const TEXT: &str = "Caf\u{e9} \u{1f600} wraps across lines in a narrow box.";

fn main() {
    let format = TextFormat::new("Segoe UI", 20.0).with_word_wrapping(WordWrapping::Wrap);
    let layout = TextLayout::new(TEXT, &format, 160.0, 400.0);

    let m = layout.metrics();
    println!(
        "{:.1} x {:.1} DIPs in {} lines",
        m.width, m.height, m.line_count
    );

    for (position, ch) in TEXT.char_indices().take(8) {
        let caret = layout.caret_bounds(position, false);
        println!(
            "{position:>3} {ch:?} caret at ({:.1}, {:.1})",
            caret.left, caret.top
        );
    }

    let hit = layout.hit_test_point(Vector2::new(m.width, m.height));
    println!("last line ends with {:?}", &TEXT[hit.text_range]);
}
