#pragma once
#include "Async.Test.g.h"

namespace winrt::Component::Async::implementation
{
    struct Test
    {
        static winrt::Windows::Foundation::IAsyncAction CreateAsyncAction(Windows::Foundation::TimeSpan delay, bool fail)
        {
            co_await resume_after(delay);
            if (fail) throw hresult_invalid_argument(L"test");
        }
        static winrt::Windows::Foundation::IAsyncActionWithProgress<int32_t> CreateAsyncActionWithProgress(Windows::Foundation::TimeSpan delay, bool fail)
        {
            co_await resume_after(delay);
            if (fail) throw hresult_invalid_argument(L"test");
        }
        static winrt::Windows::Foundation::IAsyncOperation<int32_t> CreateAsyncOperation(Windows::Foundation::TimeSpan delay, bool fail, int32_t result)
        {
            co_await resume_after(delay);
            if (fail) throw hresult_invalid_argument(L"test");
            co_return result;
        }
        static winrt::Windows::Foundation::IAsyncOperationWithProgress<int32_t, int32_t> CreateAsyncOperationWithProgress(Windows::Foundation::TimeSpan delay, bool fail, int32_t result)
        {
            co_await resume_after(delay);
            if (fail) throw hresult_invalid_argument(L"test");
            co_return result;
        }
    };
}
namespace winrt::Component::Async::factory_implementation
{
    struct Test : TestT<Test, implementation::Test>
    {
    };
}
