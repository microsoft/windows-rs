#pragma once
#include "Classes.NoWeakRef.g.h"

namespace winrt::Component::Classes::implementation
{
    struct NoWeakRef : NoWeakRefT<NoWeakRef, no_weak_ref>
    {
        NoWeakRef() = default;

        void Method()
        {
        }
    };
}
namespace winrt::Component::Classes::factory_implementation
{
    struct NoWeakRef : NoWeakRefT<NoWeakRef, implementation::NoWeakRef>
    {
    };
}
