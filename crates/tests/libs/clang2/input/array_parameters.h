typedef void (*StaticBound)(int values[static 3]);
typedef void (*QualifiedBound)(int values[const 3]);
typedef void (*VariableBound)(int count, int values[count]);
typedef void (*GoodBound)(int values[1 + 2]);
