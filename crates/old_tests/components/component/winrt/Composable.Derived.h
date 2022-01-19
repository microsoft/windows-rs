#pragma once
#include "Composable.Derived.g.h"
#include "Composable.Base.h"

namespace winrt::Component::Composable::implementation
{
    struct Derived : DerivedT<Derived, Component::Composable::implementation::Base>
    {
        Derived() = default;

        Derived(int32_t v)
        {
            m_value = v;
        }

        hstring DerivedRequired()
        {
            return L"DerivedRequired";
        }
    };
}
namespace winrt::Component::Composable::factory_implementation
{
    struct Derived : DerivedT<Derived, implementation::Derived>
    {
    };
}
