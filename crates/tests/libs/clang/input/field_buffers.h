#ifndef VERSION
#define VERSION "2"
#endif
#ifndef ANNOTATION
#define ANNOTATION "_Field_size_"
#endif
#ifndef ARGUMENTS
#define ARGUMENTS "capacity"
#endif
#ifndef TAG
#define TAG FieldBuffers
#endif
#ifndef COUNT_TYPE
#define COUNT_TYPE Count
#endif
#ifndef BUFFER_FIELD
#define BUFFER_FIELD unsigned short* values
#endif
#define CONTRACT __attribute__((annotate("windows_clang:sal:" VERSION ":" ANNOTATION "\n(" ARGUMENTS ")\n(" ARGUMENTS ")")))

typedef unsigned int Count;
struct TAG {
    COUNT_TYPE capacity;
    Count readable;
    CONTRACT BUFFER_FIELD;
};
typedef TAG FieldAlias;
struct OuterFields {
    Count capacity;
    struct Inner {
        COUNT_TYPE capacity;
        Count readable;
        CONTRACT BUFFER_FIELD;
    } inner;
};
