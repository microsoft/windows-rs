#ifndef SHARED_VALUE
#define SHARED_VALUE 1
#endif
enum SharedKind {
    SharedFirst = SHARED_VALUE,
#ifdef EXTRA_MEMBER
    SharedSecond = 2,
#endif
};
struct EnumConsumer {
    SharedKind kind;
};
struct EnumIndependent {
    int value;
};
