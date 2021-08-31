#pragma once
#include "Classes.Required.g.h"

namespace winrt::Component::Classes::implementation
{
    struct Required : RequiredT<Required>
    {
        Required() = default;

        int32_t Property() const
        {
            return m_value;
        }

        void Property(int32_t value)
        {
            m_value = value;
        }

    private:

        int32_t m_value{};
    };
}
namespace winrt::Component::Classes::factory_implementation
{
    struct Required : RequiredT<Required, implementation::Required>
    {
    };
}
