enum E { EValue };
enum F { FValue };
namespace Dependency {
    enum E { OtherValue };
}

namespace Reopened {
    extern "C" IMPORT void First(ASSOC(E) int value);
}
namespace Reopened {
    using Dependency::E;
    extern "C" IMPORT void Good(ASSOC(F) int first, ASSOC(F) int second);
    extern "C" IMPORT void BadUsing(ASSOC(E) int value);
    extern "C" IMPORT void Absolute(ASSOC(::E) int value);
}

namespace Directed {
    using namespace Dependency;
    extern "C" IMPORT void BadDirective(ASSOC(E) int value);
    extern "C" IMPORT void AbsoluteDirective(ASSOC(::E) int value);
}

struct Shadowed {
    int F;
    ASSOC(E) int first;
    ASSOC(F) int second;
};

struct Clear {
    ASSOC(E) int first;
    ASSOC(F) int second;
};
