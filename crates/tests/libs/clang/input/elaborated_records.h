struct Holder {
    struct Context* context;
    struct Shared* first;
};

struct Other {
    struct Shared* second;
};

typedef struct AliasTarget* Target;
extern "C" struct Argument* EchoArgument(struct Argument* value);

struct DefinitionHolder {
    struct Defined* pointer;
};

struct Defined {
    int value;
};

struct Nested {
    struct Child {
        int value;
    };
    Child child;
};
