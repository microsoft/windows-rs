// Typedef that renames a struct tag to a public name (anonymous struct via typedef).
typedef struct {
    int width;
    int height;
} Size;

typedef struct _Impl {
    void* handle;
    int refcount;
} Widget;

// A typedef aliasing an already-defined public tag (not inline, no _/tag prefix) must
// keep the tag's name and emit a distinct alias - mirrors dwrite's IDWriteGeometrySink.
typedef struct Public { int a; } Public;
typedef Public PublicAlias;
