#define SUCCESS(expr) __attribute__((annotate("windows_clang:sal:2:_Success_\n(" #expr ")\n(" #expr ")")))
#define DEFAULT(expr) __attribute__((annotate("windows_clang:sal:2:_Return_type_success_\n(" #expr ")\n(" #expr ")")))
#define LEGACY(expr) __attribute__((annotate("windows_clang:sal:1.1:__success\n(" #expr ")\n(" #expr ")")))
#define POST __attribute__((annotate("windows_clang:sal:2:_Post_\n()\n()")))
#define DIFFERENT __attribute__((annotate("windows_clang:sal:2:_Success_\n(return == ZERO)\n(return == 0)")))

typedef DEFAULT(return >= 0) long Status, *PStatus;
typedef Status CopyStatus;
typedef Status* PointerCopy;
typedef DEFAULT(return == 0) Status CompetingStatus;
typedef DEFAULT(return == 0) unsigned long ZeroStatus;
typedef DEFAULT(return >= 0) unsigned long UnsignedStatus;
typedef DEFAULT(return == 0) float FloatStatus;
enum Code { Bad = -1, Good = 0 };
enum class ScopedCode : int { Bad = -1, Good = 0 };
typedef DEFAULT(return >= 0) enum NamedCode { NamedBad = -1, NamedGood = 0 } NamedStatus;

extern "C" {
    SUCCESS(return >= 0) int SignedResult();
    SUCCESS(return >= 0) unsigned int UnsignedResult();
    SUCCESS(return == 1) int ExactOne();
    SUCCESS(return != 0) int Truth();
    SUCCESS(return) bool BoolTruth();
    SUCCESS(return != false) int FalseTruth();
    SUCCESS(0 != return) void* NonNull();
    SUCCESS(return == 0) void* NullSuccess();
    SUCCESS(return > 0) int Positive();
    SUCCESS(return >= 0) Code EnumResult();
    SUCCESS(return >= 0) ScopedCode ScopedEnumResult();
    SUCCESS(return == 0) Status Override();
    CopyStatus InheritedDefault();
    CompetingStatus UnresolvedDefault();
    LEGACY(return == 0L) unsigned long LegacyZero();
    SUCCESS(return == 0) SUCCESS(return == 0) int Duplicate();
    SUCCESS(return == 0) SUCCESS(return != 0) int Conflicting();
    SUCCESS(1) int Always();
    SUCCESS(return != 0 && return < capacity) int Bounded(int capacity);
    SUCCESS(return == 0 || return == 234L) int Partial();
    SUCCESS(return >= 0) void* InvalidPointer();
    SUCCESS(return == 1) void* OnePointer();
    SUCCESS(return != 0) float Floating();
    POST SUCCESS(return == 0) int Scoped();
    DIFFERENT int Expanded();
    void Parameter(SUCCESS(return == 0) int* value);
}

typedef SUCCESS(return == 0) int (__stdcall *Callback)();
typedef Callback CallbackAlias;
typedef SUCCESS(return != 0) Callback ConflictingCallback;

struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IStatus {
    virtual SUCCESS(return == 0) Status __stdcall Read() = 0;
    virtual SUCCESS(return == 1) int __stdcall Exact() = 0;
};
