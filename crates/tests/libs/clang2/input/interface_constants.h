struct IUnknown { virtual void Method() = 0; };
typedef IUnknown* Raw;
typedef Raw LPUNKNOWN;
#define UNKNOWN_NULL ((LPUNKNOWN)0)
#define INTEGER_CONSTANT 42
extern "C" void Use(IUnknown* input, LPUNKNOWN* output);
