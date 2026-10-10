#define Initializer { 17, 23 }
#define Alias Initializer
#define EmptyInitializer {}
#define Wrapped Initializer + 1
#define Good (17 + 23)
#define String "{17, 23}"
struct Pair { int first; int second; };
#define Typed Pair{ 17, 23 }
#define MacroAttribute(text) __attribute__((annotate(text)))
#define Attribute MacroAttribute("marker")
#define Scalar(value) ((value) + 1)
#define ScalarCall Scalar(17)
