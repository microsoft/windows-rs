//! args -x c
// An anonymous struct defined inside a macro expansion, exactly as winnt.h's
// TYPE_ALIGNMENT / FIELD_OFFSET / C_ASSERT do for compile-time alignment checks.
// libclang spells such a tag "struct (unnamed at <file>:<line>:<col>)" — the
// tag-keyword form — which must be recognised as anonymous and dropped, not
// emitted as a type named after its raw spelling.
#define FIELD_OFFSET(type, field) ((long)(long long) & (((type *)0)->field))
#define TYPE_ALIGNMENT(t) FIELD_OFFSET(struct { char x; t test; }, test)
#define C_ASSERT(e) typedef char __C_ASSERT__[(e) ? 1 : -1]

typedef struct _THING {
    int value;
} THING;

C_ASSERT(TYPE_ALIGNMENT(THING) == 4);
