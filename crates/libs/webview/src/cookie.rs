use super::*;

/// The `SameSite` policy of a [`Cookie`], controlling whether it is sent with
/// cross-site requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SameSite {
    /// Sent with all requests, including cross-site (requires
    /// [`is_secure`](Cookie::is_secure)).
    None,
    /// Sent with same-site requests and top-level cross-site navigations.
    Lax,
    /// Sent only with same-site requests.
    Strict,
}

impl SameSite {
    fn from_raw(value: COREWEBVIEW2_COOKIE_SAME_SITE_KIND) -> Self {
        match value {
            0 => Self::None,
            2 => Self::Strict,
            _ => Self::Lax,
        }
    }

    fn to_raw(self) -> COREWEBVIEW2_COOKIE_SAME_SITE_KIND {
        match self {
            Self::None => 0,
            Self::Lax => 1,
            Self::Strict => 2,
        }
    }
}

/// A browser cookie. Read cookies with [`CookieManager::get_cookies`] and write
/// them with [`CookieManager::add_or_update_cookie`].
#[derive(Clone, Debug, PartialEq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    /// Whether the cookie is sent only over HTTPS.
    pub is_secure: bool,
    /// Whether the cookie is hidden from script (`HttpOnly`).
    pub is_http_only: bool,
    pub same_site: SameSite,
    /// When the cookie expires, as seconds since the Unix epoch, or `None` for a
    /// session cookie that is cleared when the browser closes.
    pub expires: Option<f64>,
}

impl Cookie {
    /// Creates a cookie for the given `name`, `value`, `domain`, and `path` with
    /// default attributes (a non-secure, `Lax`, session cookie). Adjust the
    /// public fields before passing it to
    /// [`CookieManager::add_or_update_cookie`].
    pub fn new(name: &str, value: &str, domain: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            domain: domain.to_string(),
            path: path.to_string(),
            is_secure: false,
            is_http_only: false,
            same_site: SameSite::Lax,
            expires: None,
        }
    }

    fn from_com(cookie: &ICoreWebView2Cookie) -> Result<Self> {
        unsafe {
            let expires = if cookie.IsSession()?.as_bool() {
                None
            } else {
                Some(cookie.Expires()?)
            };
            Ok(Self {
                name: string::take(cookie.Name()?),
                value: string::take(cookie.Value()?),
                domain: string::take(cookie.Domain()?),
                path: string::take(cookie.Path()?),
                is_secure: cookie.IsSecure()?.as_bool(),
                is_http_only: cookie.IsHttpOnly()?.as_bool(),
                same_site: SameSite::from_raw(cookie.SameSite()?),
                expires,
            })
        }
    }
}

/// Reads, writes, and deletes the browser's cookies. Obtain it with
/// [`WebView::cookie_manager`].
pub struct CookieManager(pub(crate) ICoreWebView2CookieManager);

impl CookieManager {
    /// Asynchronously retrieves the cookies that apply to `uri` (or all cookies
    /// when `uri` is empty). The `handler` closure receives them on the UI
    /// thread.
    pub fn get_cookies<F: FnOnce(Result<Vec<Cookie>>) + 'static>(
        &self,
        uri: &str,
        handler: F,
    ) -> Result<()> {
        let uri = HSTRING::from(uri);
        let handler = handler::GetCookiesCompleted::create(handler);
        unsafe { self.0.GetCookies(&uri, &handler) }
    }

    /// Adds the cookie, or updates the existing cookie with the same name,
    /// domain, and path.
    pub fn add_or_update_cookie(&self, cookie: &Cookie) -> Result<()> {
        let name = HSTRING::from(&cookie.name);
        let value = HSTRING::from(&cookie.value);
        let domain = HSTRING::from(&cookie.domain);
        let path = HSTRING::from(&cookie.path);
        unsafe {
            let raw = self.0.CreateCookie(&name, &value, &domain, &path)?;
            raw.SetIsSecure(cookie.is_secure)?;
            raw.SetIsHttpOnly(cookie.is_http_only)?;
            raw.SetSameSite(cookie.same_site.to_raw())?;
            if let Some(expires) = cookie.expires {
                raw.SetExpires(expires)?;
            }
            self.0.AddOrUpdateCookie(&raw)
        }
    }

    /// Deletes cookies with the matching `name` that apply to `uri`.
    pub fn delete_cookies(&self, name: &str, uri: &str) -> Result<()> {
        let name = HSTRING::from(name);
        let uri = HSTRING::from(uri);
        unsafe { self.0.DeleteCookies(&name, &uri) }
    }

    /// Deletes cookies with the matching `name`, `domain`, and `path`.
    pub fn delete_cookies_with_domain_and_path(
        &self,
        name: &str,
        domain: &str,
        path: &str,
    ) -> Result<()> {
        let name = HSTRING::from(name);
        let domain = HSTRING::from(domain);
        let path = HSTRING::from(path);
        unsafe { self.0.DeleteCookiesWithDomainAndPath(&name, &domain, &path) }
    }

    /// Deletes all cookies.
    pub fn delete_all_cookies(&self) -> Result<()> {
        unsafe { self.0.DeleteAllCookies() }
    }
}

pub(crate) fn collect(list: &ICoreWebView2CookieList) -> Result<Vec<Cookie>> {
    let count = unsafe { list.Count()? };
    let mut cookies = Vec::with_capacity(count as usize);
    for index in 0..count {
        let cookie = unsafe { list.GetValueAtIndex(index)? };
        cookies.push(Cookie::from_com(&cookie)?);
    }
    Ok(cookies)
}
