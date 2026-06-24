use std::collections::HashMap;
use std::fmt::Write;
use windows_metadata::reader::{File, Index, Item, TypeCategory};

/// The metadata that backs the published `windows` and `windows-sys` crates.
/// Both crates share the same namespace-to-feature taxonomy, so a single index
/// answers "which feature do I enable?" for either crate.
const WINMD: [&str; 3] = [
    "crates/libs/bindgen/default/Windows.winmd",
    "crates/libs/bindgen/default/Windows.Win32.winmd",
    "crates/libs/bindgen/default/Windows.Wdk.winmd",
];

/// The folder published to GitHub Pages by `web.yml`; regenerated and checked
/// in by `gen.yml` like every other tool's output. Nested under `features` so
/// the page is served at `microsoft.github.io/windows-rs/features`.
const OUTPUT: &str = "web/features";

/// A record in the search index: an API's namespace and simple name.
struct Entry {
    namespace: usize,
    name: String,
}

fn main() {
    let time = std::time::Instant::now();
    generate_page(OUTPUT);
    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

/// Loads the bundled metadata and projects every type, function, constant, and
/// interface method into a flat list of [`Entry`] records plus the namespace
/// table they index. The output is canonical (namespaces sorted, entries sorted
/// and de-duplicated) so the generated page is byte-for-byte deterministic.
fn load() -> (Vec<String>, Vec<Entry>) {
    let files: Vec<File> = WINMD
        .iter()
        .map(|path| File::read(path).unwrap_or_else(|| panic!("cannot read {path}")))
        .collect();

    let index = Index::new(files);

    let mut raw: Vec<(String, String)> = Vec::new();

    for (namespace, name, item) in index.iter_items() {
        // An interface also contributes its methods as `Interface::Method`
        // entries; a method always requires the same feature as its interface.
        if let Item::Type(ty) = item
            && ty.category() == TypeCategory::Interface
        {
            for method in ty.methods() {
                raw.push((namespace.to_string(), format!("{name}::{}", method.name())));
            }
        }
        raw.push((namespace.to_string(), name.to_string()));
    }

    let mut namespaces: Vec<String> = raw.iter().map(|(ns, _)| ns.clone()).collect();
    namespaces.sort();
    namespaces.dedup();

    let lookup: HashMap<&str, usize> = namespaces
        .iter()
        .enumerate()
        .map(|(i, ns)| (ns.as_str(), i))
        .collect();

    let mut entries: Vec<Entry> = raw
        .into_iter()
        .map(|(ns, name)| Entry {
            namespace: lookup[ns.as_str()],
            name,
        })
        .collect();

    entries.sort_by(|a, b| (a.namespace, &a.name).cmp(&(b.namespace, &b.name)));
    entries.dedup_by(|a, b| a.namespace == b.namespace && a.name == b.name);

    (namespaces, entries)
}

/// Emits a single self-contained, dependency-free `index.html` that searches an
/// inlined index in the browser. The Cargo feature for each API is derived in
/// the page from its namespace (drop the leading `Windows`, join with `_`; the
/// two `Foundation` namespaces are always compiled and need no feature).
/// Inlining keeps it to one file that works both when hosted and when opened
/// directly from disk (a `file://` page cannot `fetch` a sibling file).
fn generate_page(dir: &str) {
    let (namespaces, entries) = load();

    std::fs::create_dir_all(dir).unwrap();

    let mut json = String::from("{\"namespaces\":[");
    for (i, namespace) in namespaces.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        write!(json, "\"{}\"", escape(namespace)).unwrap();
    }
    json.push_str("],\"items\":[");
    for (i, entry) in entries.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        write!(json, "[\"{}\",{}]", escape(&entry.name), entry.namespace).unwrap();
    }
    json.push_str("]}");

    let html_path = format!("{dir}/index.html");
    std::fs::write(&html_path, PAGE.replace("__FEATURES_JSON__", &json)).unwrap();

    println!(
        "Wrote {} items across {} namespaces:\n  {html_path} (self-contained, {} KB)",
        entries.len(),
        namespaces.len(),
        json.len() / 1024
    );
}

/// Minimal JSON string escaping for metadata identifiers.
fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => write!(out, "\\u{:04x}", c as u32).unwrap(),
            c => out.push(c),
        }
    }
    out
}

const PAGE: &str = r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>windows-rs feature search</title>
<style>
  :root { color-scheme: light dark; }
  body { font-family: system-ui, sans-serif; max-width: 60rem; margin: 2rem auto; padding: 0 1rem; }
  h1 { font-size: 1.4rem; }
  p { color: gray; }
  input { width: 100%; box-sizing: border-box; font-size: 1.1rem; padding: 0.5rem; margin: 0.5rem 0 1rem; }
  table { border-collapse: collapse; width: 100%; }
  th, td { text-align: left; padding: 0.3rem 0.6rem; border-bottom: 1px solid #8884; vertical-align: top; }
  code { font-family: ui-monospace, monospace; }
  .feature { font-weight: bold; }
  .none { color: gray; font-weight: normal; }
  .status { color: gray; margin-top: 0.5rem; }
</style>
</head>
<body>
<h1>windows-rs feature search</h1>
<p>Type an API name to find the Cargo feature to enable in the <code>windows</code> or <code>windows-sys</code> crate. Matching is a case-insensitive regular expression over <code>Namespace::Name</code>.</p>
<input id="q" placeholder="CreateFileW, IDWriteFactory, Direct3D11, ..." autofocus>
<div id="status" class="status">Loading index&hellip;</div>
<table><thead><tr><th>API</th><th>Feature</th></tr></thead><tbody id="results"></tbody></table>
<script id="data" type="application/json">__FEATURES_JSON__</script>
<script>
const LIMIT = 200;
const DATA = JSON.parse(document.getElementById('data').textContent);
const input = document.getElementById('q');
const status = document.getElementById('status');
const results = document.getElementById('results');

function featureOf(ns) {
  if (ns === 'Windows.Foundation' || ns === 'Windows.Win32.Foundation') return null;
  const parts = ns.split('.').slice(1);
  return parts.length ? parts.join('_') : null;
}

function run() {
  const query = input.value.trim();
  results.innerHTML = '';
  if (!query) { status.textContent = DATA.items.length + ' APIs indexed.'; return; }
  let re;
  try { re = new RegExp(query, 'i'); } catch { re = null; }
  let count = 0, shown = 0;
  const rows = [];
  for (const [name, nsIdx] of DATA.items) {
    const ns = DATA.namespaces[nsIdx];
    const path = ns.split('.').slice(1);
    const full = (path.length ? path.join('::') + '::' : '') + name;
    const hit = re ? re.test(full) : full.toLowerCase().includes(query.toLowerCase());
    if (!hit) continue;
    count++;
    if (shown >= LIMIT) continue;
    shown++;
    const feature = featureOf(ns);
    const cell = feature
      ? '<code class="feature">' + feature + '</code>'
      : '<span class="none">(no feature required)</span>';
    rows.push('<tr><td><code>' + full + '</code></td><td>' + cell + '</td></tr>');
  }
  results.innerHTML = rows.join('');
  status.textContent = count === 0 ? 'No matches.'
    : count > LIMIT ? (count + ' matches (showing first ' + LIMIT + ') \u2014 refine to narrow.')
    : (count + ' match(es).');
}

input.addEventListener('input', run);
run();
</script>
</body>
</html>
"#;
