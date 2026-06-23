//! Settings fixture: disabling script prevents the page from running it.

use crate::harness::Harness;

const SETS_TITLE: &str = "<!DOCTYPE html><html><head><title>original</title></head>\
     <body><script>document.title = 'changed';</script></body></html>";

/// With script disabled the page's inline script does not run, so the title
/// stays as authored; re-enabling lets it run.
pub fn disable_script_blocks_execution(harness: &Harness) {
    let Ok(settings) = harness.webview().settings() else {
        harness.check("Settings_Get", false);
        return;
    };

    if settings.set_script_enabled(false).is_err() {
        harness.check("Settings_Disable", false);
        return;
    }
    harness.navigate_html(SETS_TITLE);
    harness.check(
        "Settings_ScriptDisabled_TitleUnchanged",
        harness.webview().document_title() == "original",
    );

    let _ = settings.set_script_enabled(true);
    harness.navigate_html(SETS_TITLE);
    let changed = harness.wait(|| harness.webview().document_title() == "changed");
    harness.check("Settings_ScriptEnabled_TitleChanged", changed);
}
