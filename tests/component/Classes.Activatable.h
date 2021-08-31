#pragma once
#include "Classes.Activatable.g.h"

namespace winrt::Component::Classes::implementation
{
    struct Activatable : ActivatableT<Activatable>
    {
        Activatable() = default;

        Activatable(int32_t value) :
            m_value(value)
        {
        }

        int32_t Property() const
        {
            return m_value;
        }

    private:

        int32_t m_value{};
    };
}
namespace winrt::Component::Classes::factory_implementation
{
    struct Activatable : ActivatableT<Activatable, implementation::Activatable>
    {
    };
}
