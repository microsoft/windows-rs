#ifndef TAG
#define TAG First
#endif
#ifndef FIELD_TYPE
#define FIELD_TYPE int
#endif
#ifndef PACKING
#define PACKING
#endif
#ifndef FIELD_ARRAY
#define FIELD_ARRAY
#endif
struct Left { int unused; };
struct Right { int unused; };
struct PACKING TAG {
#ifndef NO_NEXT
    TAG* next;
#endif
    FIELD_TYPE value FIELD_ARRAY;
};
#ifdef POINTER_ALIAS
typedef TAG* Public;
#else
typedef TAG Public;
#endif
