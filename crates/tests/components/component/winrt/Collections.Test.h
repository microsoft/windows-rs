#pragma once
#include "Collections.Test.g.h"

namespace winrt::Component::Collections::implementation
{
    struct Test
    {
        static Windows::Foundation::Collections::IVector<int32_t> CreateInt32Vector()
        {
            return multi_threaded_vector<int32_t>();
        }

        static Windows::Foundation::Collections::IVector<hstring> CreateStringVector()
        {
            return multi_threaded_vector<hstring>();
        }

        static Windows::Foundation::Collections::IVector<Windows::Foundation::IStringable> CreateStringableVector()
        {
            return multi_threaded_vector<Windows::Foundation::IStringable>();
        }
    };
}
namespace winrt::Component::Collections::factory_implementation
{
    struct Test : TestT<Test, implementation::Test>
    {
    };
}
