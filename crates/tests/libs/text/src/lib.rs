//! Tests for windows-text. DirectWrite needs no graphics device, so these run headless.

#[cfg(test)]
mod tests {
    use windows_text::*;

    fn layout(text: &str) -> TextLayout {
        let format = TextFormat::new("Segoe UI", 24.0).unwrap();
        TextLayout::new(text, &format, 400.0, 200.0).unwrap()
    }

    #[test]
    fn rect_from_xywh_and_accessors() {
        let rect = Rect::from_xywh(10.0, 20.0, 30.0, 40.0);
        assert_eq!(rect, Rect::new(10.0, 20.0, 40.0, 60.0));
        assert_eq!(rect.width(), 30.0);
        assert_eq!(rect.height(), 40.0);

        let moved = rect.offset(5.0, -8.0);
        assert_eq!(moved, Rect::new(15.0, 12.0, 45.0, 52.0));
    }

    #[test]
    fn text_layout_metrics() {
        let layout = layout("Hello, Canvas!");
        assert_eq!(layout.text(), "Hello, Canvas!");
        assert_eq!(layout.max_size(), (400.0, 200.0));

        let m = layout.metrics();
        assert_eq!(m.line_count, 1);
        assert!(m.width > 0.0);
        assert!(m.height > 0.0);
        assert!(m.width_including_trailing_whitespace >= m.width);
        assert_eq!(m.layout_width, 400.0);
        assert_eq!(m.layout_height, 200.0);
        assert_eq!(m.bounds().width(), m.width);
        assert_eq!(m.bounds().height(), m.height);
    }

    #[test]
    fn bold_is_wider_than_normal() {
        let normal = TextFormat::new("Segoe UI", 24.0).unwrap();
        let bold = TextFormat::new_bold("Segoe UI", 24.0).unwrap();
        let light = TextFormat::with_weight("Segoe UI", 24.0, FontWeight::LIGHT).unwrap();
        let width = |f| {
            TextLayout::new("Weight", f, 400.0, 200.0)
                .unwrap()
                .metrics()
                .width
        };
        assert!(width(&bold) > width(&normal));
        assert!(width(&normal) > width(&light));
    }

    #[test]
    fn alignment_moves_text() {
        let leading = TextFormat::new("Segoe UI", 24.0).unwrap();
        let centered = TextFormat::new("Segoe UI", 24.0)
            .unwrap()
            .with_alignment(TextAlignment::Center)
            .with_paragraph_alignment(ParagraphAlignment::Bottom);
        let leading = TextLayout::new("Hi", &leading, 400.0, 200.0).unwrap();
        let centered = TextLayout::new("Hi", &centered, 400.0, 200.0).unwrap();
        assert_eq!(leading.metrics().left, 0.0);
        assert!(centered.metrics().left > 100.0);
        assert!(centered.metrics().top > 100.0);
    }

    #[test]
    fn format_clones_are_independent() {
        let left = |format: &TextFormat| {
            TextLayout::new("Hi", format, 400.0, 200.0)
                .unwrap()
                .metrics()
                .left
        };

        let base = TextFormat::new("Segoe UI", 24.0).unwrap();
        let centered = base.clone().with_alignment(TextAlignment::Center);
        let trailing = centered.clone().with_alignment(TextAlignment::Trailing);
        assert_eq!(left(&base), 0.0);
        assert!(left(&centered) > 100.0);
        assert!(left(&trailing) > left(&centered));

        // A layout keeps the settings its format had when the layout was created.
        let layout = TextLayout::new("Hi", &base, 400.0, 200.0).unwrap();
        let _ = base.with_alignment(TextAlignment::Trailing);
        assert_eq!(layout.metrics().left, 0.0);
    }

    #[test]
    fn text_layout_wraps_when_narrow() {
        let format = TextFormat::new("Segoe UI", 24.0)
            .unwrap()
            .with_word_wrapping(WordWrapping::Wrap);

        let mut wide = TextLayout::new("word word word word word", &format, 1000.0, 500.0).unwrap();
        assert_eq!(wide.metrics().line_count, 1);

        let narrow = TextLayout::new("word word word word word", &format, 60.0, 500.0).unwrap();
        assert!(narrow.metrics().line_count > 1);

        wide.set_max_size(60.0, 500.0);
        assert!(wide.metrics().line_count > 1);
    }

    #[test]
    fn text_layout_no_wrap_stays_one_line() {
        let format = TextFormat::new("Segoe UI", 24.0)
            .unwrap()
            .with_word_wrapping(WordWrapping::NoWrap);
        let layout = TextLayout::new("word word word word word", &format, 40.0, 500.0).unwrap();
        assert_eq!(layout.metrics().line_count, 1);
    }

    #[test]
    fn hit_test_point_and_caret() {
        let layout = layout("Hello");

        let first = layout.hit_test_point(Vector2::new(0.0, 5.0));
        assert_eq!(first.text_position, 0);
        assert_eq!(first.length, 1);
        assert!(first.bounds.width() > 0.0);

        let past = layout.hit_test_point(Vector2::new(1000.0, 1000.0));
        assert!(!past.is_inside);
        assert!(past.is_trailing_hit);
        assert_eq!(past.text_position + past.length, 5);

        let caret0 = layout.caret_bounds(0, false);
        let caret3 = layout.caret_bounds(3, false);
        assert!(caret3.left > caret0.left);
        assert_eq!(caret0.width(), 0.0);
        assert!(caret0.height() > 0.0);

        let caret0_trailing = layout.caret_bounds(0, true);
        assert!(caret0_trailing.left > caret0.left);
        assert_eq!(caret0_trailing.left, layout.caret_bounds(1, false).left);
    }

    #[test]
    fn positions_are_utf8_offsets() {
        // 'a' is 1 byte, 'e' with acute is 2 bytes, the emoji is 4 bytes (a UTF-16 surrogate
        // pair), and 'b' is 1 byte.
        let text = "a\u{e9}\u{1f600}b";
        let layout = layout(text);

        let carets: Vec<f32> = [0, 1, 3, 7, 8]
            .into_iter()
            .map(|i| layout.caret_bounds(i, false).left)
            .collect();
        assert!(carets.windows(2).all(|w| w[1] > w[0]));
        assert_eq!(
            layout.caret_bounds(3, true).left,
            layout.caret_bounds(7, false).left
        );

        for (start, ch) in text.char_indices() {
            let caret = layout.caret_bounds(start, false);
            let hit = layout.hit_test_point(Vector2::new(caret.left + 1.0, caret.top + 1.0));
            assert_eq!(hit.text_position, start);
            assert_eq!(hit.length, ch.len_utf8());
            assert!(!hit.is_trailing_hit);
        }
    }

    #[test]
    #[should_panic]
    fn caret_bounds_rejects_non_boundary() {
        layout("\u{e9}").caret_bounds(1, false);
    }
}
