struct AnonymousArray {
    struct { int value; } items[2];
};
struct AnonymousPointer {
    struct { int value; } *item;
};
struct SharedAnonymous {
    struct { int value; } first, second;
};
struct QualifiedAnonymous {
    const struct { int value; } item;
};
struct NamedInUnnamed {
    struct {
        struct Inner { int value; } item;
    } outer;
};
class PrivateOwner {
    struct Inner { int value; };
    Inner item;
};
struct BehavioralOwner {
    struct Inner { int value; };
    int value;
    void Method();
};
