typedef enum _STATUS {
    STATUS_IDLE = 0,
    STATUS_RUNNING = 1,
    STATUS_ERROR = -1,
} STATUS;

typedef enum _FLAGS {
    FLAG_NONE = 0,
    FLAG_READ = 1,
    FLAG_WRITE = 2,
    FLAG_EXECUTE = 4,
} FLAGS;

// Realistic definition of DEFINE_ENUM_FLAG_OPERATORS as found in the Windows
// SDK headers.  Each expansion produces several inline operator overloads
// (functions with bodies) that cannot be represented in RDL and must be
// silently skipped by the parser.
#define DEFINE_ENUM_FLAG_OPERATORS(ENUMTYPE) \
    inline ENUMTYPE operator | (ENUMTYPE a, ENUMTYPE b) { return ENUMTYPE(((int)a)|((int)b)); } \
    inline ENUMTYPE& operator |= (ENUMTYPE& a, ENUMTYPE b) { return (ENUMTYPE&)(((int&)a)|=((int)b)); } \
    inline ENUMTYPE operator & (ENUMTYPE a, ENUMTYPE b) { return ENUMTYPE(((int)a)&((int)b)); } \
    inline ENUMTYPE& operator &= (ENUMTYPE& a, ENUMTYPE b) { return (ENUMTYPE&)(((int&)a)&=((int)b)); } \
    inline ENUMTYPE operator ~ (ENUMTYPE a) { return ENUMTYPE(~((int)a)); } \
    inline ENUMTYPE operator ^ (ENUMTYPE a, ENUMTYPE b) { return ENUMTYPE(((int)a)^((int)b)); } \
    inline ENUMTYPE& operator ^= (ENUMTYPE& a, ENUMTYPE b) { return (ENUMTYPE&)(((int&)a)^=((int)b)); }

DEFINE_ENUM_FLAG_OPERATORS(FLAGS)
