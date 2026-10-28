#ifndef ALTERNATE_TYPE
#define ALTERNATE_TYPE int
#endif
#ifdef UNOWNED_CANONICAL
struct Hidden {
    int value;
};
typedef Hidden Canonical;
#else
struct Canonical {
    int value;
};
#endif
#ifdef DISTINCT_ALIAS
struct Alternate {
    ALTERNATE_TYPE value;
};
#else
typedef Canonical Alternate;
#endif
typedef Alternate* PAlternate;
struct CanonicalConsumer {
    Canonical value;
};
struct AlternateConsumer {
    PAlternate value;
};
