/***
* ==++==
*
* Copyright (c) Microsoft Corporation.  All rights reserved.
*
* ==--==
* =+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+=+
*
* amprt_exceptions.h
*
* Define the C++ interfaces exported by the C++ AMP runtime
*
* =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
****/
#pragma once

#ifndef _SILENCE_AMP_DEPRECATION_WARNINGS
#error <amprt_exceptions.h> is part of C++ AMP which is deprecated by Microsoft and will be REMOVED. \
You can define _SILENCE_AMP_DEPRECATION_WARNINGS to acknowledge that you have received this warning.
#endif // _SILENCE_AMP_DEPRECATION_WARNINGS

#include <exception>

#if !defined(_AMPIMP)
#define _AMPIMP     __declspec(dllimport)
#endif

// exceptions
namespace Concurrency {
/// <summary>
///     Exception thrown due to a C++ AMP runtime_exception.
///     This is the base type for all C++ AMP exception types.
/// </summary>
class runtime_exception : public std::exception
{
public:
    /// <summary>
    ///     Construct a runtime_exception exception with a message and an error code
    /// </summary>
    /// <param name="_Message">
    ///     Descriptive message of error
    /// </param>
    /// <param name="_Hresult">
    ///     HRESULT of error that caused this exception
    /// </param>
    _AMPIMP runtime_exception(const char * _Message, HRESULT _Hresult) throw();

    /// <summary>
    ///     Construct a runtime_exception exception with an error code
    /// </summary>
    /// <param name="_Hresult">
    ///     HRESULT of error that caused this exception
    /// </param>
    _AMPIMP explicit runtime_exception(HRESULT _Hresult) throw();

    /// <summary>
    ///     Copy construct a runtime_exception exception
    /// </summary>
    /// <param name="_Other">
    ///     The runtime_exception object to be copied from
    /// </param>
    _AMPIMP runtime_exception(const runtime_exception &_Other) throw();

    /// <summary>
    ///     Assignment operator
    /// </summary>
    /// <param name="_Other">
    ///     The runtime_exception object to be assigned from
    /// </param>
    _AMPIMP runtime_exception &operator=(const runtime_exception &_Other) throw();

    /// <summary>
    ///     Destruct a runtime_exception exception object instance
    /// </summary>
    _AMPIMP virtual ~runtime_exception() noexcept;

    /// <summary>
    ///     Get the error code that caused this exception
    /// </summary>
    /// <returns>
    ///     HRESULT of error that caused the exception
    /// </returns>
    _AMPIMP HRESULT get_error_code() const throw();

private:
    HRESULT _M_error_code;
}; // class runtime_exception

/// <summary>
///     Exception thrown when an underlying OS/DirectX call fails
///     due to lack of system or device memory
/// </summary>
class out_of_memory : public runtime_exception
{
public:
    /// <summary>
    ///     Construct an out_of_memory exception with a message
    /// </summary>
    /// <param name="_Message">
    ///     Descriptive message of error
    /// </param>
    _AMPIMP explicit out_of_memory(const char * _Message) throw();

    /// <summary>
    ///     Construct an out_of_memory exception
    /// </summary>
    _AMPIMP out_of_memory () throw();
}; // class out_of_memory

/// <summary>
/// Exception thrown when an underlying DirectX call fails
/// due to the Windows timeout detection and recovery mechanism
/// </summary>
class accelerator_view_removed : public runtime_exception
{
public:
    /// <summary>
    ///     Construct an accelerator_view_removed exception with a message and
    ///     a view removed reason code
    /// </summary>
    /// <param name="_Message">
    ///     Descriptive message of error
    /// </param>
    /// <param name="_View_removed_reason">
    ///     HRESULT error code indicating the cause of removal of the accelerator_view
    /// </param>
    _AMPIMP explicit accelerator_view_removed(const char * _Message, HRESULT _View_removed_reason) throw();

    /// <summary>
    ///     Construct an accelerator_view_removed exception
    /// </summary>
    /// <param name="_View_removed_reason">
    ///     HRESULT error code indicating the cause of removal of the accelerator_view
    /// </param>
    _AMPIMP explicit accelerator_view_removed(HRESULT _View_removed_reason) throw();

    /// <summary>
    ///     Returns an HRESULT error code indicating the cause of the accelerator_view's removal
    /// </summary>
    /// <returns>
    ///     The HRESULT error code that indicates the cause of accelerator_view's removal
    /// </returns>
    _AMPIMP HRESULT get_view_removed_reason() const throw();

private:

    HRESULT _M_view_removed_reason_code;
}; // class accelerator_view_removed

/// <summary>
///     Exception thrown when the runtime fails to launch a kernel
///     using the compute domain specified at the parallel_for_each call site.
/// </summary>
class invalid_compute_domain : public runtime_exception
{
public:
    /// <summary>
    ///     Construct an invalid_compute_domain exception with a message
    /// </summary>
    /// <param name="_Message">
    ///     Descriptive message of error
    /// </param>
    _AMPIMP explicit invalid_compute_domain(const char * _Message) throw();

    /// <summary>
    ///     Construct an invalid_compute_domain exception
    /// </summary>
    _AMPIMP invalid_compute_domain() throw();
}; // class invalid_compute_domain

/// <summary>
///     Exception thrown when an unsupported feature is used
/// </summary>
class unsupported_feature  : public runtime_exception
{
public:
    /// <summary>
    ///     Construct an unsupported_feature exception with a message
    /// </summary>
    /// <param name="_Message">
    ///     Descriptive message of error
    /// </param>
    _AMPIMP explicit unsupported_feature(const char * _Message) throw();

    /// <summary>
    ///     Construct an unsupported_feature exception
    /// </summary>
    _AMPIMP unsupported_feature() throw();
}; // class unsupported_feature
}
