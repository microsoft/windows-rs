struct Initialized { int value = 1; };
struct InitializedExpression { int value = 1 + 2; };
struct InitializedArray { int values[1 + 2] = {}; };
struct InitializedBits { unsigned value : (1 + 2) = 1; };
struct Dependent { Initialized* value; };
struct PlatformInitializer {
#if defined(_M_IX86)
    int value = 1;
#else
    int value;
#endif
};
struct Safe { int values[1 + 2]; };
