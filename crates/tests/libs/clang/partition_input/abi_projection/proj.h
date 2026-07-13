// Out-of-scope WinRT C++ projection header analogue (the `winrt/asyncinfo.h` case). It
// declares a type inside the `ABI::Windows::*` projection namespace, re-exports it into
// global scope with `using`, then a global-scope declaration references it through that
// alias. The global declaration is out of scope and unreferenced by any in-scope API, so
// the reachability sweep drops it — but emission still resolves its ABI-projected field
// type first. That type must map to the corresponding `Windows.winmd` reference
// (`Windows.Foundation.ProjStatus`) instead of failing on the unexposed
// `ABI::Windows::Foundation::ProjStatus` spelling.
namespace ABI {
namespace Windows {
namespace Foundation {
enum class ProjStatus { Started };
}
}
}
using ABI::Windows::Foundation::ProjStatus;

struct ProjThing {
    ProjStatus status;
};
