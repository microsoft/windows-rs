// Tests that a non-variadic fn-ptr typedef is emitted as a callback while a
// variadic fn-ptr typedef is silently dropped (winmd cannot represent variadic
// callbacks).  Only RegularCb must appear in the generated RDL.

typedef void (*RegularCb)(int a);
typedef int (*VariadicCb)(const char* fmt, ...);
