// MIDL coclass pattern: a `typedef class X X;` forward decl followed by a
// `DECLSPEC_UUID(...)` forward declaration carrying the CLSID. The scraper must
// still emit the GUID constant (bindgen projects it as `const X: GUID`).
#define DECLSPEC_UUID(x) __declspec(uuid(x))

typedef class UIAnimationManager2 UIAnimationManager2;

class DECLSPEC_UUID("D25D8842-8884-4A4A-B321-091314379BDD")
UIAnimationManager2;
