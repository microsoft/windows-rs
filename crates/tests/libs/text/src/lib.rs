//! Tests for windows-text. DirectWrite needs no graphics device, so these run headless.

#[cfg(test)]
mod tests {
    use windows_text::*;

    fn layout(text: &str) -> TextLayout {
        let format = TextFormat::new("Segoe UI", 24.0).unwrap();
        TextLayout::new(text, &format, 400.0, 200.0).unwrap()
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
        let light = TextFormat::new("Segoe UI", 24.0)
            .unwrap()
            .with_weight(FontWeight::LIGHT);
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
        // 'a' is 1 byte, 'e' with acute is 2 bytes, the euro sign is 3 bytes, the emoji is 4 bytes
        // (a UTF-16 surrogate pair), and 'b' is 1 byte.
        let text = "a\u{e9}\u{20ac}\u{1f600}b";
        let layout = layout(text);

        let carets: Vec<f32> = [0, 1, 3, 6, 10, 11]
            .into_iter()
            .map(|i| layout.caret_bounds(i, false).left)
            .collect();
        assert!(carets.windows(2).all(|w| w[1] > w[0]));
        assert_eq!(
            layout.caret_bounds(6, true).left,
            layout.caret_bounds(10, false).left
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
    fn segoe() -> FontFace {
        FontCollection::system()
            .find("Segoe UI", FontWeight::NORMAL, FontStyle::Normal)
            .unwrap()
    }

    #[test]
    fn system_collection_finds_faces() {
        let system = FontCollection::system();
        let names = system.family_names();
        assert!(names.iter().any(|name| name == "Segoe UI"));
        assert!(
            system
                .find("No Such Family", FontWeight::NORMAL, FontStyle::Normal)
                .is_none()
        );

        let bold = system
            .find("segoe ui", FontWeight::BOLD, FontStyle::Italic)
            .unwrap();
        assert_eq!(bold.family_name(), "Segoe UI");
        assert_eq!(bold.weight(), FontWeight::BOLD);
        assert_eq!(bold.style(), FontStyle::Italic);
        assert!(!bold.is_color_font());

        // DirectWrite reuses face objects, so a face can key a cache.
        assert_eq!(segoe(), segoe());
        assert_ne!(segoe(), bold);
        assert!(
            system
                .find("Segoe UI Emoji", FontWeight::NORMAL, FontStyle::Normal)
                .unwrap()
                .is_color_font()
        );
    }

    #[test]
    fn font_and_glyph_metrics() {
        let face = segoe();
        let metrics = face.metrics();
        assert_eq!(metrics.design_units_per_em, 2048);
        assert!(metrics.ascent > metrics.cap_height);
        assert!(metrics.cap_height > metrics.x_height);
        assert!(metrics.glyph_box_bottom < 0);

        let w = face.glyph_index('W').unwrap();
        let i = face.glyph_index('i').unwrap();
        assert!(face.glyph_metrics(w).advance_width > face.glyph_metrics(i).advance_width);
        assert_eq!(face.glyph_index('\u{10ffff}'), None);
    }

    #[test]
    fn collection_from_data() {
        let data = std::fs::read(std::env::var("WINDIR").unwrap() + r"\Fonts\consola.ttf").unwrap();
        let collection = FontCollection::from_data([data.as_slice()]).unwrap();
        assert_eq!(collection.family_names(), ["Consolas"]);
        assert!(
            collection
                .find("Segoe UI", FontWeight::NORMAL, FontStyle::Normal)
                .is_none()
        );

        // A format that uses the collection lays out with its font: Consolas is monospaced.
        let format = TextFormat::new("Consolas", 24.0)
            .unwrap()
            .with_collection(&collection);
        let width = |text| {
            TextLayout::new(text, &format, 400.0, 200.0)
                .unwrap()
                .metrics()
                .width
        };
        assert_eq!(width("iiii"), width("WWWW"));

        // A collection without Consolas cannot supply it, so the text is no longer monospaced.
        let data = std::fs::read(std::env::var("WINDIR").unwrap() + r"\Fonts\segoeui.ttf").unwrap();
        let collection = FontCollection::from_data([data.as_slice()]).unwrap();
        let format = format.with_collection(&collection);
        let width = |text| {
            TextLayout::new(text, &format, 400.0, 200.0)
                .unwrap()
                .metrics()
                .width
        };
        assert!(width("iiii") < width("WWWW") / 2.0);

        assert!(FontCollection::from_data([b"not a font".as_slice()]).is_err());
    }

    #[test]
    fn format_style_and_locale() {
        let width = |format: &TextFormat| {
            TextLayout::new("Slanted text", format, 400.0, 200.0)
                .unwrap()
                .metrics()
                .width
        };
        let normal = TextFormat::new("Segoe UI", 24.0).unwrap();
        let italic = normal.clone().with_style(FontStyle::Italic);
        assert_ne!(width(&normal), width(&italic));

        // Changing a setting fixed at creation keeps the other settings.
        let centered = normal
            .with_alignment(TextAlignment::Center)
            .with_locale("en-gb")
            .with_style(FontStyle::Italic);
        let layout = TextLayout::new("Hi", &centered, 400.0, 200.0).unwrap();
        assert!(layout.metrics().left > 100.0);
    }

    #[test]
    fn fallback_selects_preferred_family() {
        const SNOWMAN: char = '\u{2603}';
        let system = FontCollection::system();
        let face = |family| {
            system
                .find(family, FontWeight::NORMAL, FontStyle::Normal)
                .unwrap()
        };
        assert_eq!(face("Consolas").glyph_index(SNOWMAN), None);

        let width = |fallback: Option<&FontFallback>| {
            let mut format = TextFormat::new("Consolas", 100.0).unwrap();
            if let Some(fallback) = fallback {
                format = format.with_fallback(fallback);
            }
            TextLayout::new(&SNOWMAN.to_string(), &format, 1000.0, 1000.0)
                .unwrap()
                .metrics()
                .width
        };
        let advance = |family| {
            let face = face(family);
            let glyph = face.glyph_index(SNOWMAN).unwrap();
            face.glyph_metrics(glyph).advance_width as f32 * 100.0
                / face.metrics().design_units_per_em as f32
        };

        // The character takes the advance of the first listed family that has it.
        for family in ["Segoe UI Symbol", "Segoe UI Emoji"] {
            let fallback = FontFallback::new(&system, &["No Such Family", family]);
            assert!(
                (width(Some(&fallback)) - advance(family)).abs() < 0.01,
                "{family}"
            );
        }
        assert_ne!(advance("Segoe UI Symbol"), advance("Segoe UI Emoji"));

        // Families without the character leave it to the system fallback.
        let fallback = FontFallback::new(&system, &["Consolas"]);
        assert_eq!(width(Some(&fallback)), width(None));
    }

    fn ranges(clusters: &[ClusterMetrics]) -> Vec<(usize, usize)> {
        clusters
            .iter()
            .map(|c| (c.text_range.start, c.text_range.end))
            .collect()
    }

    #[test]
    fn range_formatting() {
        let mut text = layout("ab");
        let height = text.metrics().height;
        text.set_font_size(1..2, 48.0);
        assert!(text.metrics().height > height);

        let mut text = layout("iiiiWWWW");
        let widths = |text: &TextLayout| {
            let clusters = text.cluster_metrics();
            (clusters[0].width, clusters[7].width)
        };
        let (narrow, wide) = widths(&text);
        assert!(narrow < wide);
        text.set_font_family(.., "Consolas");
        let (narrow, wide) = widths(&text);
        assert_eq!(narrow, wide);

        let mut text = layout("Hello");
        let width = text.metrics().width;
        text.set_font_weight(.., FontWeight::BLACK);
        assert!(text.metrics().width > width);

        let mut text = layout("Slanted text");
        let width = text.metrics().width;
        text.set_font_style(.., FontStyle::Italic);
        assert_ne!(text.metrics().width, width);

        // Consolas is missing from a Segoe UI collection until the range uses the system fonts.
        let data = std::fs::read(std::env::var("WINDIR").unwrap() + r"\Fonts\segoeui.ttf").unwrap();
        let segoe = FontCollection::from_data([data.as_slice()]).unwrap();
        let format = TextFormat::new("Consolas", 24.0)
            .unwrap()
            .with_collection(&segoe);
        let mut text = TextLayout::new("iiiiWWWW", &format, 400.0, 200.0).unwrap();
        let (narrow, wide) = widths(&text);
        assert!(narrow < wide);
        text.set_font_collection(.., &FontCollection::system());
        let (narrow, wide) = widths(&text);
        assert_eq!(narrow, wide);
    }

    #[test]
    fn font_features() {
        let format = TextFormat::new("Calibri", 24.0).unwrap();
        let mut layout = TextLayout::new("fi", &format, 400.0, 200.0).unwrap();
        assert_eq!(ranges(&layout.cluster_metrics()), [(0, 2)]);

        layout.set_font_features(.., &[FontFeature::new(b"liga", 0)]);
        assert_eq!(ranges(&layout.cluster_metrics()), [(0, 1), (1, 2)]);

        // The features replace the defaults, so ligatures need `liga` in the list.
        layout.set_font_features(.., &[FontFeature::new(b"kern", 1)]);
        assert_eq!(ranges(&layout.cluster_metrics()), [(0, 1), (1, 2)]);
        layout.set_font_features(.., &[FontFeature::new(b"liga", 1)]);
        assert_eq!(ranges(&layout.cluster_metrics()), [(0, 2)]);

        layout.set_font_features(.., &[FontFeature::new(b"kern", 1)]);
        layout.set_font_features(.., &[]);
        assert_eq!(ranges(&layout.cluster_metrics()), [(0, 2)]);
    }

    #[test]
    #[should_panic]
    fn range_must_be_on_char_boundaries() {
        layout("\u{e9}").set_underline(0..1, true);
    }

    #[test]
    fn line_metrics() {
        let lines = layout("one\r\ntwo \n\u{e9}\u{1f600}").line_metrics();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].text_range, 0..5);
        assert_eq!(lines[0].newline_length, 2);
        assert_eq!(lines[0].trailing_whitespace_length, 2);
        assert_eq!(lines[1].text_range, 5..10);
        assert_eq!(lines[1].newline_length, 1);
        assert_eq!(lines[1].trailing_whitespace_length, 2);
        assert_eq!(lines[2].text_range, 10..16);
        assert_eq!(lines[2].newline_length, 0);
        assert!(
            lines
                .iter()
                .all(|line| line.height > 0.0 && line.baseline > 0.0)
        );

        let format = TextFormat::new("Segoe UI", 24.0).unwrap();
        let wrapped = TextLayout::new("aaaa bbbb", &format, 70.0, 200.0).unwrap();
        let lines = wrapped.line_metrics();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].text_range, 0..5);
        assert_eq!(lines[0].trailing_whitespace_length, 1);
        assert_eq!(lines[1].text_range, 5..9);

        let bounds = wrapped.range_bounds(..);
        assert_eq!(bounds.len(), 2);
        assert!(bounds[0].top < bounds[1].top);
        assert_eq!(wrapped.range_bounds(1..3).len(), 1);
    }

    #[test]
    fn cluster_metrics() {
        let clusters = layout("a e\u{301}\u{1f600}\n").cluster_metrics();
        assert_eq!(ranges(&clusters), [(0, 1), (1, 2), (2, 5), (5, 9), (9, 10)]);
        assert!(clusters[1].is_whitespace && clusters[1].can_wrap_line_after);
        assert!(clusters[4].is_newline);
        assert!(!clusters[0].is_whitespace && !clusters[0].is_right_to_left);

        assert!(layout("\u{5d0}").cluster_metrics()[0].is_right_to_left);
        assert!(layout("").cluster_metrics().is_empty());
    }

    #[test]
    fn long_text_offsets() {
        // Characters of every UTF-8 length, so that chunks of the offset index split them.
        let text = "a\u{e9}\u{20ac}\u{1f600}".repeat(100);
        let format = TextFormat::new("Segoe UI", 24.0).unwrap();
        let layout = TextLayout::new(&text, &format, 1e6, 200.0).unwrap();

        let clusters = layout.cluster_metrics();
        let expected: Vec<_> = text
            .char_indices()
            .map(|(index, ch)| (index, index + ch.len_utf8()))
            .collect();
        assert_eq!(ranges(&clusters), expected);

        let mut left = 0.0;
        for cluster in &clusters {
            let start = cluster.text_range.start;
            let caret = layout.caret_bounds(start, false);
            assert!((caret.left - left).abs() < 0.01);

            let hit = layout.hit_test_point(Vector2::new(left + cluster.width / 2.0, 10.0));
            assert_eq!(
                hit.text_position..hit.text_position + hit.length,
                cluster.text_range
            );
            left += cluster.width;
        }
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn glyph_runs_place_glyphs_at_carets() {
        let layout = layout("Hello, world");
        let runs = layout.glyph_runs();
        assert_eq!(runs.len(), 1);

        let run = &runs[0];
        assert_eq!(run.font_face.family_name(), "Segoe UI");
        assert_eq!(run.font_size, 24.0);
        assert!(!run.is_right_to_left);
        assert_eq!(run.text_range, 0..12);
        assert_eq!(run.baseline_origin.x, 0.0);
        assert!(close(
            run.baseline_origin.y,
            layout.line_metrics()[0].baseline
        ));
        assert_eq!(run.glyphs.len(), 12);

        let face = &run.font_face;
        for (index, (glyph, ch)) in run.glyphs.iter().zip(layout.text().chars()).enumerate() {
            assert_eq!(glyph.text_position, index);
            assert_eq!(Some(glyph.id), face.glyph_index(ch));
            assert_eq!(glyph.position.y, run.baseline_origin.y);
            assert!(close(
                glyph.position.x,
                layout.caret_bounds(index, false).left
            ));
        }

        let last = run.glyphs.last().unwrap();
        assert!(close(
            last.position.x + last.advance,
            layout.metrics().width
        ));
    }

    #[test]
    fn glyph_runs_map_clusters_to_utf8() {
        // A combining accent joins its letter's cluster, and the emoji falls back to another
        // face in its own run.
        let layout = layout("e\u{301}x\u{1F600}");
        let runs = layout.glyph_runs();
        assert!(runs.len() >= 2);
        assert_eq!(runs[0].text_range, 0..4);
        assert_eq!(runs.last().unwrap().text_range, 4..8);
        assert_ne!(runs[0].font_face, runs.last().unwrap().font_face);

        let positions: Vec<_> = runs
            .iter()
            .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.text_position))
            .collect();
        assert_eq!(positions.first(), Some(&0));
        assert!(
            positions
                .iter()
                .all(|position| [0, 3, 4].contains(position))
        );
        assert!(positions.contains(&3));
        assert_eq!(positions.last(), Some(&4));
    }

    #[test]
    fn glyph_runs_right_to_left() {
        let layout = layout("\u{5E9}\u{5DC}\u{5D5}\u{5DD}");
        let runs = layout.glyph_runs();
        assert_eq!(runs.len(), 1);

        let run = &runs[0];
        assert!(run.is_right_to_left);
        assert_eq!(run.text_range, 0..8);

        // The first glyph in text order is the rightmost, ending at the run's origin.
        let first = &run.glyphs[0];
        assert!(close(
            first.position.x + first.advance,
            run.baseline_origin.x
        ));

        for pair in run.glyphs.windows(2) {
            assert!(close(
                pair[1].position.x + pair[1].advance,
                pair[0].position.x
            ));
        }

        for glyph in &run.glyphs {
            let caret = layout.caret_bounds(glyph.text_position, false);
            assert!(close(glyph.position.x + glyph.advance, caret.left));
        }
    }

    #[test]
    fn decorations() {
        let mut layout = layout("Hello world");
        assert!(layout.decorations().is_empty());

        layout.set_underline(0..5, true);
        layout.set_strikethrough(6.., true);
        let decorations = layout.decorations();
        assert_eq!(decorations.len(), 2);

        let baseline = layout.line_metrics()[0].baseline;
        let underline = decorations
            .iter()
            .find(|d| d.kind == DecorationKind::Underline)
            .unwrap();
        let strikethrough = decorations
            .iter()
            .find(|d| d.kind == DecorationKind::Strikethrough)
            .unwrap();

        let hello = layout.range_bounds(0..5)[0];
        assert!(close(underline.bounds.left, hello.left));
        assert!(close(underline.bounds.right, hello.right));
        assert!(underline.bounds.top > baseline);
        assert!(underline.bounds.height() > 0.0);

        let world = layout.range_bounds(6..)[0];
        assert!(close(strikethrough.bounds.left, world.left));
        assert!(close(strikethrough.bounds.right, world.right));
        assert!(strikethrough.bounds.bottom < baseline);
        assert!(strikethrough.bounds.height() > 0.0);
    }

    #[test]
    fn decorations_right_to_left() {
        let mut layout = layout("abc \u{5E9}\u{5DC}\u{5D5}\u{5DD}");
        layout.set_underline(4.., true);
        let decorations = layout.decorations();
        assert_eq!(decorations.len(), 1);

        let hebrew = layout.range_bounds(4..)[0];
        assert!(close(decorations[0].bounds.left, hebrew.left));
        assert!(close(decorations[0].bounds.right, hebrew.right));
    }

    #[test]
    fn shared_across_threads() {
        let data = std::fs::read(std::env::var("WINDIR").unwrap() + r"\Fonts\consola.ttf").unwrap();
        let loaded = FontCollection::from_data([data.as_slice()]).unwrap();
        let system = FontCollection::system();
        let fallback = FontFallback::new(&system, &["Segoe UI Symbol"]);
        let format = TextFormat::new("Consolas", 24.0)
            .unwrap()
            .with_collection(&loaded)
            .with_fallback(&fallback);
        let face = loaded
            .find("Consolas", FontWeight::NORMAL, FontStyle::Normal)
            .unwrap();

        let work = |face: &FontFace| {
            let glyph = face.glyph_index('W').unwrap();
            let layout = TextLayout::new("\u{2603} snow", &format, 400.0, 200.0).unwrap();
            (
                face.metrics(),
                face.glyph_metrics(glyph),
                layout.metrics(),
                system
                    .find("Segoe UI", FontWeight::BOLD, FontStyle::Normal)
                    .unwrap()
                    .weight(),
            )
        };
        let expected = work(&face);

        // Use the same objects from several threads at once.
        let barrier = std::sync::Barrier::new(8);
        std::thread::scope(|scope| {
            for _ in 0..8 {
                scope.spawn(|| {
                    barrier.wait();
                    for _ in 0..50 {
                        assert_eq!(work(&face), expected);
                    }
                });
            }
        });

        // A layout can move to another thread.
        let layout = TextLayout::new("snow", &format, 400.0, 200.0).unwrap();
        let width = std::thread::spawn(move || layout.metrics().width)
            .join()
            .unwrap();
        assert!(width > 0.0);
    }
}
