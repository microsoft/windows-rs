#pragma once
#include "Composable.Base.g.h"

namespace winrt::Component::Composable::implementation
{
    struct Base : BaseT<Base>
    {
        Base() = default;

        Base(int32_t v) : m_value(v) {}

        int32_t Value()
        {
            return m_value;
        }

        void Value(int32_t value)
        {
            m_value = value;
        }

        hstring BaseRequired()
        {
            return L"BaseRequired";
        }

        int32_t m_value{};
    };
}
namespace winrt::Component::Composable::factory_implementation
{
    struct Base : BaseT<Base, implementation::Base>
    {
    };
}
