enum { Zero = 0, Expression = (2 << 3), Negative = -4 };
enum : unsigned long long { High = 0xffffffffffffffffULL };
enum : long long { Low = (-9223372036854775807LL - 1) };
enum { value__ = 1 };
enum : unsigned short { Small = 65535 };
struct SharedName { int value; };
enum { SharedName = 7 };
typedef enum { NamedValue = 5 } Named;
#if defined(_M_IX86)
enum { Width = 32 };
#else
enum : unsigned long long { Width = 64 };
#endif
