// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="index.html">Introduction</a></li><li class="chapter-item expanded affix "><a href="rust-getting-started/index.html">Getting Started with Rust</a></li><li class="chapter-item expanded "><a href="rust-getting-started/windows-or-windows-sys.html"><strong aria-hidden="true">1.</strong> Choosing between the windows and windows-sys crates</a></li><li class="chapter-item expanded "><a href="rust-getting-started/how-are-crates-built.html"><strong aria-hidden="true">2.</strong> How are these crates built?</a></li><li class="chapter-item expanded "><a href="rust-getting-started/how-to-find-api.html"><strong aria-hidden="true">3.</strong> How do I find a particular API?</a></li><li class="chapter-item expanded "><a href="rust-getting-started/what-apis-are-included.html"><strong aria-hidden="true">4.</strong> What APIs are included?</a></li><li class="chapter-item expanded "><a href="rust-getting-started/where-are-the-macros.html"><strong aria-hidden="true">5.</strong> Where&#39;s my favorite macro from the Windows SDK?</a></li><li class="chapter-item expanded "><a href="rust-getting-started/calling-your-first-windows-api.html"><strong aria-hidden="true">6.</strong> Calling your first API with the windows crate</a></li><li class="chapter-item expanded "><a href="rust-getting-started/calling-your-first-windows-sys-api.html"><strong aria-hidden="true">7.</strong> Calling your first API with the windows-sys crate</a></li><li class="chapter-item expanded "><a href="rust-getting-started/calling-your-first-com-api.html"><strong aria-hidden="true">8.</strong> Calling your first COM API</a></li><li class="chapter-item expanded "><a href="rust-getting-started/calling-your-first-winrt-api.html"><strong aria-hidden="true">9.</strong> Calling your first WinRT API</a></li><li class="chapter-item expanded "><a href="rust-getting-started/how-to-query-for-com-interface.html"><strong aria-hidden="true">10.</strong> How do I query for a specific COM interface?</a></li><li class="chapter-item expanded "><a href="rust-getting-started/how-to-implement-com-interface.html"><strong aria-hidden="true">11.</strong> How do I implement an existing COM interface?</a></li><li class="chapter-item expanded "><a href="rust-getting-started/how-to-implement-winrt-collection.html"><strong aria-hidden="true">12.</strong> How do I create stock collections for WinRT collection interfaces?</a></li><li class="chapter-item expanded "><a href="rust-getting-started/understanding-windows-targets.html"><strong aria-hidden="true">13.</strong> Understanding the windows-targets crate</a></li><li class="chapter-item expanded "><a href="rust-getting-started/standalone-code-generation.html"><strong aria-hidden="true">14.</strong> Standalone code generation</a></li><li class="chapter-item expanded "><a href="rust-getting-started/creating-your-first-dll.html"><strong aria-hidden="true">15.</strong> Creating your first DLL in Rust</a></li><li class="chapter-item expanded "><a href="rust-getting-started/implement-win32-api.html"><strong aria-hidden="true">16.</strong> Implement a traditional Win32-style API</a></li><li class="chapter-item expanded "><a href="rust-getting-started/string-tutorial.html"><strong aria-hidden="true">17.</strong> Working with strings in the windows crate</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
