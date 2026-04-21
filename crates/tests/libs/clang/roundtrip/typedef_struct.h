// Verifies that the old-fashioned C typedef pattern is handled correctly:
//   typedef struct _TAG { ... } ALIAS, *PALIAS;
// The internal tag name (_TAG) must never appear in the generated RDL;
// only the public typedef alias (ALIAS) should be used.
typedef struct _TEST
{
    int value;
} TEST, *PTEST;

struct TEST2
{
    TEST value;
    PTEST pointer;
};
