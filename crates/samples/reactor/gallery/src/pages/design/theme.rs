use crate::controls::*;
use windows_reactor::*;

pub fn theme_page(_: &(), _cx: &mut RenderCx) -> Element {
    page_content(
        "Theme",
        "Guidance on applying light, dark, and high-contrast themes using WinUI theme resources.",
        vec![
            sample_card(
                "Theme Token Reference",
                vstack((
                    text_block("Use ThemeRef variants to reference system brushes that automatically adapt to the active theme.")
                        .opacity(0.6)
                        .font_size(13.0),
                    token_row("Accent", ThemeRef::Accent, "Brand accent color"),
                    token_row("CardBackground", ThemeRef::CardBackground, "Card surfaces"),
                    token_row("SubtleFill", ThemeRef::SubtleFill, "Subtle backgrounds"),
                    token_row("LayerFill", ThemeRef::LayerFill, "Floating surfaces"),
                    token_row("SolidBackground", ThemeRef::SolidBackground, "Page background"),
                ))
                .spacing(8.0)
                ,
                r#"// Use theme references for adaptive colors:
text_block("Hello").foreground(ThemeRef::PrimaryText)
vstack((...)).background(ThemeRef::CardBackground)"#,
            ),
            sample_card(
                "Text Theme Tokens",
                vstack((
                    text_block("Primary text").foreground(ThemeRef::PrimaryText),
                    text_block("Secondary text").foreground(ThemeRef::SecondaryText),
                    text_block("Tertiary text").foreground(ThemeRef::TertiaryText),
                    text_block("Disabled text").foreground(ThemeRef::DisabledText),
                    text_block("Accent text").foreground(ThemeRef::AccentText),
                ))
                .spacing(4.0)
                ,
                r#"text_block("Primary").foreground(ThemeRef::PrimaryText)
text_block("Secondary").foreground(ThemeRef::SecondaryText)
text_block("Accent").foreground(ThemeRef::AccentText)"#,
            ),
            sample_card(
                "System Status Colors",
                vstack((
                    status_row("Attention", ThemeRef::SystemAttention, ThemeRef::SystemAttentionBackground),
                    status_row("Success", ThemeRef::SystemSuccess, ThemeRef::SystemSuccessBackground),
                    status_row("Caution", ThemeRef::SystemCaution, ThemeRef::SystemCautionBackground),
                    status_row("Critical", ThemeRef::SystemCritical, ThemeRef::SystemCriticalBackground),
                ))
                .spacing(8.0)
                ,
                r#"// System status colors:
ThemeRef::SystemSuccess       // foreground
ThemeRef::SystemSuccessBackground  // background"#,
            ),
            sample_card(
                "Stroke Tokens",
                vstack((
                    stroke_row("CardStroke", ThemeRef::CardStroke),
                    stroke_row("SurfaceStroke", ThemeRef::SurfaceStroke),
                    stroke_row("DividerStroke", ThemeRef::DividerStroke),
                    stroke_row("ControlStroke", ThemeRef::ControlStroke),
                ))
                .spacing(8.0)
                ,
                r#"border(content)
    .border_brush_theme(ThemeRef::CardStroke)
    .border_thickness(Thickness::uniform(1.0))"#,
            ),
        ],
    )
}

fn token_row(name: &str, theme_ref: ThemeRef, description: &str) -> Element {
    hstack((
        border(vstack(()).width(22.0).height(22.0))
            .background(theme_ref)
            .corner_radius(4.0),
        vstack((
            text_block(format!("ThemeRef::{name}"))
                .font_size(13.0)
                .bold(),
            text_block(description).font_size(12.0).opacity(0.6),
        ))
        .spacing(2.0),
    ))
    .spacing(8.0)
    .into()
}

fn status_row(name: &str, fg: ThemeRef, bg: ThemeRef) -> Element {
    hstack((border(
        text_block(name)
            .font_size(12.0)
            .foreground(fg)
            .padding(Thickness::xy(8.0, 4.0)),
    )
    .background(bg)
    .corner_radius(4.0),))
    .into()
}

fn stroke_row(name: &str, _theme_ref: ThemeRef) -> Element {
    hstack((
        border(vstack(()).width(46.0).height(22.0))
            .corner_radius(4.0)
            .border_brush(Color::rgb(128, 128, 128))
            .border_thickness(Thickness::uniform(1.0)),
        text_block(format!("ThemeRef::{name}")).font_size(13.0),
    ))
    .spacing(8.0)
    .into()
}
