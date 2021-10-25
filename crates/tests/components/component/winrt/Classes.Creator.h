#pragma once
#include "Classes.Creator.g.h"

namespace winrt::Component::Classes::implementation
{
    struct Creator : CreatorT<Creator>
    {
        static winrt::Component::Classes::Creator Create(int32_t value)
        {
            return make<Creator>(value);
        }

        Creator(int32_t value) :
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
    struct Creator : CreatorT<Creator, implementation::Creator>
    {
    };
}
