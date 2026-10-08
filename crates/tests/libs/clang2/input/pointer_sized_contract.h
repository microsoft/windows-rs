struct Object { unsigned long long value; };
typedef unsigned long long AnnotatedBase __attribute__((annotate("_In_range_(0, 1024)")));
typedef NATIVE_TYPE Size;
typedef ALIAS_TYPE Alias;
struct Container { Alias value; };
extern "C" Alias Use(Alias value);
