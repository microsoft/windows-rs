use windows_webview::*;

fn main() -> Result<()> {
    WebViewWindow::new("WebView2 cookies - windows-rs")
        .size(1024, 768)
        .run(|host| {
            let webview = host.webview();
            let cookies = webview.cookie_manager()?;

            let mut cookie = Cookie::new("session", "abc123", "example.com", "/");
            cookie.is_http_only = true;
            cookie.same_site = SameSite::Strict;
            cookies.add_or_update_cookie(&cookie)?;

            cookies.get_cookies("https://example.com/", |result| match result {
                Ok(cookies) => {
                    println!("{} cookie(s) for example.com:", cookies.len());
                    for cookie in cookies {
                        println!("  {} = {}", cookie.name, cookie.value);
                    }
                }
                Err(error) => println!("failed to read cookies: {error}"),
            })?;

            webview.navigate("https://example.com/")?;
            Ok(())
        })
}
