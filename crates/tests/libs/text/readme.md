# Text tests

```console
cargo test -p test_text
cargo test -p test_text --test vertical
```

The tests use real DirectWrite objects. Rendering comparisons use off-screen WARP targets and
do not need an interactive desktop.

| Location | Coverage |
| --- | --- |
| `src/lib.rs` | Public font, format, layout, UTF-8 mapping, glyph, and rasterization contracts |
| `tests/vertical.rs` | Orientation transforms, extracted drawing, caret and column geometry |
| `tests/reference/` | Native reference behavior and public writing-mode/Canvas comparisons |

`tests/fixtures/*.txt` contains Unicode input strings included by specific tests, not expected
output or automatically discovered test cases. Native comparisons run on the same machine as the
public API; they do not depend on committed screenshots or fixed glyph IDs.

Tests use installed Windows fonts, including Segoe UI, Segoe UI Emoji, and Consolas. Vertical
cases request Yu Gothic with Japanese locale and system fallback. Font-loading tests read
Consolas and Segoe UI from `%WINDIR%\Fonts`. Font versions and fallback can change native results,
so failures should be investigated with the installed fonts in mind.

Pixel comparisons allow one 8-bit channel step of rounding difference. Public rendering failures
report writing mode, alignment, trimming, and layout size. The workspace test workflow runs the
crate on x64, x86, and ARM64.
