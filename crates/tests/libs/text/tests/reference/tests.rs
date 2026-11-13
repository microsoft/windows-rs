use super::*;

fn position(layout: &IDWriteTextLayout2, offset: u32) -> DWRITE_HIT_TEST_METRICS {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut metrics = DWRITE_HIT_TEST_METRICS::default();
    unsafe {
        layout
            .HitTestTextPosition(offset, false, &mut x, &mut y, &mut metrics)
            .unwrap();
    }
    metrics
}

#[test]
fn columns_follow_flow_not_reading_direction() {
    for mode in [Mode::VerticalRightToLeft, Mode::VerticalLeftToRight] {
        let layout = layout("日本\n日本", mode, 400.0, 300.0).unwrap();
        let first = position(&layout, 0);
        let second = position(&layout, 1);
        let next_column = position(&layout, 3);
        assert_eq!(first.left, second.left);
        assert!(second.top > first.top);
        assert_eq!(first.top, next_column.top);
        match mode {
            Mode::VerticalRightToLeft => assert!(next_column.left < first.left),
            Mode::VerticalLeftToRight => assert!(next_column.left > first.left),
            _ => unreachable!(),
        }
    }
}

#[test]
fn vertical_wrap_uses_height_and_reflows() {
    let layout = layout(
        "日本語日本語日本語",
        Mode::VerticalRightToLeft,
        600.0,
        600.0,
    )
    .unwrap();
    assert_eq!(metrics(&layout).base.lineCount, 1);
    unsafe { layout.SetMaxHeight(60.0).unwrap() };
    assert!(metrics(&layout).base.lineCount > 1);
    unsafe { layout.SetMaxHeight(600.0).unwrap() };
    assert_eq!(metrics(&layout).base.lineCount, 1);
}

#[test]
fn native_hit_tests_keep_utf16_clusters_and_choices() {
    for mode in Mode::ALL {
        let layout = layout(TEXT, mode, 700.0, 700.0).unwrap();
        for (index, choice) in CHOICES.iter().enumerate() {
            let range = range(TEXT, choice);
            let boxes = bounds(&layout, range);
            assert!(!boxes.is_empty());
            for rect in boxes {
                let hit = hit(
                    &layout,
                    rect.left + rect.width / 2.0,
                    rect.top + rect.height / 2.0,
                );
                assert!(hit.inside);
                assert_eq!(super::choice(&hit), Some(index));
            }
        }
        let range = range(TEXT, "😀");
        let rect = position(&layout, range.startPosition);
        let hit = hit(
            &layout,
            rect.left + rect.width / 2.0,
            rect.top + rect.height / 2.0,
        );
        assert_eq!(hit.metrics.textPosition, range.startPosition);
        assert_eq!(hit.metrics.length, 2);
        assert_eq!(choice(&hit), None);
        assert_eq!(choice(&super::hit(&layout, -100.0, -100.0)), None);
    }
}

#[test]
fn vertical_metrics_include_trailing_space_on_y_axis() {
    let layout = layout("日本  ", Mode::VerticalRightToLeft, 400.0, 300.0).unwrap();
    let metrics = metrics(&layout);
    assert!(metrics.heightIncludingTrailingWhitespace > metrics.base.height);
    assert_eq!(
        metrics.base.width,
        metrics.base.widthIncludingTrailingWhitespace
    );
}

#[test]
fn parallel_reading_and_flow_are_rejected_on_use() {
    let layout = layout(TEXT, Mode::Horizontal, 400.0, 300.0).unwrap();
    unsafe {
        layout
            .SetReadingDirection(DWRITE_READING_DIRECTION_TOP_TO_BOTTOM)
            .unwrap();
        let mut metrics = DWRITE_TEXT_METRICS1::default();
        assert_eq!(
            layout.GetMetrics(&mut metrics),
            DWRITE_E_FLOWDIRECTIONCONFLICTS
        );
    }
}

#[test]
fn reference_agrees_with_windows_text_geometry() {
    for mode in Mode::ALL {
        let reference = layout(TEXT, mode, 700.0, 700.0).unwrap();
        let actual = windows_text::TextLayout::new(TEXT, &format(mode), 700.0, 700.0);
        let reference_metrics = metrics(&reference);
        let actual_metrics = actual.metrics();
        assert_eq!(actual_metrics.width, reference_metrics.base.width);
        assert_eq!(actual_metrics.height, reference_metrics.base.height);
        assert_eq!(actual_metrics.line_count, reference_metrics.base.lineCount);
        assert_eq!(
            actual_metrics.height_including_trailing_whitespace,
            reference_metrics.heightIncludingTrailingWhitespace
        );

        for text in CHOICES.into_iter().chain(["😀", "e\u{301}"]) {
            let native_range = range(TEXT, text);
            let start = TEXT.find(text).unwrap();
            let actual_bounds = actual.range_bounds(start..start + text.len());
            let expected = bounds(&reference, native_range);
            assert_eq!(actual_bounds.len(), expected.len());
            for (actual_rect, expected_rect) in actual_bounds.iter().zip(expected) {
                assert_eq!(*actual_rect, crate::rectangle(&expected_rect));
                let point = windows_text::Vector2::new(
                    expected_rect.left + expected_rect.width / 2.0,
                    expected_rect.top + expected_rect.height / 2.0,
                );
                let actual_hit = actual.hit_test_point(point);
                assert!(actual_hit.is_inside);
                assert!((start..start + text.len()).contains(&actual_hit.text_range.start));
                assert!(TEXT.is_char_boundary(actual_hit.text_range.start));
                assert!(TEXT.is_char_boundary(actual_hit.text_range.end));
                let native_hit = hit(&reference, point.x, point.y);
                let selected = CHOICES.iter().position(|choice| {
                    let start = TEXT.find(choice).unwrap();
                    (start..start + choice.len()).contains(&actual_hit.text_range.start)
                });
                assert_eq!(selected, choice(&native_hit));
                assert_eq!(
                    actual.caret_bounds(actual_hit.text_range.start, actual_hit.is_trailing_hit),
                    caret(&reference, mode, &native_hit),
                );
            }
        }
    }
}
