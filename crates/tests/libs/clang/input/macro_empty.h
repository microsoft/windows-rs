#define EMPTY
#define COMMENT_ONLY /* no replacement */
#define LINE_ONLY // no replacement
#define INDIRECT_EMPTY EMPTY
#define REPLACED
#undef REPLACED
#define REPLACED 42
#define BECAME_EMPTY 42
#undef BECAME_EMPTY
#define BECAME_EMPTY
#define SPACED \
    6
#define ZERO 0
