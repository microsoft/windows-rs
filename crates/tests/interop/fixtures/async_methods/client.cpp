#include <winrt/Windows.Foundation.h>
#include <winrt/AsyncMethods.h>
#include "check.h"

using namespace winrt;
using namespace Windows::Foundation;
using namespace AsyncMethods;

IAsyncAction action(bool fail = false)
{
    co_await resume_background();
    if (fail)
    {
        throw hresult_out_of_bounds();
    }
}

IAsyncActionWithProgress<uint32_t> action_with_progress(bool fail = false)
{
    co_await resume_background();
    if (fail)
    {
        throw hresult_out_of_bounds();
    }
    auto progress = co_await get_progress_token();
    progress(1);
}

IAsyncOperation<int32_t> operation()
{
    co_await resume_background();
    co_return 41;
}

IAsyncOperationWithProgress<int32_t, uint32_t> operation_with_progress()
{
    co_await resume_background();
    auto progress = co_await get_progress_token();
    progress(1);
    co_return 41;
}

template <typename T>
void check_bounds(T const& async)
{
    try
    {
        async.get();
        check(false);
    }
    catch (hresult_out_of_bounds const&)
    {
    }
}

void test()
{
    Widget widget;

    widget.Action(1).get();
    auto with_progress = widget.ActionWithProgress(1);
    with_progress.Progress([](auto&&, uint32_t) {});
    with_progress.get();
    check(widget.Operation(1).get() == 1);
    auto operation_progress = widget.OperationWithProgress(1);
    operation_progress.Progress([](auto&&, uint32_t) {});
    check(operation_progress.get() == 1);

    check_bounds(widget.Action(-1));
    check_bounds(widget.ActionWithProgress(-1));
    check_bounds(widget.Operation(-1));
    check_bounds(widget.OperationWithProgress(-1));

    widget.AwaitAction(action()).get();
    widget.AwaitActionWithProgress(action_with_progress()).get();
    check_bounds(widget.AwaitAction(action(true)));
    check_bounds(widget.AwaitActionWithProgress(action_with_progress(true)));
    check(widget.AwaitOperation(operation()).get() == 42);
    check(widget.AwaitOperationWithProgress(operation_with_progress()).get() == 42);
}
