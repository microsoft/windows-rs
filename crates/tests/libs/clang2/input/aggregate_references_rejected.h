struct Id {
    unsigned long a;
    unsigned short b, c;
    unsigned char d[8];
};
Id Mutable = {1, 2, 3, {4, 5, 6, 7, 8, 9, 10, 11}};
volatile const Id Volatile = {1, 2, 3, {4, 5, 6, 7, 8, 9, 10, 11}};
extern const Id Missing;
extern const Id CycleB;
extern const Id CycleA = CycleB;
extern const Id CycleB = CycleA;
#define MUTABLE_ALIAS Mutable
#define VOLATILE_ALIAS Volatile
#define MISSING_ALIAS Missing
#define CYCLIC_ALIAS CycleA
struct MutableMember {
    mutable unsigned long value;
};
struct VolatileMember {
    volatile unsigned long value;
};
struct NestedMutableMember {
    MutableMember value[1];
};
extern const MutableMember MutableMemberSource = {1};
extern const MutableMember MutableMemberCopy = MutableMemberSource;
extern const VolatileMember VolatileMemberSource = {1};
extern const NestedMutableMember NestedMutableMemberSource = {{{1}}};
#define MUTABLE_MEMBER_ALIAS MutableMemberSource
#define MUTABLE_MEMBER_COPY_ALIAS MutableMemberCopy
#define VOLATILE_MEMBER_ALIAS VolatileMemberSource
#define NESTED_MUTABLE_MEMBER_ALIAS NestedMutableMemberSource
constexpr Id Partial = {1, 2, 3, {4}};
#define PARTIAL_ALIAS Partial
#define PARTIAL_LITERAL Id{1, 2, 3, {4}}

Id RuntimeId();
constexpr Id ContextualId() {
    return {__builtin_is_constant_evaluated() ? 1UL : 2UL, 2, 3, {4, 5, 6, 7, 8, 9, 10, 11}};
}
#define RUNTIME_ALIAS RuntimeId()
#define CONTEXTUAL_ALIAS ContextualId()

struct Custom {
    unsigned long a;
    unsigned short b, c;
    unsigned char d[8];
    Custom(unsigned long value) : a(value), b(2), c(3), d{} {}
    Custom(const Custom& other) : a(other.a + 1), b(other.b), c(other.c), d{} {}
};
extern const Custom CustomSource = Custom(1);
extern const Custom CustomCopy = CustomSource;
#define CUSTOM_ALIAS CustomSource
#define CUSTOM_COPY_ALIAS CustomCopy
