#pragma once
#include "Classes.Static.g.h"

namespace winrt::Component::Classes::implementation
{
    struct Static
    {
    };
}
namespace winrt::Component::Classes::factory_implementation
{
    struct Static : StaticT<Static, implementation::Static, static_lifetime>
    {
        int32_t Method() const
        {
            return m_value;
        }

        int32_t Property() const
        {
            return m_value;
        }

        void Property(int32_t value)
        {
            m_value = value;
        }

        int32_t ReadOnly() const
        {
            return m_value;
        }

    private:

        int32_t m_value{};
    };
}
