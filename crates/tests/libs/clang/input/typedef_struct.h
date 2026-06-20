// Typedef that renames a struct tag to a public name (anonymous struct via typedef).
typedef struct {
    int width;
    int height;
} Size;

typedef struct _Impl {
    void* handle;
    int refcount;
} Widget;
