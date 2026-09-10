enum class IncludedForward : unsigned short {
    IncludedForwardOne = 1,
};

typedef enum IncludedEnum : unsigned int {
    IncludedEnumOne = 1,
} IncludedEnum;

typedef unsigned short EnumStorage;

enum class TypedefBacked : EnumStorage {
    TypedefBackedOne = 1,
};
