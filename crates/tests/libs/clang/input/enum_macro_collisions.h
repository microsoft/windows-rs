enum Pool {
    PoolDefault = 0,
    PoolAligned = 4,
    PoolNx = 512,
    PoolNxAligned = 516,
    MatchingMember = 7,
    FailedMember = 9
};

enum class Scoped { ScopedMember = 11 };
namespace Other {
    enum Named { NamespacedMember = 13 };
}

struct PoolConsumer {
    Pool value;
};

#if defined(_M_ARM64)
#define PoolDefault PoolNx
#define PoolAligned PoolNxAligned
#endif
#define MatchingMember 7
#define FailedMember missing_identifier
#define ScopedMember 12
#define NamespacedMember 14
